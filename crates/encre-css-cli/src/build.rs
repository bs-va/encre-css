use crate::DEFAULT_CONFIG_FILE;

use std::{fs, sync::mpsc::channel, time::{Duration, Instant}, path::PathBuf};
use encre_css::{EncreGenerator, Config};
use notify::{Watcher, RecursiveMode, DebouncedEvent::*, watcher};

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

pub fn build(config: Option<String>, extra_input: Option<PathBuf>, output: Option<String>, watch: bool, display_time: bool) {
    let config_file = if let Some(ref config_file) = config {
        config_file
    } else {
        DEFAULT_CONFIG_FILE
    };

    if watch {
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

        if let Some(ref path) = extra_input {
            generator.scan_path(path);
        }

        // Initial generation
        gen_css(&generator, output.as_ref().map(PathBuf::from).as_ref(), display_time);

        println!("`encre-css` successfully launched in watch mode");

        loop {
            match rx.recv() {
                // TODO: More clever reloading method (just reload changed files and prevent rebuilding an
                // `EncreGenerator`)
                Ok(event) => {
                    if let Create(ref path) | Write(ref path) | Remove(ref path) | Rename(_, ref path) = event {
                        // Prevent infinite loop because the watcher detects changes of the output file
                        if let Some(ref output_path) = output {
                            if let (Ok(path1), Ok(path2)) = (PathBuf::from(output_path).canonicalize(), path.canonicalize()) {
                                if path1 == path2 {
                                    continue;
                                }
                            }
                        }

                        // TODO: Handle configuration changes

                        println!("Changes detected. Reloading…");
                        generator.clear_scanned_selectors();

                        input.iter().for_each(|path| {
                            generator.scan_path(path);
                        });

                        if let Some(ref path) = extra_input {
                            generator.scan_path(path);
                        }

                        gen_css(&generator, output.as_ref().map(PathBuf::from).as_ref(), display_time);
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        let mut generator = EncreGenerator::new(PathBuf::from(config_file));

        if let Some(path) = extra_input {
            generator.scan_path(&path);
        }

        gen_css(&generator, output.map(PathBuf::from).as_ref(), display_time);
    }
}
