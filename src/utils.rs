use hyper::{Client};
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
  let client = Client::new();
  let response: hyper::Response<hyper::Body> = client.get(url).await?;
  // println!("Response: {}", response.status());
  // println!("Headers: {:#?}\n", response.headers());

  Ok(response)
}

// pub fn post(url: &str, payload: &str) -> Result<(), Error> {
//     let client = hyper::Client::new();
//     let mut headers = Headers::new();
//     headers.set(
//         ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![(Attr::Charset, Value::Utf8)]))
//     );

//     let mut res = client.post(url).headers(headers).body(payload).send()?;
//     let mut buffer = String::new();
//     (res.read_to_string(&mut buffer)?;
//     println!("{}", buffer);
//     Ok(())
// }