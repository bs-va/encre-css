use crate::DEFAULT_CONFIG_FILE;

use encre_css::{Config, EncreGenerator};
use notify::{watcher, DebouncedEvent::*, RecursiveMode, Watcher};
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

fn result_equal<T: PartialEq, E>(res1: Result<T, E>, res2: Result<T, E>) -> bool {
    if let (Ok(res1), Ok(res2)) = (res1, res2) {
        res1 == res2
    } else {
        false
    }
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

pub fn build(
    config: Option<String>,
    extra_input: Option<PathBuf>,
    output: Option<String>,
    watch: bool,
    display_time: bool,
) {
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
            }
        };

        let input = config.input.clone();
        let mut generator = EncreGenerator::from_config(config);

        if let Some(ref path) = extra_input {
            generator.scan_path(path);
        }

        // Initial generation
        gen_css(
            &generator,
            output.as_ref().map(PathBuf::from).as_ref(),
            display_time,
        );

        println!("`encre-css` successfully launched in watch mode");

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Create(ref path)
                    | Write(ref path)
                    | Remove(ref path)
                    | Rename(_, ref path) = event
                    {
                        let mut need_reloading = false;

                        // Check that the changed file is watched
                        if input.iter().any(|i| {
                            result_equal(
                                PathBuf::from(i).canonicalize(),
                                PathBuf::from(path).canonicalize(),
                            )
                        }) || extra_input.is_some()
                            && result_equal(
                                PathBuf::from(extra_input.as_ref().unwrap()).canonicalize(),
                                PathBuf::from(path).canonicalize(),
                            )
                        {
                            println!("Changes detected. Reloading…");
                            generator.reset();
                            need_reloading = true;
                        } else if result_equal(PathBuf::from(path).canonicalize(), PathBuf::from(DEFAULT_CONFIG_FILE).canonicalize()) {
                            // Handle configuration changes
                            println!("Configuration file changed. Reloading…");
                            generator = EncreGenerator::new(PathBuf::from(path));
                            need_reloading = true;
                        }

                        if need_reloading {
                            input.iter().for_each(|path| {
                                generator.scan_path(path);
                            });

                            if let Some(ref path) = extra_input {
                                generator.scan_path(path);
                            }

                            gen_css(
                                &generator,
                                output.as_ref().map(PathBuf::from).as_ref(),
                                display_time,
                            );
                        }
                    }
                }
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
