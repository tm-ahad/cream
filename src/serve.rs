use std::path::PathBuf;
use std::process::exit;

use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;
use crate::{config::Config, std_err::{ErrType::{HttpServingError, NotFound}, StdErr}};
use crate::std_err::ErrType::SyntaxError;

async fn index() -> Html<String> {
    Html(
         tokio::fs::read_to_string("./build/index.html")
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
                .not_found_service(get(index))
        )
}

pub async fn serve(conf: &Config) {
    let router = create_router();
    let root = PathBuf::from(&conf.root);
    let root = root
        .to_str()
        .unwrap_or_else(|| {
            StdErr::exec(SyntaxError, "Invalid root in config.toml:");
            ""
        });

    let root_path = &format!("localhost:{}", conf.port);
    let listener =
        tokio::net::TcpListener::bind(root_path)
        .await
        .unwrap_or_else(|err| {
            StdErr::exec(HttpServingError, &format!("{err}"));
            exit(1)
        });

    let app = if root == "/" {
        router
    } else {
        Router::new()
            .nest(&root, router)
    };

    println!("Serving at {}{}", root_path, root);
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| {
            StdErr::exec(
                HttpServingError,
                &format!("{err}")
            );
            exit(1)
        });
}

