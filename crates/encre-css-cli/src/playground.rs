use crate::DEFAULT_CONFIG_FILE;

use rand::distributions::{Alphanumeric, DistString};
use std::{fs, path::PathBuf};

const RANDOM_NAME_LENGTH: usize = 10;
const DEFAULT_HTML_CONTENT: &str = r#"<!DOCTYPE html>
  <head>
    <meta charset="utf-8">
    <title>encre-css playground</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="styles.css">
  </head>
  <body>

  </body>
</html>
"#;
const DEFAULT_CONFIG_CONTENT: &str = r#"input = ["index.html"]"#;

/// Launch a new playground environment:
/// - Create a new directory
/// - Create the `index.html` and `encre.toml` files in it and fill them with the default content
pub fn launch_playground(name: Option<String>) {
    let name = name.unwrap_or_else(|| {
        format!("playground-{}", Alphanumeric.sample_string(&mut rand::thread_rng(), RANDOM_NAME_LENGTH))
    });
    let dir_path = PathBuf::from(&name);

    fs::create_dir(&name).expect("failed to create a new directory in the current directory");
    fs::write(dir_path.join("index.html"), DEFAULT_HTML_CONTENT).expect("failed to create an `encre.toml` file in the created directory");
    fs::write(dir_path.join(DEFAULT_CONFIG_FILE), DEFAULT_CONFIG_CONTENT).expect("failed to create the configuration file in the created directory");

    println!("  `{name}` is ready!\n  To start editing it, run: `cd {name} && encre build --watch -o styles.css` and open the `index.html` file in your preferred editor");
}
