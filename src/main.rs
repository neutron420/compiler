use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use std::collections::HashMap;

mod lexer;
mod parser;
mod evaluator;
mod object;

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Serialize)]
struct CompileResponse {
    result: Option<String>,
    error: Option<String>,
}

async fn compile_handler(req: web::Json<CompileRequest>) -> impl Responder {
    let code = &req.code;

    let tokens_result = lexer::tokenize(code);
    let tokens = match tokens_result {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::Ok().json(CompileResponse {
                result: None,
                error: Some(format!("Lexer Error: {}", e)),
            });
        }
    };

    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(a) => a,
        Err(e) => {
            return HttpResponse::Ok().json(CompileResponse {
                result: None,
                error: Some(format!("Parser Error: {}", e)),
            });
        }
    };
    
    let mut env = HashMap::new();
    match evaluator::evaluate(&ast, &mut env) {
        Ok(value) => HttpResponse::Ok().json(CompileResponse {
            result: Some(value.to_string()),
            error: None,
        }),
        Err(e) => HttpResponse::Ok().json(CompileResponse {
            result: None,
            error: Some(format!("Evaluation Error: {}", e)),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(" Advanced compiler server starting on http://127.0.0.1:8080");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/compile", web::post().to(compile_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}