use std::ops::Deref;

use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes, Request};
use tokio::task::spawn_blocking;
use tracing::{error, info};

struct Headers(String);

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct ActionEndpoint {
    payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: String,
    username: String,
    name: String,
    team_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Container {
    r#type: String,
    message_ts: String,
    channel_id: String,
    is_ephemeral: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Team {
    id: String,
    domain: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    r#type: String,
    subtype: String,
    text: String,
    ts: String,
    bot_id: String,
    blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Header {
    r#type: String,
    block_id: String,
    text: PlainText,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Button {
    r#type: String,
    action_id: String,
    text: PlainText,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Actions {
    r#type: String,
    block_id: String,
    elements: Vec<Button>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Block {
    r#type: String,
    block_id: Option<String>,
    text: Option<PlainText>,
    elements: Option<Vec<Button>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Action {
    action_id: String,
    block_id: String,
    text: PlainText,
    value: String,
    r#type: String,
    action_ts: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct PlainText {
    r#type: String,
    text: String,
    emoji: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct SlackPayload {
    r#type: String,
    user: User,
    api_app_id: String,
    token: String,
    container: Container,
    trigger_id: String,
    team: Team,
    enterprise: Option<String>,
    is_enterprise_install: bool,
    channel: Channel,
    message: Message,
    response_url: String,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Channel {
    id: String,
    name: String,
}

#[derive(Clone, Debug, FromForm, Serialize)]
#[serde(crate = "rocket::serde")]
struct SlackCommandBody {
    // A verification token (deprecated) used to verify that requests were legitimately being sent by Slack to your app
    token: Option<String>,
    // The command that was typed in to trigger this request
    command: String,
    // The part of the Slash Command after the command itself, and it can contain absolutely anything that the user might decide to type
    // It's common to use this text parameter to provide extra context for the command
    text: String,
    // A temporary webhook URL that you can use to generate messages responses
    response_url: String,
    // A short-lived ID that will let your app open a modal
    trigger_id: String,
    // The ID of the user who triggered the command
    user_id: String,
    // The plain text name of the user who triggered the command (deprecated, use user_id instead)
    user_name: String,
    // The unique identifier of your Slack app
    api_app_id: String,
    // The ID of the Slack workspace where the user triggered the command
    team_id: Option<String>,
    // The name of the Slack workspace where the user triggered the command
    team_name: Option<String>,
    // The ID of the channel where the user triggered the command
    channel_id: String,
    // The name of the channel where the user triggered the command
    channel_name: String,
    // The unique identifier of the Slack Enterprise Grid where the user triggered the command (only included for Enterprise Grid workspaces)
    enterprise_id: Option<String>,
    // The name of the Slack Enterprise Grid where the user triggered the command (only included for Enterprise Grid workspaces)
    enterprise_name: Option<String>,
}

impl Deref for Headers {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Headers {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        let x: Vec<String> = req
            .headers()
            .clone()
            .into_iter()
            .map(|header| header.to_string())
            .chain(Some(req.uri().path().to_string()))
            .collect();

        Outcome::Success(Headers(x.join("\n")))
    }
}

#[post("/init", data = "<data>")]
async fn init(data: Form<SlackCommandBody>) -> RawJson<json::Value> {
    let response_txt = format!("{:?}", data);
    info!("{:?}", response_txt);

    RawJson(json!(
    {
      "response_type": "in_channel",
      "blocks": [
        {
            "type": "header",
            "text": {
                "type": "plain_text",
                "text": "W K W -- Wer kommt Wann?",
                "emoji": true
            }
        },
        {
            "type": "context",
            "elements": [
                {
                    "type": "plain_text",
                    "text": "Jetzt schnell eintragen und mit etwas Glück schon morgen 1000€ auf der Straße finden!",
                    "emoji": true
                }
            ]
        },
        {
            "type": "actions",
            "elements": [
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "😩 MO",
                        "emoji": true
                    },
                    "value": "Montag",
                    "action_id": "monday"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "🫡 DI",
                        "emoji": true
                    },
                    "value": "Dienstag",
                    "action_id": "tuesday"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "⛰️ MI",
                        "emoji": true
                    },
                    "value": "Mittwoch",
                    "action_id": "wednesday"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "🍻 DO",
                        "emoji": true
                    },
                    "value": "Donnerstag",
                    "action_id": "thursday"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "🍾 FR",
                        "emoji": true
                    },
                    "value": "Freitag",
                    "action_id": "friday"
                }
            ]
        }
      ]
    }
            ))
}

#[post("/<_..>", data = "<payload>")]
async fn catch_all(headers: Headers, payload: Form<String>) -> Custom<String> {
    info!("catch all called");
    let response_txt = format!("{}\nBody:\n\n{:?}", headers.clone(), &payload);
    info!("{}", response_txt);

    match json::from_str(&payload) {
        Ok(SlackPayload {
            response_url,
            message,
            user,
            actions,
            ..
        }) => {
            spawn_blocking(move || {
                let json_value = json!(
                    {
                        "response_type": "in_channel",
                        "replace_original": false,
                        "thread_ts": message.ts,
                        "text": format!("Ha! <@{}> kommt {} ins Büro", user.username, actions[0].value)
                    }
                );

                info!("sending {} reply to {}", &json_value, response_url);
                let client = reqwest::blocking::Client::new();
                let _ = client.post(&response_url).json(&json_value).send();
                let json_value = json!(
                    {
                        "response_type": "ephemeral",
                        "replace_original": false,
                        "text": format!("Mega, dass du am {} kommst 🥰", actions[0].value)
                    }
                );
                let _ = client.post(&response_url).json(&json_value).send();
            });
            Custom(Status::Ok, "OK".to_string())
        }
        Err(e) => {
            error!("error interpreting action request: {}", e);
            Custom(Status::BadRequest, e.to_string())
        }
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![init, catch_all]);

    Ok(rocket.into())
}
