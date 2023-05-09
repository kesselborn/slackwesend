use std::ops::Deref;

use rocket::form::{Form, FromForm};
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::RawJson;
use rocket::serde::json::serde_json::json;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes, Request};
use tokio::task::spawn_blocking;
use tracing::info;

struct Headers(String);

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: String,
    username: String,
    name: String,
    team_id: String,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Container {
    r#type: String,
    message_ts: String,
    channel_id: String,
    is_ephemeral: bool,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Team {
    id: String,
    domain: String,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    r#type: String,
    subtype: String,
    text: String,
    ts: String,
    bot_id: String,
    blocks: Vec<Block>,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Header {
    r#type: String,
    block_id: String,
    text: PlainText,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Button {
    r#type: String,
    action_id: String,
    text: PlainText,
    value: String,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Actions {
    r#type: String,
    block_id: String,
    elements: Vec<Button>,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Block {
    r#type: String,
    block_id: Option<String>,
    text: Option<PlainText>,
    elements: Option<Vec<Button>>,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Action {
    action_id: String,
    block_id: String,
    text: PlainText,
    value: String,
    r#type: String,
    action_ts: String,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct PlainText {
    r#type: String,
    text: String,
    emoji: bool,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct SlackPayload {
    #[field(validate = one_of("block_actions".chars()))]
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

#[derive(Debug, FromForm, Deserialize, Serialize)]
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
                "text": "MO",
                "emoji": true
              },
              "value": "monday",
              "action_id": "monday"
            },
            {
              "type": "button",
              "text": {
                "type": "plain_text",
                "text": "DI",
                "emoji": true
              },
              "value": "tuesday",
              "action_id": "tuesday"
            },
            {
              "type": "button",
              "text": {
                "type": "plain_text",
                "text": "MI",
                "emoji": true
              },
              "value": "wednesday",
              "action_id": "wednesday"
            },
            {
              "type": "button",
              "text": {
                "type": "plain_text",
                "text": "DO",
                "emoji": true
              },
              "value": "thursday",
              "action_id": "thursday"
            },
            {
              "type": "button",
              "text": {
                "type": "plain_text",
                "text": "FR",
                "emoji": true
              },
              "value": "friday",
              "action_id": "friday"
            }
          ]
        }
      ]
    }
            ))
}

#[post("/<_..>", data = "<body>")]
async fn catch_all(headers: Headers, body: Form<SlackPayload>) -> String {
    info!("catch all called");
    let response_txt = format!("{}\nBody:\n\n{:?}", headers.clone(), &body);
    spawn_blocking(move || {
        let json_value = json!(
            {
                "text": "Thanks for your request, we'll process it and get back to you."
            }
        );

        info!("sending reply to {}", &body.response_url);

        let client = reqwest::blocking::Client::new();
        client.post(&body.response_url).json(&json_value).send()
    });
    info!("{}", response_txt);
    "OK".to_string()
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![init, catch_all]);

    Ok(rocket.into())
}
