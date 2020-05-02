use hyper::{Client, Request, Response, Body, Method};
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

async fn fetch_url(url: hyper::Uri) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let response: Response<Body> = client.get(url).await?;
  // println!("Response: {}", response.status());
  // println!("Headers: {:#?}\n", response.headers());

  Ok(response)
}

pub async fn post(url: &'static str, payload: Body) -> Result<(Response<Body>, serde_json::Value), Box<dyn std::error::Error + Send + Sync>> {
  let mut response = post_url(url, payload).await.unwrap();

  let body = hyper::body::to_bytes(response.body_mut()).await.unwrap();

  let body_string = String::from_utf8_lossy(&body);
  let json_value = serde_json::from_str(&body_string)?;

  Ok((response, json_value))
}

async fn post_url(url: &'static str, payload: Body) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
  let req = Request::builder()
    .method(Method::POST)
    .uri(url)
    .header("content-type", "application/json")
    .body(payload)?;

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let response = client.request(req).await?;

  Ok(response)
}