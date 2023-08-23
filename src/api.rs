use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use serde_derive::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use crate::conversion::{un_pretty_bytes, un_pretty_time};

fn default_l() -> String {
    "10KB".to_string()
}

fn default_t() -> String {
    "0".to_string()
}

fn default_s() -> u16 {
    200
}

fn default_d() -> u8 {
    0
}

#[derive(Deserialize, Serialize)]
struct QueryParameters {
    #[serde(default = "default_l")]
    l: String,
    #[serde(default = "default_t")]
    t: String,
    #[serde(default = "default_s")]
    s: u16,
    #[serde(default = "default_d")]
    d: u8,
}

impl Default for QueryParameters {
    fn default() -> Self {
        Self {
            l: default_l(),
            t: default_t(),
            s: default_s(),
            d: default_d(),
        }
    }
}

const MAX_REQUEST_SIZE: u64 = 50 * 1000 * 1000; // 50 MB
const MAX_TIMEOUT: u64 = 5 * 1000 * 1000; // 5 minutess

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut params = QueryParameters::default();

    if let Some(query) = req.uri().query() {
        params = serde_urlencoded::from_str(query).unwrap_or_default();
    }

    let mut time = 0;

    if let Some(res) = un_pretty_time(&params.t) {
        time = res;
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Time not parsed"))
            .unwrap())
    }

    if time > MAX_TIMEOUT {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Request timeout too long"))
            .unwrap())
    }

    if time > 0 {
        sleep(Duration::from_millis(time.into())).await;
    }

    let mut byte_len: usize = 10000;

    if let Some(len) = un_pretty_bytes(&params.l) {
        if len <= MAX_REQUEST_SIZE {
            byte_len = len as usize;
        } else {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Request size too large"))
                .unwrap())
        }
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Len not parsed"))
            .unwrap())
    }

    let bytes = vec![b'A'; byte_len];

    let mut body_content: String = String::from_utf8(bytes).unwrap();

    if params.d != 0 {
        body_content = serde_json::json!({
                "p": params,
                "c": body_content
            }).to_string();
    }

    let status_code = StatusCode::from_u16(params.s).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    Ok(Response::builder()
        .status(status_code)
        .body(Body::from(body_content))
        .unwrap())
}
