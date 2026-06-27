use std::process::exit;

use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;
use crate::{config::Config, std_err::{ErrType::{HttpServingError, NotFound}, StdErr}};

async fn index() -> Html<String> {
    Html(
         tokio::fs::read_to_string(
            "./build/index.html"
        )
        .await
        .unwrap_or_else(|_| {
            StdErr::exec(NotFound, "build/index.html");
            exit(1)
        })
    )
}

pub fn create_router() -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new("./build")
                .not_found_service(
                    get(index)
                )
        )
}

pub async fn serve(conf: &Config) {
    let router = create_router();
    let listener =
        tokio::net::TcpListener::bind(
            format!("localhost:{}", conf.port)
        )
        .await
        .unwrap_or_else(|err| {
            StdErr::exec(HttpServingError, &format!("{err}"));
            exit(1)
        });

    println!("Serving at http://localhost:{}", conf.port);
    axum::serve(listener, router)
        .await
        .unwrap_or_else(|err| {
            StdErr::exec(
                HttpServingError,
                &format!("{err}")
            );
            exit(1)
        });
}

