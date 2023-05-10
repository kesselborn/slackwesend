use rocket::form::{Form, FromForm};
use rocket::http::Status;

use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes};
use slackwesend::wkw_action_handler::{
    Actions, Block, Button, Divider, Header, MarkdownText, SlackActionPayload,
};
use slackwesend::wkw_command::{SlackCommandBody, SlackCommandResponse};

use tracing::{error, info};

#[post("/init", data = "<data>")]
async fn init(data: Form<SlackCommandBody>) -> Json<SlackCommandResponse> {
    let response_txt = format!("{:?}", data);
    info!("{:?}", response_txt);

    let response = SlackCommandResponse::default();

    Json(response)
}

#[post("/", data = "<payload>")]
async fn handle_action(payload: Form<String>) -> Custom<String> {
    match json::from_str(&payload) {
        Ok(payload) => {
            info!("payload:\n{}", json::to_pretty_string(&payload).unwrap());

            let SlackActionPayload {
                user,
                response_url,
                message,
                actions,
                ..
            } = payload;

            tokio::task::spawn_blocking(move || {
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
    let rocket = rocket::build().mount("/", routes![init, handle_action]);

    Ok(rocket.into())
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ActionEndpoint {
    pub payload: String,
}
