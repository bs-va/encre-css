use crate::DEFAULT_CONFIG_FILE;

use encre_css::{
    error::{Error, Result},
    generate, Config as EncreConfig,
};
use notify::{event::ModifyKind, EventKind, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{
    env, fs,
    io::{BufReader, Read, Seek, SeekFrom},
    iter,
    path::{Path, PathBuf},
    result,
    sync::{mpsc::channel, Arc},
};
use wax::Glob;

#[derive(Default, PartialEq, Debug, Deserialize)]
struct Config {
    /// Specify which files should be scanned using globs.
    #[serde(default)]
    input: Vec<PathBuf>,

    #[serde(flatten)]
    encre_config: EncreConfig,
}

impl Config {
    fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(&path).map_err(
            |e| Error::ConfigFileNotFound(path.as_ref().to_path_buf(), e),
        )?)?)
    }
}

fn result_equal<T: PartialEq, E>(res1: result::Result<T, E>, res2: result::Result<T, E>) -> bool {
    if let (Ok(res1), Ok(res2)) = (res1, res2) {
        res1 == res2
    } else {
        false
    }
}

fn gen_css<'a, T: AsRef<Path>>(
    sources: impl IntoIterator<Item = &'a str>,
    config: &EncreConfig,
    output: Option<T>,
) -> color_eyre::Result<()> {
    let css = generate(sources, config);

    if let Some(file) = output {
        if let Some(parent) = file.as_ref().parent() {
            // Create parent directories
            fs::create_dir_all(parent)?;
        }

        fs::write(file, css)?;
    } else {
        // If no file is specified, the CSS generated is written to the standard output
        println!("{css}");
    }
    Ok(())
}

fn scan_path<T: AsRef<Path>>(glob_path: T, buffer: &mut String) {
    let (prefix, glob) = match Glob::new(
        glob_path
            .as_ref()
            .to_str()
            .expect("failed to convert the glob to a string"),
    ) {
        Ok(g) => g.partition(),
        Err(e) => panic!("{}", e),
    };

    if prefix == glob_path.as_ref() && prefix.is_file() {
        match fs::File::open(&glob_path) {
            Ok(mut file) => {
                let file_len = file
                    .seek(SeekFrom::End(0))
                    .expect("failed to seek to the end of the file");
                file.rewind()
                    .expect("failed to seek to the start of the file");

                #[allow(clippy::cast_possible_truncation)]
                buffer.reserve(file_len as usize);

                if let Err(e) = file.read_to_string(buffer) {
                    eprintln!(
                        "Failed to read the file {}: {}",
                        glob_path.as_ref().display(),
                        e
                    );
                }
            }
            Err(e) => eprintln!(
                "Failed to open the file {}: {}",
                glob_path.as_ref().display(),
                e
            ),
        }
    } else {
        glob.walk(prefix).for_each(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    match fs::File::open(path) {
                        Ok(file) => {
                            let mut reader = BufReader::new(file);
                            let file_len = reader
                                .seek(SeekFrom::End(0))
                                .expect("failed to seek to the end of the file");
                            reader
                                .rewind()
                                .expect("failed to seek to the start of the file");

                            #[allow(clippy::cast_possible_truncation)]
                            buffer.reserve(file_len as usize);

                            if let Err(e) = reader.read_to_string(buffer) {
                                eprintln!(
                                    "Failed to read the file {}: {}",
                                    glob_path.as_ref().display(),
                                    e
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to open the file {}: {}",
                                glob_path.as_ref().display(),
                                e
                            );
                        }
                    }
                }
            }
        });
    }
}

fn build_single<T: AsRef<Path>>(
    config_file: &str,
    extra_input: Option<T>,
    output: Option<String>,
) -> color_eyre::Result<()> {
    let config = match Config::from_file(config_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            Config::default()
        }
    };

    let mut buffer = String::new();

    if let Some(glob_path) = extra_input {
        scan_path(glob_path, &mut buffer);
    }

    config.input.iter().for_each(|glob_path| {
        scan_path(glob_path, &mut buffer);
    });

    gen_css([buffer.as_str()], &config.encre_config, output)
}

#[allow(clippy::too_many_lines)]
fn watch<T: AsRef<Path>>(
    config_file: &str,
    extra_input: Option<&T>,
    output: Option<&String>,
) -> color_eyre::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx)?;

    let (mut input, mut config) = {
        let config = match Config::from_file(config_file) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{e}");
                Config::default()
            }
        };

        (Arc::new(config.input), config.encre_config)
    };

    // Due to https://github.com/notify-rs/notify/issues/247, the whole current directory is
    // watched
    watcher.watch(
        &extra_input
            .as_ref()
            .and_then(|i| i.as_ref().parent().map(PathBuf::from))
            .unwrap_or_else(|| env::current_dir().expect("failed to access the current directory")),
        RecursiveMode::Recursive,
    )?;

    let mut buffer = String::new();

    {
        // Initial generation
        if let Some(ref glob_path) = extra_input.as_ref() {
            scan_path(glob_path, &mut buffer);
        }

        input.iter().for_each(|glob_path| {
            scan_path(glob_path, &mut buffer);
        });

        gen_css([buffer.as_str()], &config, output.as_ref())?;
    }

    println!("`encre-css` successfully launched in watch mode");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                let mut need_reloading = false;

                let mut files = input.iter().flat_map(|glob_path| {
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

                let extra_input_files = if let Some(ref extra_input) = extra_input.as_ref() {
                    let (prefix, glob) = Glob::new(
                        extra_input
                            .as_ref()
                            .to_str()
                            .expect("failed to convert the glob to a string"),
                    )?
                    .partition();

                    if prefix == extra_input.as_ref() {
                        Some(
                            iter::once(extra_input.as_ref().to_path_buf())
                                .collect::<Vec<PathBuf>>(),
                        )
                    } else {
                        Some(
                            glob.walk(prefix)
                                .map(|e| e.unwrap().into_path())
                                .collect::<Vec<PathBuf>>(),
                        )
                    }
                } else {
                    None
                };

                if matches!(
                    event.kind,
                    EventKind::Access(..) | EventKind::Modify(ModifyKind::Metadata(..))
                ) {
                    continue;
                }

                // Check that the changed file is watched
                if files.any(|file_path| {
                    event.paths.iter().any(|event_path| {
                        result_equal(file_path.canonicalize(), event_path.canonicalize())
                    })
                }) || (extra_input_files.is_some()
                    && extra_input_files.unwrap().iter().any(|file_path| {
                        event.paths.iter().any(|event_path| {
                            result_equal(file_path.canonicalize(), event_path.canonicalize())
                        })
                    }))
                {
                    println!("Changes detected. Reloading\u{2026}");
                    need_reloading = true;
                } else if event.paths.iter().any(|event_path| {
                    result_equal(
                        event_path.canonicalize(),
                        PathBuf::from(DEFAULT_CONFIG_FILE).canonicalize(),
                    )
                }) {
                    // Handle configuration changes
                    println!("Configuration file changed. Reloading\u{2026}");

                    let (new_input, new_config) = {
                        let config = match Config::from_file(config_file) {
                            Ok(config) => config,
                            Err(e) => {
                                eprintln!("{e}");
                                Config::default()
                            }
                        };

                        (Arc::new(config.input), config.encre_config)
                    };

                    input = new_input;
                    config = new_config;
                    need_reloading = true;
                }

                if need_reloading {
                    buffer.clear();

                    if let Some(ref glob_path) = extra_input.as_ref() {
                        scan_path(glob_path, &mut buffer);
                    }

                    input.iter().for_each(|glob_path| {
                        scan_path(glob_path, &mut buffer);
                    });

                    gen_css([buffer.as_str()], &config, output.as_ref())?;
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {e}"),
            Err(e) => eprintln!("MPSC channel error: {e}"),
        }
    }
}

pub(crate) fn build<T: AsRef<Path>>(
    config: Option<&String>,
    extra_input: Option<T>,
    output: Option<String>,
    need_watch: bool,
) -> color_eyre::Result<()> {
    let config_file = if let Some(config_file) = config {
        config_file
    } else {
        DEFAULT_CONFIG_FILE
    };

    if need_watch {
        watch(config_file, extra_input.as_ref(), output.as_ref())
    } else {
        build_single(config_file, extra_input, output)
    }
}
