use rocket::serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    pub(crate) name: &'a str,
    pub(crate) aws_region: &'a str,
    pub(crate) architecture: &'a str,
    pub(crate) handler: &'a str,
    pub(crate) deploy_zip: &'a str,
    pub(crate) endpoint: &'a str,
    pub(crate) function_arn: &'a str,
}

impl<'a> Config<'a> {
    pub fn write(&self, config_file: &str) -> std::io::Result<()> {
        fs::write(config_file, serde_json::to_string(self).unwrap())
    }
}

impl<'a> Display for Config<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Config {
            name,
            aws_region,
            architecture,
            handler,
            deploy_zip,
            endpoint,
            function_arn,
        } = self;

        f.write_fmt(format_args!(
            r#"
Lambda name:         {name}
AWS Region:          {aws_region}
Architecture:        {architecture}
Deploy-Zip:          {deploy_zip}
Handler:             {handler}
Endpoint:            {endpoint}
Function ARN:        {function_arn}
        "#
        ))
    }
}
