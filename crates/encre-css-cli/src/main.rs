use std::{fs, sync::mpsc::channel, time::{Instant, Duration}, path::PathBuf};
use encre_css::{EncreGenerator, Config};
use clap::Parser;
use notify::{Watcher, RecursiveMode, DebouncedEvent::*, watcher};

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

    /// Watch for changes
    #[clap(short, long)]
    watch: bool,

    /// Whether to display the time taken to generate the CSS
    #[clap(long)]
    display_time: bool,
}

fn gen_css(generator: &EncreGenerator, output: Option<&PathBuf>, display_time: bool) {
    let start = Instant::now();
    let css = generator.generate();
    let duration = start.elapsed();

    if let Some(file) = output {
        fs::write(file, css).expect("failed to write to the file");
    } else {
        // If no file is specified, the CSS generated is written to the standard output
        println!("{}", css);
    }

    if display_time {
        println!("CSS generated in {:?}", duration);
    }
}

fn main() {
    let cli = Cli::parse();

    let config_file = if let Some(ref config_file) = cli.config {
        config_file
    } else {
        DEFAULT_CONFIG_FILE
    };

    if cli.watch {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();

        // Due to https://github.com/notify-rs/notify/issues/247, the whole current directory is
        // watched
        watcher.watch(".", RecursiveMode::Recursive).unwrap();

        let config = match Config::from_file(PathBuf::from(config_file)) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", e);
                Config::default()
            },
        };

        let input = config.input.clone();
        let mut generator = EncreGenerator::from_config(config);

        if let Some(ref path) = cli.input {
            generator.scan_path(path);
        }

        // Initial generation
        gen_css(&generator, cli.output.as_ref().map(PathBuf::from).as_ref(), cli.display_time);

        loop {
            match rx.recv() {
                // TODO: More clever reloading method (just reload changed files and prevent rebuilding an
                // `EncreGenerator`)
                Ok(event) => {
                    if let Create(ref path) | Write(ref path) | Remove(ref path) | Rename(_, ref path) = event {
                        // Prevent infinite loop because the watcher detects changes of the output file
                        if let Some(ref output_path) = cli.output {
                            if PathBuf::from(output_path).canonicalize().unwrap() == path.canonicalize().unwrap() {
                                continue;
                            }
                        }

                        generator.clear_scanned_selectors();

                        input.iter().for_each(|path| {
                            generator.scan_path(path);
                        });

                        if let Some(ref path) = cli.input {
                            generator.scan_path(path);
                        }

                        gen_css(&generator, cli.output.as_ref().map(PathBuf::from).as_ref(), cli.display_time);
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        let mut generator = EncreGenerator::new(PathBuf::from(config_file));

        if let Some(path) = cli.input {
            generator.scan_path(&path);
        }

        gen_css(&generator, cli.output.map(PathBuf::from).as_ref(), cli.display_time);
    }
}
