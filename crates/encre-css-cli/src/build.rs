use crate::DEFAULT_CONFIG_FILE;

use encre_css::{Config, EncreGenerator};
use notify::{watcher, DebouncedEvent::*, RecursiveMode, Watcher};
use std::{
    fs, iter,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::{Duration, Instant},
};
use wax::Glob;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

fn result_equal<T: PartialEq, E>(res1: Result<T, E>, res2: Result<T, E>) -> bool {
    if let (Ok(res1), Ok(res2)) = (res1, res2) {
        res1 == res2
    } else {
        false
    }
}

fn gen_css<T: AsRef<Path>>(generator: &EncreGenerator, output: Option<T>, display_time: bool) {
    let start = Instant::now();
    let css = generator.generate().expect("failed to generate the CSS");
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

pub fn build<T: AsRef<Path>>(
    config: Option<String>,
    extra_input: Option<T>,
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

        let config = match Config::from_file(config_file) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", e);
                Config::default()
            }
        };

        let mut generator = EncreGenerator::from_config(config);

        if let Some(ref path) = extra_input {
            generator.scan_path(path);
        }

        // Initial generation
        gen_css(&generator, output.as_ref(), display_time);

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
                        let input = &generator.get_config().input;

                        #[cfg(target_arch = "wasm32")]
                        let iter = input.iter();

                        #[cfg(not(target_arch = "wasm32"))]
                        let iter = input.par_iter();

                        let files = iter.flat_map(|glob_path| {
                            let (prefix, glob) = match Glob::new(
                                glob_path
                                    .to_str()
                                    .expect("failed to convert the glob to a string"),
                            ) {
                                Ok(g) => g.partition(),
                                Err(e) => panic!("{}", e),
                            };

                            if &prefix == glob_path {
                                iter::once(glob_path.clone()).collect::<Vec<PathBuf>>()
                            } else {
                                glob.walk(prefix)
                                    .map(|e| e.unwrap().into_path())
                                    .collect::<Vec<PathBuf>>()
                            }
                        });

                        // Check that the changed file is watched
                        if files.any(|file_path| {
                            result_equal(
                                file_path.canonicalize(),
                                PathBuf::from(path).canonicalize(),
                            )
                        }) || extra_input.is_some()
                            && result_equal(
                                extra_input.as_ref().unwrap().as_ref().canonicalize(),
                                PathBuf::from(path).canonicalize(),
                            )
                        {
                            println!("Changes detected. Reloading…");
                            need_reloading = true;
                        } else if result_equal(
                            PathBuf::from(path).canonicalize(),
                            PathBuf::from(DEFAULT_CONFIG_FILE).canonicalize(),
                        ) {
                            // Handle configuration changes
                            println!("Configuration file changed. Reloading…");

                            let config = match Config::from_file(config_file) {
                                Ok(config) => config,
                                Err(e) => {
                                    eprintln!("{}", e);
                                    Config::default()
                                }
                            };

                            generator.set_config(config);
                            need_reloading = true;
                        }

                        if need_reloading {
                            generator.reset();
                            input.iter().for_each(|p| generator.scan_path(p));

                            if let Some(ref path) = extra_input {
                                generator.scan_path(path);
                            }

                            gen_css(&generator, output.as_ref(), display_time);
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        let mut generator = EncreGenerator::new(config_file);

        if let Some(path) = extra_input {
            generator.scan_path(&path);
        }

        gen_css(&generator, output, display_time);
    }
}
