use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tera") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".tera"]);
        tera
    };
}

async fn index() -> impl IntoResponse {
    TEMPLATES
        .render("index.tera", &tera::Context::new())
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn shutdown(
    Extension(tx): Extension<tokio::sync::mpsc::Sender<()>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match headers.get("Secret-Key") {
        Some(key) => match key.to_str() {
            Ok(key) => {
                if key != dotenv!("SHUTDOWN_KEY") {
                    return Err((StatusCode::BAD_REQUEST, "Invalid key".to_string()));
                }
            }
            Err(e) => {
                return Err((StatusCode::BAD_REQUEST, e.to_string()));
            }
        },
        None => {
            return Ok(StatusCode::UNAUTHORIZED);
        }
    }
    tx.send(())
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub fn router(shutdown_tx: tokio::sync::mpsc::Sender<()>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/shutdown", post(shutdown))
        .layer(Extension(shutdown_tx))
}
