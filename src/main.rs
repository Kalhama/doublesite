use bytes::Bytes;
use clap::Parser;
use har::v1_2::Entries;
use har_v0_8_1 as har;
use http_body_util::{BodyExt, Empty, Full};
use hyper::http::{HeaderName, HeaderValue};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Uri;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use once_cell::sync::OnceCell;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "8080")]
    port: String,

    har_file_path: std::path::PathBuf,
}

static HAR_LOG: OnceCell<har::v1_2::Log> = OnceCell::new();

fn get_har_log(path: PathBuf) -> har::v1_2::Log {
    match har::from_path(path) {
        Ok(spec) => match spec.log {
            har::Spec::V1_2(log) => log,
            har::Spec::V1_3(_) => panic!("unsupported type"), // TODO support for also v1.3
        },
        _ => panic!("no file"),
    }
}

fn match_method(req: &Request<IncomingBody>, entry: &Entries) -> bool {
    Method::from_bytes(entry.request.method.as_bytes()).unwrap() == req.method()
}

fn match_url(req: &Request<IncomingBody>, entry: &Entries) -> bool {
    let request_url = uri_to_har_url(req.uri());
    let har_uri = entry.request.url.as_str();
    let is_match = request_url.contains(&har_uri);

    is_match
}

fn match_har_response(req: &Request<IncomingBody>) -> Option<har::v1_2::Response> {
    let har_log = HAR_LOG.get().unwrap();

    // TODO refactor?
    let entry = har_log
        .entries
        .iter()
        .find(|entry| match_url(req, entry) && match_method(req, entry));

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

fn not_found_response(
) -> Result<hyper::Response<http_body_util::combinators::BoxBody<bytes::Bytes, hyper::Error>>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(empty())
        .unwrap())
}

fn har_response(
    response: har::v1_2::Response,
) -> Result<hyper::Response<http_body_util::combinators::BoxBody<bytes::Bytes, hyper::Error>>> {
    // status
    let mut builder = Response::builder().status(response.status as u16);
    let headers = builder.headers_mut().unwrap();

    // TODO redirectURL field in har?

    // headers
    response
        .headers
        .iter()
        .map(|header| {
            let key = header.name.clone();
            let value = header.value.clone();
            (key, value)
        })
        .filter(|header| {
            // todo more filters required?
            // filter content encoding away. har is decoded
            let banned_headers = vec!["content-encoding", "content-length"];
            banned_headers.contains(&header.1.as_str())
        })
        .for_each(|header| {
            headers.insert(
                HeaderName::from_str(header.0.as_str()).unwrap(),
                HeaderValue::from_str(header.1.as_str()).unwrap(),
            );
        });

    // body
    // TODO rewrite urls in content
    let body = response.content.text.clone();
    match body {
        Some(body) => Ok(builder.body(full(body)).unwrap()),
        None => Ok(builder.body(empty()).unwrap()),
    }
}

async fn response(
    req: Request<IncomingBody>,
) -> Result<hyper::Response<http_body_util::combinators::BoxBody<bytes::Bytes, hyper::Error>>> {
    let response = match_har_response(&req);

    match response {
        Some(response) => har_response(response),
        None => not_found_response(),
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
    let args = Args::parse();

    pretty_env_logger::init();

    HAR_LOG.set(get_har_log(args.har_file_path)).unwrap();

    let addr: SocketAddr = format!("127.0.0.1:{}", args.port).parse().unwrap();

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| response(req));

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
