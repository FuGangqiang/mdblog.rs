use std::io;
use std::fs::File;
use std::thread;
use std::path::{Path, PathBuf};
use mime_guess::get_mime_type;
use percent_encoding::percent_decode;
use futures::sync::oneshot;
use futures::future::{self, Future};
use hyper::{self, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Request, Response, Service};

static NOTFOUND: &[u8] = b"Not Found";

/// Http Service for static file server
pub struct HttpService {
    pub root: PathBuf,
}


impl Service for HttpService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let uri_path = percent_decode(req.uri().path().as_bytes()).decode_utf8().unwrap();
        match local_path_for_request(&self.root, &uri_path) {
            None => {
                warn!("Not Found {}", uri_path);
                Box::new(future::ok(not_found_response()))
            },
            Some(path) => {
                info!("{}", uri_path);
                let (tx, rx) = oneshot::channel();
                thread::spawn(move || {
                    let mut file = match File::open(&path) {
                        Ok(f) => f,
                        Err(_) => {
                            error!("open file error: {}", &path.display());
                            tx.send(not_found_response()).expect("Send error on open");
                            return;
                        },
                    };
                    let mut buf: Vec<u8> = Vec::new();
                    match io::copy(&mut file, &mut buf) {
                        Ok(_) => {
                            let mime = get_mime_type(&path.extension()
                                                          .expect("get file extention error")
                                                          .to_str()
                                                          .expect("get str slice error"));
                            let res = Response::new()
                                .with_header(ContentLength(buf.len() as u64))
                                .with_header(ContentType(mime))
                                .with_body(buf);
                            tx.send(res)
                              .expect("Send error on successful file read");
                        },
                        Err(_) => {
                            error!("read file error: {}", &path.display());
                            tx.send(Response::new().with_status(StatusCode::InternalServerError))
                              .expect("Send error on error reading file");
                        },
                    };
                });
                Box::new(rx.map_err(|e| io::Error::new(io::ErrorKind::Other, e).into()))
            }
        }
    }
}

fn not_found_response() -> Response {
    Response::new()
            .with_status(StatusCode::NotFound)
            .with_header(ContentLength(NOTFOUND.len() as u64))
            .with_body(NOTFOUND)
}


fn local_path_for_request<P: AsRef<Path>>(root_dir: P, request_path: &str) -> Option<PathBuf> {
    if !request_path.starts_with("/") {
        return None;
    }
    // skip query string
    let end = request_path.find('?').unwrap_or(request_path.len());
    let request_path = &request_path[0..end];

    let mut path = root_dir.as_ref().to_owned();
    // request path start with "/"
    path.push(&request_path[1..]);
    if request_path == "/" {
        path.push("index.html");
    }

    Some(path)
}
