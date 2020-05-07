use hyper::{Client, Request, Response, Body, Method};
use hyper_tls::HttpsConnector;
use crate::errors::Error;

pub async fn get(url: &'static str) -> Result<serde_json::Value, Error> {
    let uri = hyper::Uri::from_static(url);
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let mut response: Response<Body> = client.get(uri).await.unwrap();

    let body = hyper::body::to_bytes(response.body_mut()).await.unwrap();
    // println!("{} {:?}", response.status(), body);

    let body_string = String::from_utf8_lossy(&body);
    let json_value = serde_json::from_str(&body_string)?;

    Ok(json_value)
}

pub async fn post(url: &str, payload: Body) -> Result<(Response<Body>, hyper::body::Bytes), Box<dyn std::error::Error + Send + Sync>> {
  let req = Request::builder()
    .method(Method::POST)
    .uri(url)
    .header("content-type", "application/json")
    .body(payload)?;

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let mut response = client.request(req).await.unwrap();

  let body = hyper::body::to_bytes(response.body_mut()).await.unwrap();

  Ok((response, body))
}
