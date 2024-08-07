use rocket::{
    http::Status,
    post,
    request::{FromRequest, Outcome},
    serde::{json::Json, Deserialize},
    Request,
};
use tracing::debug;

#[derive(Deserialize)]
pub struct UrlVerification {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(rename = "token")]
    _token: String,
    challenge: String,
}

#[derive(Deserialize, Debug)]
pub struct EventCallback {
    _token: String,
    _team_id: String,
    _api_app_id: String,
    event: Event,
    _event_id: String,
    _event_time: i64,
    _authed_users: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    _user: String,
    _text: String,
    _ts: String,
    _channel: String,
    _event_ts: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UrlVerification {
    type Error = ();

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        return Outcome::Forward(Status::BadRequest);
        /*
                match request.guard::<UrlVerification>().await {
                    rocket::outcome::Outcome::Success(url_verification) => {
                        if url_verification.event_type.as_str() == "url_verification" {
                            Outcome::Success(url_verification)
                        } else {
                            Outcome::Forward(Status::BadRequest)
                        }
                    }
                    _ => Outcome::Forward(Status::BadRequest),
                }
        */
    }
}

#[post("/event", data = "<data>")]
pub fn handle_url_verification(data: Json<UrlVerification>, _x: UrlVerification) -> Option<String> {
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
    debug!("body: {:?}", data);
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
