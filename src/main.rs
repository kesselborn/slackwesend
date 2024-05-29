use reqwest::Client;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{catch, catchers, uri, Request};
use std::path::Path;
use tokio::join;
use tracing_subscriber::FmtSubscriber;

use anyhow::Context;
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda};
use rocket::form::{Form, FromForm};

use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes};

use wkw::random_messages::user_comes_announcement::user_comes_announcement;
use wkw::random_messages::user_comes_user_message::user_comes_user_message;
use wkw::random_messages::user_does_not_come_announcement::user_does_not_come_announcement;
use wkw::random_messages::user_does_not_come_user_message::user_does_not_come_user_message;
use wkw::wkw_action_handler::{Blocks, SlackActionPayload, User};
use wkw::wkw_command::{SlackCommandBody, SlackCommandResponse};

use tracing::{debug, error, info, Level};

#[post("/init", data = "<data>")]
async fn init(data: Form<SlackCommandBody>) -> Json<SlackCommandResponse> {
    let data = (*data).clone();
    debug!(
        "received the following data on /init:\n{}",
        json::to_string(&data).unwrap()
    );

    info!(
        "/werkommtwann was triggered from {} by {}",
        data.channel_name, data.user_name
    );
    let response = SlackCommandResponse::default();

    Json(response)
}

#[post("/", data = "<payload>")]
async fn handle_action(payload: Form<String>) -> Custom<String> {
    debug!("████████████████████████████████████████████████████████████████████████████████████████████████████ new request");
    match json::from_str(&payload) {
        Ok(payload) => {
            debug!(
                "received the following data on /:\n{}",
                json::to_string(&payload).unwrap()
            );

            let SlackActionPayload {
                user,
                response_url,
                message,
                actions,
                ref channel,
                ..
            } = payload;

            let User {
                username,
                id: user_id,
                ..
            } = user;

            let weekday = actions[0].value.clone();

            info!("'{username}' just said in channel '#{}' that they were going to the office on '{weekday}'", channel.name);

            // the actions value is the day of the week the user clicked

            let mut blocks = Blocks(message.blocks);
            let mut presence_context = blocks.find_context(&actions[0].action_id);

            debug!("presence context: {:?}", &presence_context);

            let mut user_only_message = "mmm ... something wasn't right".to_string();
            let mut public_thread_message = "mmm ... something wasn't right".to_string();

            if let Some(ref mut context) = presence_context {
                let userlist_markdown_element = &mut context.elements[1];
                let mut users = userlist_markdown_element.extract_usernames();

                debug!("current users for {weekday}: {users:?}");

                // slack replaces usernames by user_ids in the markdown, so lets check for
                // username and user_id
                if users.contains(&user_id.as_ref()) {
                    // user is already signed up for that day ... so: remove them again
                    debug!("user '{username}' removed themself from '{weekday}' (they were already signed up for '{weekday}')");
                    users.retain(|current_username| *current_username != user_id);

                    public_thread_message = user_does_not_come_announcement(&username, &weekday);
                    user_only_message = user_does_not_come_user_message(&weekday);
                } else {
                    info!("user not found");
                    user_only_message = user_comes_user_message(&weekday);
                    public_thread_message = user_comes_announcement(&username, &weekday);
                    users.push(&user_id)
                }

                debug!("new users for {weekday}: {users:?}");
                let users = users
                    .iter()
                    .map(|user| format!("<@{}>", user))
                    .collect::<Vec<_>>();

                // each context has two markdown elements: the first one shows the weekday and
                // the second one shows the users that will be present during that weekday
                // that's the element we need to adjust
                //
                if users.is_empty() {
                    userlist_markdown_element.text = "niemand".to_string();
                } else {
                    userlist_markdown_element.text = users.join(", ");
                }
            }

            let updated_message = SlackCommandResponse {
                replace_original: Some(true),
                blocks: blocks.0,
            };

            let client = Client::new();
            let update_response = async {
                debug!(
                    "sending update to {}:\n{}",
                    &response_url,
                    serde_json::to_string(&updated_message).unwrap()
                );
                match client
                    .post(&response_url)
                    .json(&updated_message)
                    .send()
                    .await
                {
                    Err(e) => error!("error sending thread response: {}", e),
                    Ok(res) => debug!(
                        "response status: {}, body: {}",
                        &res.status(),
                        &res.text().await.unwrap_or("error getting body".to_string())
                    ),
                }
                debug!("done sending update")
            };

            let json_value = json!(
                {
                    "response_type": "in_channel",
                    "replace_original": false,
                    "thread_ts": message.ts,
                    "text": public_thread_message
                }
            );

            let thread_response = async {
                debug!(
                    "sending  thread response {} to {}",
                    &json_value, response_url
                );
                match client.post(&response_url).json(&json_value).send().await {
                    Err(e) => error!("error sending direct message response: {}", e),
                    Ok(res) => debug!(
                        "response status: {}, body: {}",
                        &res.status(),
                        &res.text().await.unwrap_or("error getting body".to_string())
                    ),
                }
                debug!("done sending thread response")
            };

            let json_value = json!(
                {
                    "response_type": "ephemeral",
                    "replace_original": false,
                    "text": user_only_message
                }
            );
            let direct_message_response = async {
                debug!(
                    "sending direct user message response {} reply to {}",
                    &json_value, response_url
                );
                match client.post(&response_url).json(&json_value).send().await {
                    Err(e) => error!("error sending direct message response: {}", e),
                    Ok(res) => debug!(
                        "response status: {}, body: {}",
                        &res.status(),
                        &res.text().await.unwrap_or("error getting body".to_string())
                    ),
                }
                debug!("done sending direct user message response");
            };

            // TODO: handle errors here
            let (_update_response, _thread_response, _direct_message_response) =
                join!(update_response, thread_response, direct_message_response);

            Custom(Status::Ok, "".to_string())
        }
        Err(e) => {
            error!("error interpreting action request: {}", e);
            Custom(Status::BadRequest, e.to_string())
        }
    }
}

#[catch(default)]
fn default_catcher(_status: Status, _request: &Request) -> Redirect {
    Redirect::moved(uri!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"))
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let prefix = Path::new("/").join(include_str!("prefix").trim());

    // we handle this via env variables as this works well with aws lambda as well
    if std::env::var("VERBOSE").as_deref() == Ok("1") {
        let package_name = env!("CARGO_PKG_NAME");

        // Create a subscriber that directs events to the standard output.
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG) // Set the max log level to DEBUG
            .with_ansi(!is_running_on_lambda())
            .with_env_filter(format!("{}=debug", package_name))
            .finish();

        // Set this subscriber as the global default.
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    let prefix = prefix
        .as_os_str()
        .to_str()
        .context("error creating prefix")?;

    info!("listening on prefix {}", prefix);
    let rocket = rocket::build()
        .mount(prefix, routes![init, handle_action])
        // security by obscurity: just return a 503 for all other requests
        .register("/", catchers![default_catcher]);
    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await.unwrap();
    } else {
        // Launch local server
        let _ = rocket.launch().await?;
    }
    Ok(())
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ActionEndpoint {
    pub payload: String,
}
