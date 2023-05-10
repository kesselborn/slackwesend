use crate::wkw_action_handler::{Actions, Block, Button, Divider, Header, MarkdownText};
use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;

#[derive(Clone, Debug, FromForm, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SlackCommandBody {
    // A verification token (deprecated) used to verify that requests were legitimately being sent by Slack to your app
    pub token: Option<String>,
    // The command that was typed in to trigger this request
    pub command: String,
    // The part of the Slash Command after the command itself, and it can contain absolutely anything that the user might decide to type
    // It's common to use this text parameter to provide extra context for the command
    pub text: String,
    // A temporary webhook URL that you can use to generate messages responses
    pub response_url: String,
    // A short-lived ID that will let your app open a modal
    pub trigger_id: String,
    // The ID of the user who triggered the command
    pub user_id: String,
    // The plain text name of the user who triggered the command (deprecated, use user_id instead)
    pub user_name: String,
    // The unique identifier of your Slack app
    pub api_app_id: String,
    // The ID of the Slack workspace where the user triggered the command
    pub team_id: Option<String>,
    // The name of the Slack workspace where the user triggered the command
    pub team_name: Option<String>,
    // The ID of the channel where the user triggered the command
    pub channel_id: String,
    // The name of the channel where the user triggered the command
    pub channel_name: String,
    // The unique identifier of the Slack Enterprise Grid where the user triggered the command (only included for Enterprise Grid workspaces)
    pub enterprise_id: Option<String>,
    // The name of the Slack Enterprise Grid where the user triggered the command (only included for Enterprise Grid workspaces)
    pub enterprise_name: Option<String>,
}

// fn get_initial_block_kit() -> {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "response_type")]
#[serde(rename = "in_channel")]
pub struct SlackCommandResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_original: Option<bool>,
    pub blocks: Vec<Block>,
}

impl Default for SlackCommandResponse {
    fn default() -> Self {
        SlackCommandResponse {
            replace_original: None,
            blocks: vec![
                Block::Header(Header::new("Ich komme am: ")),
                Block::Actions(Actions {
                    elements: vec![
                        Button::new("😩 MO", "Montag"),
                        Button::new("🫡 DI", "Dienstag"),
                        Button::new("⛰️ MI", "Mittwoch"),
                        Button::new("🍻 DO", "Donnerstag"),
                        Button::new("🍾 FR", "Freitag"),
                    ],
                }),
                Block::Divider(Divider {}),
                Block::Header(Header::new("Und hier das amtliche Wahlergebnis: ")),
                Block::Context(MarkdownText::new_context(
                    vec!["*Montag*:", " ... niemand"],
                    "presence-montag",
                )),
                Block::Context(MarkdownText::new_context(
                    vec!["*Dienstag*:", "... niemand"],
                    "presence-dienstag",
                )),
                Block::Context(MarkdownText::new_context(
                    vec!["*Mittwoch*:", "... niemand"],
                    "presence-mittwoch",
                )),
                Block::Context(MarkdownText::new_context(
                    vec!["*Donnerstag*:", "... niemand"],
                    "presence-donnerstag",
                )),
                Block::Context(MarkdownText::new_context(
                    vec!["*Freitag*:", "... niemand"],
                    "presence-freitag",
                )),
            ],
        }
    }
}
