use crate::configure;
use crate::errors;
use hyper::{Body, Response};

use crate::utils::json_request;

pub async fn notify(
    data: &serde_json::Value,
) -> Result<(Response<Body>, serde_json::Value), Box<dyn std::error::Error + Send + Sync>> {
    let slack_url: String = configure::fetch::<String>(String::from("slack_url")).unwrap();
    let payload = Body::from(data.to_string());

    let (response, body): (Response<Body>, hyper::body::Bytes) =
        match json_request::post(&slack_url, payload).await {
            Ok(result) => result,
            Err(error) => panic!("Error [slack]: {:#?}", error),
        };

    let body_string = String::from_utf8_lossy(&body);

    // Patch for success from Slack
    if response.status().eq(&200) && body_string.eq("ok") {
        let new_string = serde_json::json!({ "status": "ok"});
        return Ok((response, new_string));
    }

    // Handle invalid token error
    if response.status().eq(&403) && body_string.eq("invalid_token") {
        panic!(
            "Invalid Slack token used! Err: {:#?}",
            errors::Error::InvalidTokenError
        );
    }

    // Handle invalid team error
    if response.status().eq(&404) && body_string.eq("no_team") {
        panic!(
            "Invalid team specified within token! Err: {:#?}",
            errors::Error::InvalidTokenError
        );
    }

    let json_value = match serde_json::from_str(&body_string) {
        Ok(value) => value,
        Err(error) => panic!("Err: parsing JSON {:#?} / body: {:#?}", error, body_string),
    };

    Ok((response, json_value))
}
