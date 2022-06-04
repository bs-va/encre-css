use std::{env, path::PathBuf};
use clap::{Parser, Subcommand};
use color_eyre::Report;
use tracing_subscriber::EnvFilter;

mod playground;
mod build;

use playground::launch_playground;
use build::build;

pub const DEFAULT_CONFIG_FILE: &str = "encre.toml";

#[derive(Debug, Subcommand)]
enum Commands {
    /// Launches a playground environment for rapidly prototyping new ideas
    Playground {
        /// The name of the playground (if not provided a randomly named directory will be created)
        name: Option<String>,
    },

    /// Generates the CSS styles needed
    Build {
        /// The path to a custom configuration file
        #[clap(short)]
        config: Option<String>,

        /// An extra input path not specified in the configuration file
        #[clap(short)]
        input: Option<PathBuf>,

        /// Output file which will contains the generated CSS styles
        /// (will be printed to the standard output by default)
        #[clap(short)]
        output: Option<String>,

        /// Watch for changes
        #[clap(short, long)]
        watch: bool,

        /// Whether to display the time taken to generate the CSS
        #[clap(long)]
        display_time: bool,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Report> {
    // Enable nice panic reports with a backtrace
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    color_eyre::install()?;

    // Enable tracing using the RUST_LOG environment variable
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    match args.command {
        Commands::Playground { name } => launch_playground(name),
        Commands::Build { config, input: extra_input, output, watch, display_time } => build(config, extra_input, output, watch, display_time),
    }

    Ok(())
}
