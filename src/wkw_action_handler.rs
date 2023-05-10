use regex::Regex;
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

impl Button {
    pub fn new(label: &str, value: &str) -> Self {
        Button {
            text: Text {
                text: label.to_string(),
                emoji: true,
            },
            value: value.to_string(),
            action_id: format!("presence-{}", value.to_lowercase().to_string()),
            block_id: None,
            action_ts: None,
        }
    }
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
        assert_eq!(elements[1].extract_usernames().contains(&"bar"), true);
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
