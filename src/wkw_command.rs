use rocket::FromForm;
use rocket::serde::Serialize;

#[derive(Clone, Debug, FromForm, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SlackCommandBody {
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
