use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Header {
    pub text: Text,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Actions {
    pub elements: Vec<Button>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Divider {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Context {
    pub block_id: String,
    pub elements: Vec<MarkdownText>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "plain_text")]
pub struct Text {
    pub text: String,
    pub emoji: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "type")]
#[serde(rename = "mrkdwn")]
pub struct MarkdownText {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct State {
    pub values: serde_json::Value,
}
