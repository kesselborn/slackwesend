use aws_config::SdkConfig;
use clap::ValueEnum;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub enum State {
    On,
    Off,
    Tail,
}

pub fn execution_role_name(name: &str, region: &str) -> String {
    format!(
        "LambdaExecutionRoleFor{}InRegion{}",
        name.to_upper_camel_case(),
        region.to_upper_camel_case()
    )
}

pub fn deduce_aws_region(aws_region: &Option<String>, config: &SdkConfig) -> String {
    let mut aws_region = aws_region.to_owned();

    if aws_region.is_none() {
        aws_region = Some(
            config
                .region()
                .map_or("us-east-1".to_string(), |x| x.to_string()),
        );
    }

    debug!("aws region: {}", aws_region.as_ref().unwrap());

    aws_region.unwrap()
}
