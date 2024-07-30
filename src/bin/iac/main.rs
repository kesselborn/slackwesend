use clap::Parser;
use common::State;
use serde::{Deserialize, Serialize};

use clap::Subcommand;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::FmtSubscriber;

mod common;
mod config;
mod deploy;
mod destroy;
mod log;
mod setup;

/// Simple lambda handler
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    /// Logging level to use
    #[arg(short, long, value_enum, global = true, default_value_t = LogLevel::Info)]
    log_level: LogLevel,

    /// Name of the lambda function
    #[arg(short, long, global = true, default_value = env!("CARGO_PKG_NAME"))]
    name: String,

    /// AWS region where to of the lambda function (by default, uses the region of your current
    /// Profile)
    #[arg(short = 'r', global = true, long)]
    aws_region: Option<String>,
}

#[derive(clap::ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Subcommand, Debug, Clone, Deserialize, Serialize)]
enum Commands {
    /// Setup the lambda function
    ///
    /// Sets up a lambda function with a public endpoint
    Setup {
        /// Handler (name of the binary that should be invoked)
        #[arg(short='H', long, default_value = env!("CARGO_PKG_NAME"))]
        handler: String,

        /// Processor architecture to use (will be determined automatically if omitted)
        #[arg(short, long)]
        architecture: Option<String>,

        /// Initial bootzip to upload
        #[arg(short, long, default_value = "lambda-artifact/deploy.zip")]
        deploy_zip: String,

        /// Force: continue even if execution role already exists
        #[arg(short = 'f', long)]
        force: bool,
    },
    /// Destroy a lambda function and it's execution role
    ///
    /// Destroys a lambda function and everything setup with setup earlier
    Destroy {
        /// Force all: don't abort when deleting one entity fails
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// Deploy latest version to lambda
    ///
    /// Deploys the given zip file to the given lambda function
    Deploy {
        /// Initial bootzip to upload
        #[arg(short, long, default_value = "lambda-artifact/deploy.zip")]
        deploy_zip: String,
    },
    Log {
        #[arg(value_enum)]
        state: State,
    },
}

fn setup_logging(level: LogLevel) {
    let log_level = match level {
        LogLevel::Debug => LevelFilter::DEBUG,
        LogLevel::Error => LevelFilter::ERROR,
        LogLevel::Info => LevelFilter::INFO,
        LogLevel::Trace => LevelFilter::TRACE,
        LogLevel::Warning => LevelFilter::WARN,
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Args {
        log_level,
        name,
        aws_region,
        ..
    } = args;
    setup_logging(log_level.clone());

    match args.cmd {
        Commands::Setup {
            architecture,
            deploy_zip,
            handler,
            force,
        } => {
            setup::setup(
                &name,
                aws_region,
                architecture,
                &deploy_zip,
                &handler,
                force,
                &format!(".{}.config", &name),
            )
            .await?;
        }
        Commands::Destroy { force } => destroy::destroy(&name, aws_region, force).await?,

        Commands::Deploy { deploy_zip } => deploy::deploy(&name, aws_region, &deploy_zip).await?,
        Commands::Log { state } => log::log(&name, state, aws_region, None).await?,
    }

    Ok(())
}
