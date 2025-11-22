use chrono::{Datelike, Local, TimeZone};
use rand::Rng;
use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use reqwest::header::{COOKIE, HeaderMap};
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(serde::Serialize)]
struct UserListRequest {
    token: String,
    channels: Vec<String>,
    present_first: bool,
    filter: String,
    count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

#[derive(Deserialize)]
struct UserListResponse {
    ok: bool,
    results: Vec<User>,
    next_marker: Option<String>,
}

#[derive(Deserialize)]
struct User {
    id: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get values from environment variables and bail out if they aren't provided
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not provided");
    let cookie = env::var("SLACK_COOKIE").expect("SLACK_COOKIE not provided");
    let channel_id = env::var("SLACK_CHANNEL_ID").expect("SLACK_CHANNEL_ID not provided");
    let base_url = env::var("SLACK_BASE_URL").expect("SLACK_BASE_URL not provided");
    let team_id = env::var("SLACK_TEAM_ID").expect("SLACK_TEAM_ID not provided");
    let current_user_id = env::var("SLACK_USER_ID").ok();

    // Fetch members
    let mut members = Vec::new();
    let mut cursor: Option<String> = None;
    let client = Client::new();
    let users_url = format!(
        "https://edgeapi.slack.com/cache/{team_id}/users/list?_x_app_name=client&fp=bb&_x_num_retries=0"
    );

    loop {
        let body = UserListRequest {
            token: slack_token.clone(),
            channels: vec![channel_id.clone()],
            present_first: true,
            filter: "everyone AND NOT bots AND NOT apps".to_string(),
            count: 100,
            marker: cursor.clone(),
        };

        let resp = client
            .post(&users_url)
            .header(COOKIE, &cookie)
            .json(&body)
            .send()?
            .json::<UserListResponse>()?;

        if !resp.ok {
            return Err(format!("Failed to fetch members").into());
        }

        members.extend(resp.results);

        if let Some(marker) = resp.next_marker {
            if !marker.is_empty() {
                cursor = Some(marker);
                println!(
                    "Cursor: {:?} Fetched {:?} members so far...",
                    cursor,
                    members.len()
                );
                continue;
            }
        }
        break;
    }

    // Don't send yourself tacos
    if let Some(uid) = current_user_id {
        members.retain(|u| u.id != uid);
    }

    if members.is_empty() {
        return Err("No members found in channel".into());
    }

    // Build url
    let url = format!("{base_url}/api/drafts.create");

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie.parse()?);

    // Generate scheduled times from 8PM to 11:59PM in local time everyday for 120 days
    let mut scheduled_times = Vec::new();
    let start_date = Local::now();
    let end_date = start_date + chrono::Duration::days(120);
    let mut rng = rand::rng();

    let mut current_date = start_date;
    while current_date < end_date {
        // 8PM to 11:59PM local to running device
        let scheduled_dt = Local
            .with_ymd_and_hms(
                current_date.year(),
                current_date.month(),
                current_date.day(),
                rng.random_range(20..=23),
                rng.random_range(0..60), // Random minute
                0,
            )
            .unwrap();

        if scheduled_dt > start_date {
            scheduled_times.push(scheduled_dt.timestamp());
        }
        current_date = current_date + chrono::Duration::days(1);
    }

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
        let client = Client::new();
        let res = client
            .post(&url)
            .headers(headers.clone())
            .multipart(form)
            .send()?;

        // Debug logging
        println!("Status: {}", res.status());
        let body = res.text()?;
        println!("Body: {}", body);

        // Wait a second to avoid rate limiting
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
