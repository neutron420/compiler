use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use std::collections::HashMap;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

mod lexer;
mod parser;
mod evaluator;
mod object;

// This enum maps the Rust code to the "ExecutionStatus" enum in your database schema.
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "ExecutionStatus", rename_all = "UPPERCASE")]
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
}

#[derive(Serialize)]
struct CompileResponse {
    result: Option<String>,
    error: Option<String>,
}

async fn compile_handler(req: web::Json<CompileRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let code = &req.code;

    let tokens = match lexer::tokenize(code) {
        Ok(t) => t,
        Err(e) => {
            let error_message = format!("Lexer Error: {}", e);
            // Log the error to the database
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, error, status) VALUES ($1, $2, $3)"#,
                code,
                Some(error_message.clone()),
                ExecutionStatus::Error as _
            )
            .execute(pool.get_ref())
            .await;
            return HttpResponse::Ok().json(CompileResponse { result: None, error: Some(error_message) });
        }
    };

    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(a) => a,
        Err(e) => {
            let error_message = format!("Parser Error: {}", e);
            // Log the error to the database
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, error, status) VALUES ($1, $2, $3)"#,
                code,
                error_message,
                ExecutionStatus::Error as _
            )
            .execute(pool.get_ref())
            .await;
            return HttpResponse::Ok().json(CompileResponse { result: None, error: Some(error_message) });
        }
    };
    
    let mut env = HashMap::new();
    match evaluator::evaluate(&ast, &mut env) {
        Ok(value) => {
            let result_str = value.to_string();
            // Log the success to the database
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, result, status) VALUES ($1, $2, $3)"#,
                code,
                Some(result_str.clone()),
                ExecutionStatus::Success as _
            )
            .execute(pool.get_ref())
            .await;

            HttpResponse::Ok().json(CompileResponse { result: Some(result_str), error: None })
        },
        Err(e) => {
            let error_message = format!("Evaluation Error: {}", e);
            // Log the error to the database
            let _ = sqlx::query!(
                r#"INSERT INTO executions (code, error, status) VALUES ($1, $2, $3)"#,
                code,
                Some(error_message.clone()),
                ExecutionStatus::Error as _
            )
            .execute(pool.get_ref())
            .await;

            HttpResponse::Ok().json(CompileResponse { result: None, error: Some(error_message) })
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");

    println!("✅ Advanced compiler server starting on http://127.0.0.1:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .route("/compile", web::post().to(compile_handler))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}