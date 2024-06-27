use rocket::http::Status;
use tokio::join;

use regex::Regex;
use reqwest::Client;
use rocket::form::Form;
use rocket::post;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{self, serde_json};
use rocket::serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::random_messages::user_comes_announcement::user_comes_announcement;
use crate::random_messages::user_comes_user_message::user_comes_user_message;
use crate::random_messages::user_does_not_come_announcement::user_does_not_come_announcement;
use crate::random_messages::user_does_not_come_user_message::user_does_not_come_user_message;
use crate::wkw_command::SlackCommandResponse;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "block_actions")]
pub struct SlackActionPayload {
    pub user: User,
    pub api_app_id: String,
    pub token: String,
    pub container: Container,
    pub trigger_id: String,
    pub team: Team,
    pub enterprise: Option<String>,
    pub is_enterprise_install: bool,
    pub channel: Channel,
    pub message: Message,
    pub state: State,
    pub response_url: String,
    pub actions: Vec<Button>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub team_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "message")]
pub struct Container {
    pub message_ts: String,
    pub channel_id: String,
    pub is_ephemeral: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    pub id: String,
    pub domain: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Channel {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "message")]
pub struct Message {
    pub subtype: String,
    pub text: String,
    pub ts: String,
    pub bot_id: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "header")]
    Header(Header),
    #[serde(rename = "actions")]
    Actions(Actions),
    #[serde(rename = "divider")]
    Divider(Divider),
    #[serde(rename = "context")]
    Context(Context),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Header {
    pub text: Text,
}

impl Header {
    pub fn new(text: &str) -> Self {
        Header {
            text: Text {
                text: text.to_string(),
                emoji: true,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Actions {
    pub elements: Vec<Button>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Divider {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Context {
    pub block_id: String,
    pub elements: Vec<MarkdownText>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "button")]
pub struct Button {
    pub text: Text,
    pub value: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_ts: Option<String>,
}

impl Button {
    pub fn new(label: &str, value: &str) -> Self {
        Button {
            text: Text {
                text: label.to_string(),
                emoji: true,
            },
            value: value.to_string(),
            action_id: format!("presence-{}", value.to_lowercase()),
            block_id: None,
            action_ts: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "plain_text")]
pub struct Text {
    pub text: String,
    pub emoji: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "mrkdwn")]
pub struct MarkdownText {
    pub text: String,
}

impl MarkdownText {
    pub fn new_context(markdown_elements: Vec<&str>, id: &str) -> Context {
        let markdown_elements = markdown_elements
            .iter()
            .map(|text| MarkdownText {
                text: text.to_string(),
            })
            .collect();

        Context {
            elements: markdown_elements,
            block_id: id.to_string(),
        }
    }

    pub fn extract_usernames(&self) -> Vec<&str> {
        let re = Regex::new(r#"<@(\w+?)>"#).unwrap();
        re.captures_iter(&self.text)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct State {
    pub values: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Blocks(pub Vec<Block>);

impl Blocks {
    pub fn find_context(&mut self, name: &str) -> Option<&mut Context> {
        for block in &mut self.0 {
            if let Block::Context(ref mut context) = block {
                if context.block_id == name {
                    return Some(context);
                }
            }
        }
        None
    }
}

#[post("/", data = "<payload>")]
pub async fn handle_action(payload: Form<String>) -> Custom<String> {
    #[cfg(not(feature = "random_messages"))]
    let random_messages = false;
    #[cfg(feature = "random_messages")]
    let random_messages = true;
    #[cfg(not(feature = "direct_user_feedback"))]
    let direct_user_feedback = false;
    #[cfg(feature = "direct_user_feedback")]
    let direct_user_feedback = true;

    {
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

                // the actions value is the day of the week the user clicked
                let weekday = actions[0].value.clone();
                info!("'{username}' just said in channel '#{}' that they were going to the office on '{weekday}'", channel.name);

                let mut user_only_message = None;
                let mut public_thread_message = None;
                let updated_message;

                {
                    // TODO: s3 lock
                    // let mut state = state.lock().unwrap();
                    // debug!("action call #{}", state.counter);
                    // if let Some(old_body) = state.value.clone() {
                    //     debug!("state value body: {}", old_body)
                    // }
                    // state.counter += 1;

                    let mut blocks;

                    // if state.value.is_some() {
                    //     blocks = serde_json::from_str(&state.value.clone().unwrap()).unwrap();
                    //     debug!("read blocks from state: {}", &state.value.clone().unwrap());
                    // } else {
                    //     debug!("using post body");
                    blocks = Blocks(message.blocks);
                    // }

                    let mut presence_context = blocks.find_context(&actions[0].action_id);

                    debug!("presence context: {:?}", &presence_context);

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

                            public_thread_message = if random_messages {
                                Some(user_does_not_come_announcement(&username, &weekday))
                            } else {
                                Some(format!(
                            "{} hat es sich anders überlegt und kommt am {} doch nicht ins Büro",
                            &username, &weekday
                        ))
                            };

                            user_only_message = if direct_user_feedback {
                                if random_messages {
                                    Some(user_does_not_come_user_message(&weekday))
                                } else {
                                    Some(format!(
                                        "schade, dass du am {} doch nicht kommst!",
                                        &weekday
                                    ))
                                }
                            } else {
                                None
                            }
                        } else {
                            info!("user not found");
                            user_only_message = if random_messages {
                                Some(user_comes_user_message(&weekday))
                            } else {
                                Some(format!("cool, dass du am {} kommst!", &weekday))
                            };

                            public_thread_message = if random_messages {
                                Some(user_comes_announcement(&username, &weekday))
                            } else {
                                Some(format!("{} kommt am {} ins Büro", &username, &weekday))
                            };
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

                    updated_message = SlackCommandResponse {
                        replace_original: Some(true),
                        blocks: blocks.clone().0,
                    };
                    // state.value = Some(
                    //     serde_json::to_string(&blocks)
                    //         .context("error serializing blocks")
                    //         .unwrap(),
                    // )
                }

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

                if direct_user_feedback {
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
                } else {
                    thread_response.await;
                    update_response.await;

                    // let (_update_response, _thread_response) = join!(update_response, thread_response);
                    // TODO: handle errors here
                }

                Custom(Status::Ok, "".to_string())
            }
            Err(e) => {
                error!("error interpreting action request: {}", e);
                Custom(Status::BadRequest, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wkw_command::SlackCommandResponse;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_usernames() {
        let Context { elements, .. } =
            MarkdownText::new_context(vec!["*Monday*:", "foo <@bar>, <@baz>, bang"], "foo");

        assert_eq!(elements[1].extract_usernames(), vec!["bar", "baz"]);
        assert!(elements[1].extract_usernames().contains(&"bar"));
    }

    #[test]
    fn find_correct_markdown_element() {
        let new_text = "foobar3000";

        let SlackCommandResponse { blocks, .. } = SlackCommandResponse::default();
        if let Some(Block::Context(Context { elements, .. })) = blocks.get(6) {
            assert_ne!(elements[1].text, new_text)
        } else {
            unreachable!("did the main structure change? two markdown elements here")
        }

        let mut blocks = Blocks(blocks);
        {
            let mut presence_mittwoch_context = blocks.find_context("presence-mittwoch");

            if let Some(ref mut context) = presence_mittwoch_context {
                context.elements[1].text = new_text.to_string()
            }

            assert!(presence_mittwoch_context.is_some())
        }
        let blocks = blocks.0;

        if let Some(Block::Context(Context { elements, .. })) = blocks.get(6) {
            assert_eq!(elements[1].text, new_text)
        } else {
            unreachable!("did the main structure change? two markdown elements here")
        }
    }
}
