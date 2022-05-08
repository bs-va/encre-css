use std::{fs, time::Instant, path::PathBuf};
use encre_css::EncreGenerator;
use clap::Parser;

pub const DEFAULT_CONFIG_FILE: &str = "encre.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
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

    /// Whether to display the time taken to generate the CSS
    #[clap(long)]
    display_time: bool,
}

fn main() {
    // TODO: Watch mode
    let cli = Cli::parse();

    let config_file = if let Some(ref config_file) = cli.config {
        config_file
    } else {
        DEFAULT_CONFIG_FILE
    };

    let start = Instant::now();
    let mut generator = EncreGenerator::new(config_file.into());

    if let Some(path) = cli.input {
        generator.scan_path(&path);
    }

    let css = generator.generate();
    let duration = start.elapsed();

    if let Some(file) = cli.output {
        fs::write(file, css).expect("failed to write to the file");
    } else {
        // If no file is specified, the CSS generated is written to the standard output
        println!("{}", css);
    }

    if cli.display_time {
        println!("CSS generated in {:?}", duration);
    }
}
