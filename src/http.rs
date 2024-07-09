use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Redirect},
    routing::get,
    Router,
};
use log::debug;

#[derive(Clone)]
struct StaticDir(PathBuf);

pub struct HttpServer {
    host: String,
    port: u16,
    root_dir: PathBuf,
}

impl HttpServer {
    pub fn new(host: String, port: u16, root_dir: PathBuf) -> Self {
        HttpServer { host, port, root_dir }
    }

    pub fn run(&self) {
        let host = self.host.clone();
        let port = self.port;
        let root_dir = self.root_dir.clone();

        let (server_tx, server_rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let app = Router::new()
                .route("/", get(Redirect::permanent("/index.html")))
                .route("/*path", get(Self::handle_path))
                .with_state(StaticDir(root_dir));
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();
                axum::serve(listener, app).await.unwrap();
                server_tx.send(()).unwrap();
            });
        });
        _ = server_rx.recv().unwrap();
    }

    async fn handle_path(Path(path): Path<String>, State(static_dir): State<StaticDir>) -> Response {
        debug!("...{}", path);
        let mut path = static_dir.0.join(path);
        match tokio::fs::metadata(&path).await {
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return (StatusCode::NOT_FOUND, "not found").into_response();
                } else {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "unhandled type").into_response();
                }
            }
            Ok(metadata) => {
                if metadata.is_dir() {
                    path.push("index.html");
                }
                if metadata.is_file() || metadata.is_symlink() {
                    let guess = mime_guess::from_path(&path).first();
                    let mime_type = guess.unwrap_or(mime_guess::mime::APPLICATION_OCTET_STREAM);
                    let bytes = tokio::fs::read(&path).await.unwrap();
                    return Response::builder()
                        .header(axum::http::header::CONTENT_TYPE, mime_type.to_string())
                        .status(StatusCode::OK)
                        .body(axum::body::Body::from(bytes))
                        .unwrap();
                } else {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "unhandled type").into_response();
                }
            }
        }
    }
}
