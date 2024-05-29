use crate::{
    common::{deduce_aws_region, execution_role_name},
    config::Config,
};
use anyhow::{bail, Context};
use aws_sdk_iam::primitives::Blob;
use aws_sdk_lambda::types::{Architecture, FunctionCode, FunctionUrlAuthType, Runtime};
use std::fs;
use tracing::{debug, info};

pub async fn setup(
    name: &str,
    aws_region: Option<String>,
    architecture: Option<String>,
    deploy_zip: &str,
    handler: &str,
    force: bool,
    config_file: &str,
) -> anyhow::Result<()> {
    debug!("loading aws env");
    let config = aws_config::load_from_env().await;
    let iam_client = aws_sdk_iam::Client::new(&config);
    let lambda_client = aws_sdk_lambda::Client::new(&config);
    let aws_region = deduce_aws_region(&aws_region, &config);

    if lambda_client
        .get_function()
        .function_name(name)
        .send()
        .await
        .is_ok()
    {
        bail!(
            "lambda function with name {} already exists in region {}!",
            name,
            &aws_region
        );
    }

    let execution_role_name = execution_role_name(name, &aws_region);

    let existing_role = iam_client
        .get_role()
        .role_name(&execution_role_name)
        .send()
        .await;

    if existing_role.is_ok() && !force {
        bail!("role {} already exists!", &execution_role_name,);
    }

    if existing_role.is_ok() {
        debug!(
            "role {} already exists -- continuing anyways due to force",
            &execution_role_name
        )
    }

    let architecture = deduce_architecture(&architecture, std::env::consts::ARCH)?;

    let role = if existing_role.is_err() {
        create_execution_role(&iam_client, &execution_role_name).await?
    } else {
        existing_role
            .unwrap()
            .role()
            .context("error getting role from role output")?
            .to_owned()
    };

    let zip_blob = Blob::new(
        fs::read(deploy_zip).context(format!("error reading deploy.zip at {}", &deploy_zip))?,
    );

    info!(
        "creating lambda function {} in region {} (uploads lambda artifact ... so: can take a while)",
        name, &aws_region
    );
    let _ = lambda_client
        .create_function()
        .architectures(architecture.clone())
        .code(FunctionCode::builder().zip_file(zip_blob).build())
        .function_name(name)
        .handler(handler)
        .package_type(aws_sdk_lambda::types::PackageType::Zip)
        .publish(true)
        .role(&role.arn)
        .runtime(Runtime::Providedal2023)
        .send()
        .await
        .context("error updating lambda function")?;
    info!("    done");

    info!("making lambda function publicly accessible");
    let _ = lambda_client
        .add_permission()
        .function_name(name)
        .statement_id(name)
        .action("lambda:InvokeFunctionUrl")
        .principal("*")
        .function_url_auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .context("error adding public permission");
    info!("    done");

    info!("creating function url");
    let url_config = lambda_client
        .create_function_url_config()
        .function_name(name)
        .auth_type(FunctionUrlAuthType::None)
        .send()
        .await
        .context("error creating function url config")?;
    info!("    done");

    let config = Config {
        name,
        aws_region: &aws_region,
        architecture: architecture.as_str(),
        handler,
        deploy_zip,
        endpoint: &url_config.function_url,
        function_arn: &url_config.function_arn,
    };
    config.write(config_file)?;

    info!("Config:\n{config}");

    info!("    done");
    Ok(())
}

fn deduce_architecture(
    architecture_in: &Option<String>,
    local_arch: &str,
) -> anyhow::Result<Architecture> {
    let arch = match architecture_in.as_deref().unwrap_or(local_arch) {
        "x86_64" | "amd64" => Architecture::X8664,
        "aarch64" | "arm64" => Architecture::Arm64,
        arch => bail!("no valid architecture set or given (got {arch})"),
    };

    debug!("architecture: {}", arch.as_ref());

    Ok(arch)
}

async fn create_execution_role<'a>(
    iam: &aws_sdk_iam::Client,
    name: &'a str,
) -> Result<aws_sdk_iam::types::Role, anyhow::Error> {
    info!("creating execution role {}", name);
    let role_creation_result = iam
        .create_role()
        .role_name(name)
        // don't bother beautifying the json: aws does not accept json with unnecessary blanks
        .set_assume_role_policy_document(Some(r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"lambda.amazonaws.com"},"Action":"sts:AssumeRole"}]}"# .to_string()))
        .description(format!("execution role for lambda function {}", name))
        .send()
        .await?;
    info!("    done");

    Ok(role_creation_result
        .role()
        .context("error getting role")
        .unwrap()
        .to_owned())
}
