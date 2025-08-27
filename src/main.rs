// src/main.rs - Fixed version with correct Prisma schema mapping
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use std::collections::HashMap;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use std::process::{Command, Stdio};
use std::io::Write;
// Use Builder to create a tempfile with an extension
use tempfile::{NamedTempFile, Builder};
use tokio::time::{timeout, Duration};

mod lexer;
mod parser;
mod evaluator;
mod object;

#[derive(Debug, sqlx::Type, Clone)]
#[sqlx(type_name = "ExecutionStatus", rename_all = "SCREAMING_SNAKE_CASE")]
enum ExecutionStatus {
    Pending,
    Success,
    Error,
    Timeout,
    MemoryLimit,
}

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
    language: String, // "rust", "python", "c", or "custom"
}

#[derive(Serialize)]
struct CompileResponse {
    result: Option<String>,
    error: Option<String>,
    execution_time_ms: Option<u64>,
}

// Enhanced security for code execution
const EXECUTION_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_OUTPUT_SIZE: usize = 10_000; 

async fn compile_handler(req: web::Json<CompileRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let start_time = std::time::Instant::now();
    let code = &req.code;
    let language = &req.language.to_lowercase();

    if code.len() > 50_000 {
        return HttpResponse::BadRequest().json(CompileResponse {
            result: None,
            error: Some("Code too large (max 50KB)".to_string()),
            execution_time_ms: Some(start_time.elapsed().as_millis() as u64),
        });
    }

    let result = match language.as_str() {
        "custom" => execute_custom_language(code).await,
        "rust" => execute_rust_code(code).await,
        "python" => execute_python_code(code).await,
        "c" => execute_c_code(code).await,
        _ => Err("Unsupported language. Use: custom, rust, python, or c".to_string()),
    };

    let execution_time = start_time.elapsed().as_millis() as u64;

    let response = match result {
        Ok(output) => {
            // Log success to database - using Prisma column names
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, result, status, execution_time_ms, language) VALUES ($1, $2, $3, $4, $5)"#,
                code,
                Some(output.clone()),
                ExecutionStatus::Success as _,
                execution_time as i32,
                language
            )
            .execute(pool.get_ref())
            .await;

            CompileResponse {
                result: Some(output),
                error: None,
                execution_time_ms: Some(execution_time),
            }
        }
        Err(error) => {
            // Log error to database - using Prisma column names
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, error, status, execution_time_ms, language) VALUES ($1, $2, $3, $4, $5)"#,
                code,
                Some(error.clone()),
                ExecutionStatus::Error as _,
                execution_time as i32,
                language
            )
            .execute(pool.get_ref())
            .await;

            CompileResponse {
                result: None,
                error: Some(error),
                execution_time_ms: Some(execution_time),
            }
        }
    };

    HttpResponse::Ok().json(response)
}

// Execute custom language (your interpreter)
async fn execute_custom_language(code: &str) -> Result<String, String> {
    let tokens = lexer::tokenize(code)?;
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse_program()?;
    let mut env = HashMap::new();
    
    // Capture stdout for print statements
    let result = evaluator::evaluate(&ast, &mut env)?;
    
    // Combine any print output with final result
    let result_str = if result.to_string() == "null" {
        String::new()
    } else {
        result.to_string()
    };
    
    Ok(result_str)
}

// Execute Rust code
async fn execute_rust_code(code: &str) -> Result<String, String> {
    // Create temporary file for Rust code
    let mut temp_file = Builder::new()
        .prefix("user_code_")
        .suffix(".rs")
        .tempfile()
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    // Wrap code in main function if not present
    let wrapped_code = if !code.contains("fn main") {
        format!("fn main() {{\n{}\n}}", code)
    } else {
        code.to_string()
    };
    
    write!(temp_file, "{}", wrapped_code)
        .map_err(|e| format!("Failed to write code: {}", e))?;
    
    let temp_path = temp_file.path();
    let exe_path = temp_path.with_extension("exe");
    
    // Compile Rust code
    let compile_output = timeout(EXECUTION_TIMEOUT, async {
        Command::new("rustc")
            .arg(temp_path)
            .arg("-o")
            .arg(&exe_path)
            .arg("--crate-name") 
            .arg("user_code")   
            .arg("--edition=2021")
            .arg("-A")
            .arg("warnings")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|_| "Compilation timeout".to_string())?
    .map_err(|e| format!("Compilation failed: {}", e))?;
    
    if !compile_output.status.success() {
        return Err(format!("Compilation error: {}", 
            String::from_utf8_lossy(&compile_output.stderr)));
    }
    
    // Execute compiled binary
    let run_output = timeout(EXECUTION_TIMEOUT, async {
        Command::new(&exe_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|_| "Execution timeout".to_string())?
    .map_err(|e| format!("Execution failed: {}", e))?;
    
    // Clean up
    let _ = std::fs::remove_file(&exe_path);
    
    if !run_output.status.success() {
        return Err(format!("Runtime error: {}", 
            String::from_utf8_lossy(&run_output.stderr)));
    }
    
    let output = String::from_utf8_lossy(&run_output.stdout);
    if output.len() > MAX_OUTPUT_SIZE {
        return Err("Output too large".to_string());
    }
    
    Ok(output.to_string())
}

// Execute Python code
async fn execute_python_code(code: &str) -> Result<String, String> {
    let output = timeout(EXECUTION_TIMEOUT, async {
        // --- FIX: Changed "python3" to "python" for Windows compatibility ---
        Command::new("python")
            .arg("-c")
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|_| "Execution timeout".to_string())?
    .map_err(|e| format!("Failed to execute Python: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Python error: {}", 
            String::from_utf8_lossy(&output.stderr)));
    }
    
    let result = String::from_utf8_lossy(&output.stdout);
    if result.len() > MAX_OUTPUT_SIZE {
        return Err("Output too large".to_string());
    }
    
    Ok(result.to_string())
}

// Execute C code
async fn execute_c_code(code: &str) -> Result<String, String> {
    // --- FIX: Create a temp file that ends with ".c" ---
    let mut temp_file = Builder::new()
        .prefix("user_code_")
        .suffix(".c")
        .tempfile()
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    write!(temp_file, "{}", code)
        .map_err(|e| format!("Failed to write code: {}", e))?;
    
    let temp_path = temp_file.path();
    let exe_path = temp_path.with_extension("exe");
    
    // Compile C code
    let compile_output = timeout(EXECUTION_TIMEOUT, async {
        Command::new("gcc")
            .arg(temp_path)
            .arg("-o")
            .arg(&exe_path)
            .arg("-std=c99")
            .arg("-Wall")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|_| "Compilation timeout".to_string())?
    .map_err(|e| format!("Compilation failed: {}", e))?;
    
    if !compile_output.status.success() {
        return Err(format!("Compilation error: {}", 
            String::from_utf8_lossy(&compile_output.stderr)));
    }
    
    // Execute compiled binary
    let run_output = timeout(EXECUTION_TIMEOUT, async {
        Command::new(&exe_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|_| "Execution timeout".to_string())?
    .map_err(|e| format!("Execution failed: {}", e))?;
    
    // Clean up
    let _ = std::fs::remove_file(&exe_path);
    
    if !run_output.status.success() {
        return Err(format!("Runtime error: {}", 
            String::from_utf8_lossy(&run_output.stderr)));
    }
    
    let output = String::from_utf8_lossy(&run_output.stdout);
    if output.len() > MAX_OUTPUT_SIZE {
        return Err("Output too large".to_string());
    }
    
    Ok(output.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create database pool");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    println!("Multi-language compiler server starting on http://0.0.0.0:{}", port);
    println!("Supported languages: custom, rust, python, c");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);
            
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().limit(1024 * 1024)) 
            .route("/compile", web::post().to(compile_handler))
            .route("/health", web::get().to(|| async { HttpResponse::Ok().json("OK") }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}