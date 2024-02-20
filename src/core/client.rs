use std::time::Duration;

use reqwest::{header::HeaderMap, StatusCode};

#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
    Text,
    Buffer,
}

#[derive(Debug, Clone)]
pub struct ResponseBody {
    pub content_type: ContentType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: ResponseBody,
    pub duration: Duration,
}

pub async fn send_request(req: reqwest::Request) -> anyhow::Result<Response> {
    let start = std::time::Instant::now();
    let res = reqwest::Client::new().execute(req).await?;
    let duration = start.elapsed();
    let status = res.status();
    let headers = res.headers().clone();

    let content_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let body: ResponseBody = match content_type {
        "application/json" => ResponseBody {
            content_type: ContentType::Json,
            data: res.bytes().await?.to_vec(),
        },
        "text/plain" => ResponseBody {
            content_type: ContentType::Text,
            data: res.bytes().await?.to_vec(),
        },
        _ => ResponseBody {
            content_type: ContentType::Buffer,
            data: res.bytes().await?.to_vec(),
        },
    };

    Ok(Response {
        status,
        headers,
        body,
        duration,
    })
}

pub fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .build()
        .expect("Failed to create client")
}
