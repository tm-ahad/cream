use std::path::PathBuf;
use std::process::exit;

use crate::std_err::ErrType::SyntaxError;
use crate::{
    config::Config,
    std_err::{
        ErrType::{HttpServingError, NotFound},
        StdErr,
    },
};
use axum::http::HeaderValue;
use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

async fn index() -> Html<String> {
    Html(
        tokio::fs::read_to_string("./build/index.html")
            .await
            .unwrap_or_else(|_| {
                StdErr::exec(NotFound, "build/index.html");
                exit(1)
            }),
    )
}

pub fn create_router(root: &str) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            format!("http://localhost:{root}")
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    Router::new()
        .fallback_service(ServeDir::new("./build").not_found_service(get(index)))
        .layer(cors)
}

pub async fn serve(conf: &Config) {
    let router = create_router(&conf.root);
    let root = PathBuf::from(&conf.root);
    let root = root.to_str().unwrap_or_else(|| {
        StdErr::exec(SyntaxError, "Invalid root in config.toml:");
        ""
    });

    let root_path = &format!("localhost:{}", conf.port);
    let listener = tokio::net::TcpListener::bind(root_path)
        .await
        .unwrap_or_else(|err| {
            StdErr::exec(HttpServingError, &format!("{err}"));
            exit(1)
        });

    let app = if root == "/" {
        router
    } else {
        if root == "/" || root == "./" {
            Router::new().merge(router)
        } else {
            Router::new().nest(root, router)
        }
    };

    println!(
        "Serving at {}{}",
        root_path,
        if root.starts_with('.') {
            &root[1..]
        } else {
            root
        }
    );
    axum::serve(listener, app).await.unwrap_or_else(|err| {
        StdErr::exec(HttpServingError, &format!("{err}"));
        exit(1)
    });
}
