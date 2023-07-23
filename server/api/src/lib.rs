use std::{
    borrow::Cow,
    env,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use anyhow::Result;
use axum::{
    body::{boxed, Body},
    error_handling::HandleErrorLayer,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    BoxError, Router,
};
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::websocket::ws_handler;

mod display;
mod error;
mod websocket;

#[derive(Clone)]
pub struct AppState {}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

#[tokio::main]
async fn start() -> Result<()> {
    let static_dir = env::var("STATIC_DIR").unwrap_or("./dist".to_string());
    let addr = env::var("ADDR").unwrap_or("::1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    let log_level = env::var("LOG_LEVEL").unwrap_or("debug".to_string());
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info,tower_http=info", log_level),
        )
    }

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .fallback_service(get(|req| async move {
            match ServeDir::new(&static_dir).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };
                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10)),
        )
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .route("/api/ws", get(ws_handler))
        .nest("/api/display", display::router());

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to start server");

    Ok(())
}

pub fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("error: {err}");
    }
}
