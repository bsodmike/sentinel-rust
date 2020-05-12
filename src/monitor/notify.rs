use crate::services::slack;
use crate::services::postmark;
use crate::errors::Error;
use hyper::{Response, Body};

pub async fn notify_slack(template: &str) {
  let data: serde_json::Value = serde_json::from_str(&template).unwrap();
  let (_, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = 
    match slack::notify(&data).await {
    Ok(result) => result,
    Err(error) => panic!("Error: {:#?}", error)
  };

  println!("Slack response: {:#?}", body_json);
}

pub async fn notify_postmark(
    subject: &String,
    template: &str,
    from_address: &str,
    replyto_address: &str,
    to_address: &str
  ) -> Result<(Response<Body>, serde_json::Value), Error> {
  let mut postmark_template = String::new();
  postmark_template.push_str(r#"
  {
    "From": ""#);
    postmark_template.push_str(from_address);
    postmark_template.push_str(r#"",
    "ReplyTo": ""#);
    postmark_template.push_str(replyto_address);
    postmark_template.push_str(r#"",
    "To": ""#);
    postmark_template.push_str(to_address);
    postmark_template.push_str(r#"",
    "Subject": ""#);
    postmark_template.push_str(subject);
    postmark_template.push_str(r#"",
    "TextBody": ""#);
    postmark_template.push_str(template);
    postmark_template.push_str(r#""
  }"#);
  println!("Postmark Template: {:#?}", &postmark_template);
  
  let data: serde_json::Value = serde_json::from_str(&postmark_template).unwrap();
  let (response, response_value): (hyper::Response<hyper::Body>, serde_json::Value) = match postmark::notify(&data).await {
    Ok(value) => value,
    Err(error) => panic!("Error: {:#?}", error)
  };

  println!("Postmark response: {:#?}", response_value);
  Ok((response, response_value))
}