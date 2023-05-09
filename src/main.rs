use std::ops::Deref;

use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes, Request};
use slackwesend::wkw_action_handler::{Actions, Block, Button, Header, SlackActionPayload, Text};
use slackwesend::wkw_command::SlackCommandBody;
use tokio::task::spawn_blocking;
use tracing::{error, info};

struct Headers(String);

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

    let _blocks: Vec<Block> = vec![
        Block::Header(Header {
            text: Text {
                text: "Ich komme am:".to_string(),
                emoji: true,
            },
        }),
        Block::Actions(Actions {
            elements: vec![Button {
                text: Text {
                    text: "😩 MO".to_string(),
                    emoji: true,
                },
                value: "Montag".to_string(),
                action_id: "monday".to_string(),
                block_id: None,
                action_ts: None,
            }],
        }),
    ];

    RawJson(json!(
    {
    "response_type": "in_channel",
    "blocks": [
        {
            "type": "header",
            "text": {
                "type": "plain_text",
                "text": "Ich komme am: ",
                "emoji": true
            }
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
        },
        {
            "type": "divider"
        },
        {
            "type": "context",
            "block_id": "monday-presence",
            "elements": [
                {
                    "type": "mrkdwn",
                    "text": "*Montag*: "
                }
            ]
        },
        {
            "type": "context",
            "block_id": "tuesday-presence",
            "elements": [
                {
                    "type": "mrkdwn",
                    "text": "*Dienstag*: "
                }
            ]
        },
        {
            "type": "context",
            "block_id": "wednesday-presence",
            "elements": [
                {
                    "type": "mrkdwn",
                    "text": "*Mittwoch*: "
                }
            ]
        },
        {
            "type": "context",
            "block_id": "thursday-presence",
            "elements": [
                {
                    "type": "mrkdwn",
                    "text": "*Donnerstag*: "
                }
            ]
        },
        {
            "type": "context",
            "block_id": "friday-presence",
            "elements": [
                {
                    "type": "mrkdwn",
                    "text": "*Freitag*: "
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
        Ok(payload) => {
            info!("payload:\n{:?}", json::to_string(&payload).unwrap());

            let SlackActionPayload {
                user,
                response_url,
                message,
                actions,
                ..
            } = payload;

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
                info!("sending {} reply to {}", &json_value, response_url);
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

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ActionEndpoint {
    pub payload: String,
}
