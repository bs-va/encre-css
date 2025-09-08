use crate::DEFAULT_CONFIG_FILE;

use bool_ext::BoolExt;
use color_eyre::eyre::{eyre, Report};
use encre_css::{
    error::{Error, Result},
    generate, Config as EncreConfig,
};
use log::{info, warn};
use notify::{event::ModifyKind, EventKind, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    result,
    sync::{mpsc::channel, Arc},
};
use wax::Glob;

use super::utils::has_file_match;

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

#[derive(thiserror::Error, Debug)]
pub(crate) enum ScanError {
    #[error(transparent)]
    Glob(#[from] wax::BuildError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    IntConversion(#[from] std::num::TryFromIntError),
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

fn scan_path<T: AsRef<Path>>(
    glob_path: T,
    buffer: &mut String,
) -> result::Result<usize, ScanError> {
    let glob_string = glob_path.as_ref().to_string_lossy();
    let (prefix, glob) = Glob::new(glob_string.as_ref())?.partition();
    if prefix == glob_path.as_ref() && prefix.is_file() {
        let file_len: usize = prefix
            .metadata()?
            .len()
            .try_into()
            .inspect_err(|_e| warn!("File {} is too big. Skip!", prefix.to_string_lossy()))?;
        let mut file = fs::File::open(&glob_path)?;
        buffer.reserve(file_len);
        let read = file.read_to_string(buffer)?;
        Ok(read)
    } else {
        let how_much_read: usize = glob
            .walk(prefix.clone())
            .filter_map(|entry| -> Option<usize> {
                let entry = entry.ok()?;
                let path = entry.path();
                // Return early if `path` is not a file.
                path.is_file().then_some(())?;
                let file_len: usize = path
                    .metadata()
                    .ok()?
                    .len()
                    .try_into()
                    .inspect_err(|_e| warn!("File {} is too big. Skip!", path.to_string_lossy()))
                    .ok()?;
                let mut file = fs::File::open(path).ok()?;
                buffer.reserve(file_len);
                file.read_to_string(buffer).ok()
            })
            .sum();
        Ok(how_much_read)
    }
}

fn build_single<T: AsRef<Path>>(
    config_file: &str,
    extra_input: Option<T>,
    output: Option<String>,
) -> color_eyre::Result<()> {
    let config = Config::from_file(config_file)
        .inspect_err(|e| warn!("{e}"))
        .unwrap_or_default();

    let mut buffer = String::new();
    // We have a list of glob paths. We will iterate over them, calling `scan_path` on each.
    // Each glob path can make `scan_path`. But we won't make the whole function failed just
    // because one `scan_path` fails. We collect the successful ones and failed ones.
    // We will make this function fail if no `scan_path` is successful.
    let mut failed_scans = vec![];
    let mut successful_scans = vec![];

    if let Some(glob_path) = extra_input {
        match scan_path(glob_path, &mut buffer) {
            Ok(s) => successful_scans.push(s),
            Err(e) => failed_scans.push(e),
        }
    }

    let success_iter = config
        .input
        .iter()
        .map(|glob_path| {
            info!("Scan {}...", glob_path.to_string_lossy());
            scan_path(glob_path, &mut buffer)
        })
        .filter_map(|s| s.map_err(|e| failed_scans.push(e)).ok());
    successful_scans.extend(success_iter);

    // If no scan is successful, we take the first scan error and use as the error for this function and return early.
    successful_scans.is_empty().and_err_with(|| {
        failed_scans
            .pop()
            .map(Report::from)
            .unwrap_or(eyre!("No path to scan!"))
    })?;

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
        let config = Config::from_file(config_file).unwrap_or_else(|e| {
            warn!("Failed to load config from file. {e}");
            Config::default()
        });

        (Arc::new(config.input), config.encre_config)
    };

    let watched_dir = match extra_input.and_then(|i| i.as_ref().parent()) {
        Some(p) => p.to_path_buf(),
        None => env::current_dir()?.clone(),
    };
    // Due to https://github.com/notify-rs/notify/issues/247, the whole current directory is
    // watched
    watcher.watch(&watched_dir, RecursiveMode::Recursive)?;

    let mut buffer = String::new();

    // We have a list of glob paths. We will iterate over them, calling `scan_path` on each.
    // Each glob path can make `scan_path`. But we won't make the whole function failed just
    // because one `scan_path` fails. We collect the successful ones and failed ones.
    // We will make this function fail if no `scan_path` is successful.
    let mut failed_scans = vec![];
    let mut successful_scans = vec![];

    // Initial generation
    if let Some(glob_path) = extra_input {
        match scan_path(glob_path, &mut buffer) {
            Ok(s) => successful_scans.push(s),
            Err(e) => failed_scans.push(e),
        }
    }

    let success_iter = input
        .iter()
        .map(|glob_path| scan_path(glob_path, &mut buffer))
        .filter_map(|s| s.map_err(|e| failed_scans.push(e)).ok());
    successful_scans.extend(success_iter);

    // If no scan is successful, we take the first scan error and use as the error for this function and return early.
    successful_scans.is_empty().and_err_with(|| {
        failed_scans
            .pop()
            .map(Report::from)
            .unwrap_or(eyre!("No path to scan!"))
    })?;

    gen_css([buffer.as_str()], &config, output.as_ref())?;

    println!("`encre-css` successfully launched in watch mode");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                let mut need_reloading = false;

                let files = input
                    .iter()
                    .filter_map(|path| {
                        let glob_string = path.as_path().to_str()?;
                        Glob::new(glob_string)
                            .inspect_err(|e| info!("{e}"))
                            .map(|g| {
                                let (prefix, g) = g.partition();
                                (prefix, g, path)
                            })
                            .ok()
                    })
                    .flat_map(|(prefix, glob, glob_path)| {
                        if prefix == *glob_path {
                            vec![prefix]
                        } else {
                            glob.walk(prefix)
                                .filter_map(|wr| wr.map(wax::WalkEntry::into_path).ok())
                                .collect::<Vec<_>>()
                        }
                    });

                let extra_input_files = if let Some(extra_input) = extra_input {
                    let (prefix, glob) = extra_input
                        .as_ref()
                        .to_str()
                        .map(Glob::new)
                        .transpose()?
                        .map(Glob::partition)
                        .ok_or(eyre!("Non UTF-8 path!"))?;

                    if prefix == *extra_input.as_ref() {
                        Some(vec![prefix])
                    } else {
                        Some(
                            glob.walk(prefix)
                                .filter_map(|wr| wr.map(wax::WalkEntry::into_path).ok())
                                .collect::<Vec<_>>(),
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

                let canonical_reported_pathbufs = event
                    .paths
                    .iter()
                    .filter_map(|p| p.canonicalize().ok())
                    .collect::<Vec<_>>();
                let canonial_reported_paths: Vec<_> = canonical_reported_pathbufs
                    .iter()
                    .map(PathBuf::as_path)
                    .collect();

                let canonical_file_pathbufs: Vec<_> =
                    files.filter_map(|pb| pb.canonicalize().ok()).collect();
                let canonical_file_paths = canonical_file_pathbufs.iter().map(PathBuf::as_path);

                let canonical_extra_file_pathbufs = extra_input_files
                    .as_deref()
                    .map(|v| {
                        v.iter()
                            .filter_map(|p| p.canonicalize().ok())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let canonical_extra_file_paths =
                    canonical_extra_file_pathbufs.iter().map(PathBuf::as_path);

                // Check that the changed file is watched
                if has_file_match(canonical_file_paths, &canonial_reported_paths.iter())
                    || has_file_match(canonical_extra_file_paths, &canonial_reported_paths.iter())
                {
                    println!("Changes detected. Reloading\u{2026}");
                    need_reloading = true;
                } else if PathBuf::from(DEFAULT_CONFIG_FILE)
                    .canonicalize()
                    .is_ok_and(|f| canonial_reported_paths.iter().any(|p| *p == f))
                {
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
                    successful_scans.clear();
                    failed_scans.clear();

                    if let Some(glob_path) = extra_input {
                        match scan_path(glob_path, &mut buffer) {
                            Ok(s) => successful_scans.push(s),
                            Err(e) => failed_scans.push(e),
                        }
                    }
                    let success_iter = input
                        .iter()
                        .map(|glob_path| scan_path(glob_path, &mut buffer))
                        .filter_map(|s| s.map_err(|e| failed_scans.push(e)).ok());
                    successful_scans.extend(success_iter);

                    // If no scan is successful, continue loop.
                    if successful_scans.is_empty() {
                        continue;
                    }

                    if let Err(e) = gen_css([buffer.as_str()], &config, output.as_ref()) {
                        warn!("{e}");
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {e}");
                break Err(Report::from(e));
            }
            Err(e) => {
                eprintln!("MPSC channel error: {e}");
                // The other side of channel is already closed.
                // It does not make sense to keep running.
                break Err(Report::from(e));
            }
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
