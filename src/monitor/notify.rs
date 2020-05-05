use crate::slack;

pub async fn notify_slack(template: &str) {
  let data: serde_json::Value = serde_json::from_str(&template).unwrap();
  let (_, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = 
    match slack::notify(&data).await {
    Ok(result) => result,
    Err(error) => panic!("Error: {:#?}", error)
  };

  println!("Slack response: {:#?}", body_json);
}