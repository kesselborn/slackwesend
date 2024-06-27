use rocket::{catch, catchers, uri, FromForm, Request};
use rocket::{http::Status, response::Redirect};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing_subscriber::FmtSubscriber;

use anyhow::Context;
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda};

use rocket::routes;

use tracing::{info, Level};

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
        .mount(
            prefix,
            routes![
                wkw::wkw_command::init,
                wkw::wkw_action_handler::handle_action,
                wkw::wkw_event_listener::handle_url_verification
            ],
        )
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
