use rocket::{
    post,
    serde::{json::Json, Deserialize},
};

#[derive(Deserialize)]
pub struct UrlVerification {
    #[serde(rename = "type")]
    event_type: String,
    token: String,
    challenge: String,
}

#[derive(Deserialize)]
pub struct EventCallback {
    token: String,
    team_id: String,
    api_app_id: String,
    event: Event,
    event_id: String,
    event_time: i64,
    authed_users: Vec<String>,
}

#[derive(Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    user: String,
    text: String,
    ts: String,
    channel: String,
    event_ts: String,
}

#[post("/event", data = "<data>")]
pub fn handle_url_verification(data: Json<UrlVerification>) -> Option<String> {
    // Handle URL verification request
    if data.event_type.as_str() == "url_verification" {
        Some(data.challenge.clone())
    } else {
        None
    }
}

#[post("/event", data = "<data>")]
pub fn handle_event_callback(data: Json<EventCallback>) -> String {
    // Handle event callback
    match data.event.event_type.as_str() {
        "app_mention" => {
            // Handle app mention event
            // ...
            "Response to app mention".to_string()
        }
        _ => {
            // Handle other event types
            // ...
            "Response to other event".to_string()
        }
    }
}
