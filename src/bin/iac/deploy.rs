use std::fs;

use anyhow::Context;
use aws_sdk_iam::primitives::Blob;
use tracing::{debug, info};

use crate::common::deduce_aws_region;

pub async fn deploy(
    name: &str,
    aws_region: Option<String>,
    deploy_zip: &str,
) -> anyhow::Result<()> {
    debug!("loading aws env");
    let config = aws_config::load_from_env().await;
    let lambda_client = aws_sdk_lambda::Client::new(&config);
    let aws_region = deduce_aws_region(&aws_region, &config);

    let zip_blob = Blob::new(
        fs::read(deploy_zip).context(format!("error reading deploy.zip at {}", &deploy_zip))?,
    );

    info!(
        "updating lambda function {} in region {} with {}",
        name, &aws_region, deploy_zip
    );
    lambda_client
        .update_function_code()
        .function_name(name)
        .zip_file(zip_blob)
        .send()
        .await?;

    info!("    done");

    Ok(())
}
