use reqwest::Error;
use reqwest::blocking::multipart::Form;
use reqwest::blocking::{Client, Response};
use reqwest::header::HeaderMap;

use crate::models::Config;

pub fn send_request(
    client: &Client,
    url: &String,
    headers: HeaderMap,
    form: Form,
) -> Result<Response, Error> {
    client.post(url).headers(headers).multipart(form).send()
}

pub fn build_form(config: &Config, blocks: String, scheduled_time: String) -> Form {
    // Generate a unique client message ID
    let client_msg_id = uuid::Uuid::new_v4().to_string();

    Form::new()
        .text("token", config.slack_token.clone())
        .text("blocks", blocks.clone())
        .text("client_msg_id", client_msg_id)
        .text(
            "destinations",
            format!(r#"[{{"channel_id":"{}"}}]"#, config.channel_id),
        )
        .text("file_ids", "[]")
        .text("is_from_composer", "true")
        .text("date_scheduled", scheduled_time)
}

pub fn build_blocks(user_ids: String) -> String {
    // Build the blocks JSON with user mentions and taco emoji
    let users: Vec<&str> = user_ids.split(',').collect();
    let mut elements = Vec::new();

    for user_id in users {
        elements.push(format!(
            r#"{{"type":"user","user_id":"{}"}}"#,
            user_id.trim()
        ));
        elements.push(r#"{"type":"text","text":" "}"#.to_string());
    }
    elements.push(r#"{"type":"emoji","name":"taco","unicode":"1f32e"}"#.to_string());

    format!(
        r#"[{{"type":"rich_text","elements":[{{"type":"rich_text_section","elements":[{}]}}]}}]"#,
        elements.join(",")
    )
}
