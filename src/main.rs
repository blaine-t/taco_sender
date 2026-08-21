mod models;
mod scheduler;
mod slack;

use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderMap};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::thread::sleep;

use crate::models::Config;
use crate::scheduler::get_scheduled_times;
use crate::slack::build_form;
use crate::slack::{build_blocks, send_request};

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get values from environment variables and bail out if they aren't provided
    let config = Config {
        slack_token: env::var("SLACK_TOKEN").expect("SLACK_TOKEN not provided"),
        cookie: env::var("SLACK_COOKIE").expect("SLACK_COOKIE not provided"),
        channel_id: env::var("SLACK_CHANNEL_ID").expect("SLACK_CHANNEL_ID not provided"),
        user_ids: env::var("SLACK_USER_IDS").expect("SLACK_USER_IDS not provided"),
        base_url: env::var("SLACK_BASE_URL").expect("SLACK_BASE_URL not provided"),
    };

    let blocks = build_blocks(config.user_ids.clone());

    let client = Client::new();

    // Build url
    let url = format!("{}/api/drafts.create", config.base_url);

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, config.cookie.parse()?);

    let scheduled_times = get_scheduled_times();

    for (i, scheduled_time) in scheduled_times.iter().enumerate() {
        let mut elements = Vec::new();
        for j in 0..5 {
            let user_index = (i * 5 + j) % members.len();
            let user = &members[user_index];

            if j > 0 {
                elements.push(r#"{"type":"text","text":" "}"#.to_string());
            }

            elements.push(format!(
                r#"{{"type":"user","user_id":"{}"}}"#,
                user.id.trim()
            ));
        }
        elements.push(r#"{"type":"text","text":" "}"#.to_string());
        elements.push(r#"{"type":"emoji","name":"taco","unicode":"1f32e"}"#.to_string());
        elements.push(r#"{"type":"text","text":" Spreading the taco holiday cheer!"}"#.to_string());

        let blocks = format!(
            r#"[{{"type":"rich_text","elements":[{{"type":"rich_text_section","elements":[{}]}}]}}]"#,
            elements.join(",")
        );

        // Generate a unique client message ID
        let client_msg_id = uuid::Uuid::new_v4().to_string();

        // Build multipart form
        let form = multipart::Form::new()
            .text("token", slack_token.clone())
            .text("blocks", blocks)
            .text("client_msg_id", client_msg_id)
            .text(
                "destinations",
                format!(r#"[{{"channel_id":"{}"}}]"#, channel_id),
            )
            .text("file_ids", "[]")
            .text("is_from_composer", "true")
            .text("date_scheduled", scheduled_time.to_string());

        // Send request
        let res = send_request(&client, &url, headers.clone(), form)?;

        // Debug logging
        println!("Status: {}", res.status());
        let body = res.text()?;
        println!("Body: {}", body);


        // Wait a second to avoid rate limiting
        sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
