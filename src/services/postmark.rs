use std::collections::HashMap;
use hyper::{Client, Request, Response, Body, Method};
use hyper_tls::HttpsConnector;
use crate::configure;

async fn post(url: &str, payload: Body) -> Result<(Response<Body>, hyper::body::Bytes), Box<dyn std::error::Error + Send + Sync>> {
  let postmark_server_token: String = configure::fetch::<String>(String::from("postmark_server_token")).unwrap();
  let req = Request::builder()
    .method(Method::POST)
    .uri(url)
    .header("content-type", "application/json")
    .header("X-Postmark-Server-Token", postmark_server_token)
    .body(payload)?;

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, Body>(https);
  let mut response = client.request(req).await.unwrap();

  let body = hyper::body::to_bytes(response.body_mut()).await.unwrap();

  Ok((response, body))
}

pub async fn notify(data: &serde_json::Value) -> Result<(Response<Body>, serde_json::Value), Box<dyn std::error::Error + Send + Sync>> {
  let url = "https://api.postmarkapp.com/email";
  let payload = Body::from(data.to_string());

  let (response, body): (Response<Body>, hyper::body::Bytes) = match post(&url, payload).await {
    Ok(result) => result,
    Err(error) => panic!("Error [postmark]: {:#?}", error)
  };

  let body_string = String::from_utf8_lossy(&body);
  let json_value: serde_json::Value = match serde_json::from_str(&body_string) {
    Ok(value) => value,
    Err(error) => panic!("Err: parsing JSON {:#?} / body: {:#?}", error, body_string)
  };

  // Patch error from Postmark
  // "No Account or Server API tokens were supplied in the HTTP headers.
  // Please add a header for either X-Postmark-Server-Token or
  // X-Postmark-Account-Token."
  if response.status().eq(&422) 
  && 10.eq(&json_value["ErrorCode"]) {
    panic!("Postmark Error: {:#?}, {}:{}", json_value, file!(), line!());
  }

  // Patch error from Postmark
  if response.status().eq(&422) 
  && 300.eq(&json_value["ErrorCode"]) {
    panic!("Postmark Error: {:#?}, {}:{}", json_value, file!(), line!());
  }

  // Patch for success from Slack
  if response.status().eq(&200) 
  && 0.eq(&json_value["ErrorCode"])
  && "OK".eq(&json_value["Message"]) {

    let serde_ok = serde_json::json!("ok");
    let mut new_map: HashMap<&str, serde_json::Value> = HashMap::new();
    new_map.insert("success", serde_ok);

    let response_value = serde_json::to_value(&json_value).unwrap();
    new_map.insert("response", response_value.clone());

    return Ok((response, response_value));
  }

  Ok((response, json_value.clone()))
}