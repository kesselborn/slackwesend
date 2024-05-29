use crate::common::{deduce_aws_region, execution_role_name};
use anyhow::bail;
use tracing::{error, info};

pub async fn destroy(name: &str, aws_region: Option<String>, force: bool) -> anyhow::Result<()> {
    let config = aws_config::load_from_env().await;
    let iam_client = aws_sdk_iam::Client::new(&config);
    let lambda_client = aws_sdk_lambda::Client::new(&config);
    let aws_region = deduce_aws_region(&aws_region, &config);

    info!("destroying lambda function {} in {}", name, aws_region);
    if let Err(e) = lambda_client
        .delete_function()
        .function_name(name)
        .send()
        .await
    {
        let error_msg = format!("error deleting lambda function {}: {}", name, e);
        if !force {
            bail!(error_msg);
        } else {
            error!("{} -- continuing due to force mode", error_msg);
        }
    }
    info!("    done");

    let execution_role_name = execution_role_name(name, &aws_region);
    info!("destroying lambda execution role {}", &execution_role_name);
    if let Err(e) = iam_client
        .delete_role()
        .role_name(&execution_role_name)
        .send()
        .await
    {
        let error_msg = format!(
            "error deleting execution role {}: {}",
            execution_role_name, e
        );
        if !force {
            bail!(error_msg);
        } else {
            error!("{} -- continuing due to force mode", error_msg);
        }
    }
    info!("    done");

    Ok(())
}
