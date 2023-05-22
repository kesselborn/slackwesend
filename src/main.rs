use rocket::form::{Form, FromForm};
use rocket::http::Status;

use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{json, Deserialize, Serialize};
use rocket::{post, routes};
use slackwesend::random_messages::user_comes_announcement::user_comes_announcement;
use slackwesend::wkw_action_handler::{Blocks, SlackActionPayload, User};
use slackwesend::wkw_command::{SlackCommandBody, SlackCommandResponse};

use slackwesend::random_messages::user_does_not_come_announcement::user_does_not_come_announcement;
use tracing::{debug, error, info};

#[post("/init", data = "<data>")]
async fn init(data: Form<SlackCommandBody>) -> Json<SlackCommandResponse> {
    let data = (*data).clone();
    debug!(
        "received the following data on /init:\n{}",
        json::to_pretty_string(&data).unwrap()
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
    match json::from_str(&payload) {
        Ok(payload) => {
            debug!(
                "received the following data on /:\n{}",
                json::to_pretty_string(&payload).unwrap()
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

            tokio::task::spawn_blocking(move || {
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

                        public_thread_message =
                            user_does_not_come_announcement(&username, &weekday);
                        user_only_message = format!(
                            "völlig daneben, dass du am {weekday} jetzt doch nicht kommst!"
                        );
                    } else {
                        info!("user not found");
                        user_only_message = format!("cool, dass du am {weekday} kommst");
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

                debug!(
                    "sending update:\n{}",
                    serde_json::to_string_pretty(&updated_message).unwrap()
                );
                let client = reqwest::blocking::Client::new();
                let _ = client.post(&response_url).json(&updated_message).send();

                let json_value = json!(
                    {
                        "response_type": "in_channel",
                        "replace_original": false,
                        "thread_ts": message.ts,
                        "text": public_thread_message
                    }
                );

                info!("sending {} reply to {}", &json_value, response_url);
                let _ = client.post(&response_url).json(&json_value).send();
                let json_value = json!(
                    {
                        "response_type": "ephemeral",
                        "replace_original": false,
                        "text": user_only_message
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
