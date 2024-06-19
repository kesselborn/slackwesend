use std::{collections::HashMap, time::Duration};

use anyhow::bail;
use aws_config::SdkConfig;
use aws_sdk_lambda::types::Environment;
use tokio::time::sleep;
use tracing::debug;

use crate::common::{deduce_aws_region, execution_role_name, State};

pub const CLOUD_WATCH_POLICY: &str =
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole";

pub async fn log(
    name: &str,
    state: State,
    aws_region: Option<String>,
    config: Option<&SdkConfig>,
) -> anyhow::Result<()> {
    debug!("loading aws env");

    let config = match config {
        Some(config) => config.to_owned(),
        None => aws_config::load_from_env().await,
    };

    let lambda_client = aws_sdk_lambda::Client::new(&config);
    let aws_region = deduce_aws_region(&aws_region, &config);
    let iam_client = aws_sdk_iam::Client::new(&config);

    let execution_role_name = execution_role_name(name, &aws_region);

    if let Err(e) = iam_client
        .get_role()
        .role_name(&execution_role_name)
        .send()
        .await
    {
        bail!(
            "error: while trying to fetch role {} ... was the app already setup? Error was: {}",
            execution_role_name,
            e
        )
    }

    match state {
        State::On => {
            iam_client
                .attach_role_policy()
                .role_name(execution_role_name)
                .policy_arn(CLOUD_WATCH_POLICY)
                .send()
                .await?;

            lambda_client
                .update_function_configuration()
                .function_name(name.to_string())
                .set_environment(Some(
                    Environment::builder()
                        .set_variables(Some(HashMap::from([(
                            "VERBOSE".to_string(),
                            "1".to_string(),
                        )])))
                        .build(),
                ))
                .send()
                .await?;
        }
        State::Off => {
            iam_client
                .detach_role_policy()
                .role_name(execution_role_name)
                .policy_arn(CLOUD_WATCH_POLICY)
                .send()
                .await?;

            lambda_client
                .update_function_configuration()
                .function_name(name.to_string())
                .set_environment(Some(
                    Environment::builder()
                        .set_variables(Some(HashMap::from([])))
                        .build(),
                ))
                .send()
                .await?;
        }
        State::Tail => {
            //           Box::pin(log(
            //               name,
            //               State::On,
            //               Some(aws_region.clone()),
            //               Some(&config),
            //           ))
            //           .await?;

            let result = tail_logs(
                &config,
                "/aws/lambda/wkw",
                r#"2024/05/30/[$LATEST]03750495d8f14cb383dbe98243dac208"#,
            )
            .await;

            //           Box::pin(log(
            //               name,
            //               State::Off,
            //               Some(aws_region.clone()),
            //               Some(&config),
            //           ))
            //           .await?;

            if let Err(e) = result {
                // bail!("error tailing logs: {}", e)
                debug!("service error: {}", e);
            }
        }
    }

    Ok(())
}

async fn tail_logs(
    config: &SdkConfig,
    log_group_name: &str,
    log_stream_name: &str,
) -> anyhow::Result<()> {
    let mut next_token: Option<String> = None;

    let client = aws_sdk_cloudwatchlogs::Client::new(config);

    loop {
        let response = client
            .get_log_events()
            .log_group_name(log_group_name)
            .log_stream_name(log_stream_name)
            .start_from_head(true)
            .send()
            .await?;

        let events = response.events(); // This will never be None; at worst, it's an empty slice.

        for event in events {
            if let (Some(message), Some(timestamp)) = (event.message(), event.timestamp()) {
                println!("{}: {}", timestamp, message);
            }
        }

        // AWS recommends not polling more frequently than once every second.
        sleep(Duration::from_secs(1)).await;

        // Clone next_token for next iteration before the response object is dropped
        next_token = response.next_forward_token().map(|s| s.to_owned());
        if next_token.is_none() {
            // Break if no new logs
            break;
        }
    }

    Ok(())
}
