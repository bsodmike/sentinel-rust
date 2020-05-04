use crate::slack;

pub async fn notify_slack(message: &String) {
    // Prepare Slack Template
    let mut template = String::new();
    template.push_str(&String::from(r#"
    {
      "blocks": [
        {
          "type": "section",
          "text": {
            "type": "mrkdwn",
            "text":  "Hello, this is a test broadcast from your friendly *Sentinel*."#));
    template.push_str(&message);
    template.push_str(&String::from(r#""
          }
        }
      ]
    }"#));

    println!("{}", template);

  let data: serde_json::Value = serde_json::from_str(&template).unwrap();
  let (_, body_json): (hyper::Response<hyper::Body>, serde_json::Value) = 
    match slack::notify(&data).await {
    Ok(result) => result,
    Err(error) => panic!("Error: {:#?}", error)
  };

  println!("Slack response: {:#?}", body_json);
}