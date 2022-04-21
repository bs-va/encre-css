use globwalk::glob;
use std::{env, path::PathBuf};

fn main() {
    // TODO: Clap
    let args: Vec<String> = env::args().collect();
    // TODO: Multiple paths
    let files = glob(&args[1])
        .expect("failed to find the path")
        .map(|f| f.unwrap().into_path())
        .collect::<Vec<PathBuf>>();

    println!("{}", tailwind_rust::gen_css_from_files(&files));
}
