extern crate quick_xml;
extern crate serde;

use std::env;
use serde::{Serialize, Deserialize};
use quick_xml::de::{from_reader};
use quick_xml::{Reader};
use std::process::exit;
use std::io::{BufRead, BufReader, Cursor};
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Response, Server, Body, Method, StatusCode, HeaderMap};
use serde::de::Error;
use hyper::header::{CONTENT_TYPE, HeaderValue};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    section: Option<Vec<Section>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Title {
    #[serde(rename = "$value")]
    pub body: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FB2Body {
    title: Option<Title>,
    section: Option<Vec<Section>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Author {
    #[serde(rename = "first-name", skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(rename = "middle-name", skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    #[serde(rename = "last-name", skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(rename = "home-page", skip_serializing_if = "Option::is_none")]
    home_page: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Sequence {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Link {
    #[serde(rename = "l:href", skip_serializing_if = "Option::is_none")]
    href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CoverPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<Link>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TitleInfo {
    genre: Vec<String>,
    author: Vec<Author>,
    #[serde(rename = "book-title")]
    book_title: String,
    // TODO: panic on this on custom formatting annotations
    // #[serde(skip_serializing_if = "Option::is_none")]
    // annotation: Option<Annotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coverpage: Option<CoverPage>,
    lang: String,
    #[serde(rename = "src-lang", skip_serializing_if = "Option::is_none")]
    src_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    translator: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence: Option<Vec<Sequence>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Annotation {
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct History {
    #[serde(rename = "$value")]
    pub body: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DocumentInfo {
    author: Vec<Author>,
    #[serde(rename = "program-used", skip_serializing_if = "Option::is_none")]
    program_used: Option<String>,
    date: String,
    #[serde(rename = "src-url", skip_serializing_if = "Option::is_none")]
    src_url: Option<Vec<String>>,
    #[serde(rename = "src-ocr", skip_serializing_if = "Option::is_none")]
    src_ocr: Option<String>,
    id: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    history: Option<History>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PublishInfo {
    #[serde(rename = "book-name", skip_serializing_if = "Option::is_none")]
    book_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    isbn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence: Option<Vec<Sequence>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CustomInfo {
    #[serde(rename = "info-type", skip_serializing_if = "Option::is_none")]
    into_type: Option<String>,
    #[serde(rename = "$value")]
    pub body: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Description {
    #[serde(rename = "title-info")]
    title_info: TitleInfo,
    #[serde(rename = "src-title-info", skip_serializing_if = "Option::is_none")]
    src_title_info: Option<TitleInfo>,
    #[serde(rename = "document-info")]
    document_info: DocumentInfo,
    #[serde(rename = "publish-info", skip_serializing_if = "Option::is_none")]
    publish_info: Option<PublishInfo>,
    #[serde(rename = "custom-info", skip_serializing_if = "Option::is_none")]
    custom_info: Option<Vec<CustomInfo>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FictionBook {
    description: Description,
    body: FB2Body,
}

fn reader_to_json<R: BufRead>(reader: R) -> serde_json::Result<String> {
    let fb2: FictionBook = match from_reader(reader) {
        Ok(f) => f,
        Err(e) => return Err(
            serde_json::Error::custom(format!("{{\"error\": \"{}\"}}", e.to_string()))
        )
    };

    serde_json::to_string(&fb2)
}

async fn process(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/parse") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let reader = Reader::from_reader(
                BufReader::new(Cursor::new(full_body.to_vec()))
            );

            let mut headers = HeaderMap::new();

            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

            let mut ok = Response::default();

            *ok.body_mut() = Body::from(
                match reader_to_json(reader.into_underlying_reader()) {
                    Ok(r) => r,
                    Err(e) => {
                        let mut server_error = Response::default();
                        *server_error.body_mut() = Body::from(e.to_string());
                        *server_error.headers_mut() = headers;
                        *server_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        return Ok(server_error)
                    }
                }
            );

            *ok.headers_mut() = headers;
            *ok.status_mut() = StatusCode::OK;

            Ok(ok)
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Choose mode");
        exit(0);
    }

    let mode = &args[1];
    if mode == "cli" {
        if args.len() < 3 {
            println!("Choose file");
            exit(0);
        }

        let path = &args[2];
        let reader = match Reader::from_file(path) {
            Ok(f) => f,
            Err(e) => panic!(e.to_string()),
        };

        println!("{}", reader_to_json(reader.into_underlying_reader())?);
    }

    if mode == "server" {
        pretty_env_logger::init();

        let make_svc = make_service_fn(|_conn| {
            async { Ok::<_, Infallible>(service_fn(process)) }
        });

        let addr = ([127, 0, 0, 1], 3000).into();

        let server = Server::bind(&addr)
            .serve(make_svc);

        println!("Listening on http://{}", addr);

        server.await?;
    }

    Ok(())
}
