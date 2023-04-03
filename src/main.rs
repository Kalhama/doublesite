use std::net::SocketAddr;
use std::path::Path;

use bytes::Bytes;
use har::v1_2::{Entries, Headers, Log};
use http_body_util::{BodyExt, Empty, Full};
use hyper::http::response;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Uri;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

// static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
// static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
// static NOTFOUND: &[u8] = b"Not Found";
// static POST_DATA: &str = r#"{"original": "data"}"#;
// static URL: &str = "http://127.0.0.1:8080/test.html";

fn get_har_log() -> har::v1_2::Log {
    use std::time::Instant;
    let now = Instant::now();

    // TODO this function is painfully slow and its called many times ;(
    let log = match har::from_path("github.har") {
        Ok(spec) => match spec.log {
            har::Spec::V1_2(log) => log,
            har::Spec::V1_3(_) => panic!("unsupported type"), // TODO support for also v1.3
        },
        _ => panic!("no file"),
    };
    let elapsed = now.elapsed();
    println!("get_har_log took: {:.2?}", elapsed.as_millis());
    log
}

fn match_method(req: &Request<IncomingBody>, entry: &Entries) -> bool {
    Method::from_bytes(entry.request.method.as_bytes()).unwrap() == req.method()
}

fn match_url(req: &Request<IncomingBody>, entry: &Entries) -> bool {
    use std::time::Instant;
    let now = Instant::now();

    let request_url = uri_to_har_url(req.uri());
    let har_uri = entry.request.url.as_str();
    let is_match = har_uri.contains(&request_url);

    // println!(
    //     "request url: {} har_uri: {} is_match: {}",
    //     request_url, har_uri, is_match
    // );
    let elapsed = now.elapsed();
    println!("match_url took: {:.2?}", elapsed.as_millis());
    is_match
}

fn match_har_response(req: &Request<IncomingBody>, har_log: &Log) -> Option<har::v1_2::Response> {
    use std::time::Instant;
    let now = Instant::now();
    // TODO match based
    let entry = har_log.entries.iter().find(|entry| match_url(req, entry));

    let elapsed = now.elapsed();
    println!("match_har_response took: {:.2?}", elapsed.as_millis());
    match entry {
        Some(entry) => Some(entry.response.clone()),
        None => None,
    }
}

fn uri_to_har_url(uri: &Uri) -> String {
    let path = uri.path_and_query().unwrap().as_str();
    let res = &path[1..path.len()];
    format!("https://{}", res) // TODO warning naive https
}

async fn response_examples(
    req: Request<IncomingBody>,
) -> Result<hyper::Response<http_body_util::combinators::BoxBody<bytes::Bytes, hyper::Error>>> {
    let har_log = get_har_log();

    let response = match_har_response(&req, &har_log);

    let mut builder = Response::builder();

    match response {
        Some(response) => {
            // status
            let status = response.status;
            builder = builder.status(status as u16);

            // TODO refactor to use header_mut etc functs

            // TODO redirectURL field in har?

            // headers
            for header in response.headers.iter() {
                // todo more filters required?
                // filter content encoding away. har is decoded
                let banned_headers = vec!["content-encoding", "content-length"];
                if !banned_headers.contains(&header.name.as_str().to_lowercase().as_str()) {
                    // println!("{}", header.value.clone().as_str());
                    builder =
                        builder.header(header.name.clone().as_str(), header.value.clone().as_str());
                }
            }

            // body
            // TODO rewrite urls in content
            let body = response.content.text.clone();
            match body {
                Some(body) => Ok(builder.body(full(body)).unwrap()),
                None => Ok(builder.body(empty()).unwrap()),
            }
        }
        None => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn empty() -> BoxBody {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| response_examples(req));

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
