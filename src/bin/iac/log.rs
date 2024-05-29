use std::collections::HashMap;

use anyhow::bail;
use aws_sdk_lambda::types::Environment;
use tracing::debug;

use crate::common::{deduce_aws_region, execution_role_name, State};

pub const CLOUD_WATCH_POLICY: &str =
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole";

pub async fn log(name: &str, state: State, aws_region: Option<String>) -> anyhow::Result<()> {
    debug!("loading aws env");
    let config = aws_config::load_from_env().await;
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
    }

    Ok(())
}
