#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use std::net::SocketAddr;

use axum::{
  http::{self, HeaderValue, Method},
  response::{Html, IntoResponse},
  routing::get,
  Json, Router,
};
use error::MyError;
use tower_http::cors::CorsLayer;
use tracing::info;

mod error;
#[cfg(test)] mod tests;
mod utils;

#[tokio::main]
async fn main() -> Result<(), MyError> {
  utils::setup()?;
  tokio::try_join!(frontend(), backend())?;
  Ok(())
}

async fn frontend() -> Result<(), MyError> {
  let app = Router::new().route("/", get(html));
  serve(app, 3000).await;
  Ok(())
}

fn cors_layer() -> CorsLayer {
  CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_headers([http::header::CONTENT_TYPE])
    .allow_methods([Method::GET, Method::POST])
}

async fn backend() -> Result<(), MyError> {
  let router =
    axum::Router::new().layer(cors_layer()).route("/", get(|| async { "Hello, World!" }));
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  info!("listening on {}", listener.local_addr()?);
  serve(router, 4000).await;
  Ok(())
}

async fn serve(app: Router, port: u16) {
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn html() -> impl IntoResponse {
  axum::response::Html(
    r#"
        <script>
            fetch('http://localhost:4000/json')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
  )
}

async fn json() -> impl IntoResponse { axum::Json(vec!["one", "two", "three"]) }