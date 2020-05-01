use hyper::{Client, Request, Method, Body};
use hyper_tls::HttpsConnector;
use crate::errors::Error;

pub async fn get(url: &'static str) -> Result<serde_json::Value, Error> {
    let uri = hyper::Uri::from_static(url);
    let mut response = fetch_url(uri).await.unwrap();

    let body = hyper::body::to_bytes(response.body_mut()).await.unwrap();
    // println!("{} {:?}", response.status(), body);

    let body_string = String::from_utf8_lossy(&body);
    let json_value = serde_json::from_str(&body_string)?;

    Ok(json_value)
}

async fn fetch_url(url: hyper::Uri) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);
  let response: hyper::Response<hyper::Body> = client.get(url).await?;
  // println!("Response: {}", response.status());
  // println!("Headers: {:#?}\n", response.headers());

  Ok(response)
}

pub async fn post(url: &'static str, payload: hyper::Body) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
  let req = Request::builder()
    .method(Method::POST)
    .uri(url)
    .header("content-type", "application/json")
    .body(payload)?;

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);
  let response = client.request(req).await?;
  println!("Response: {}", response.status());

  Ok(response)
}