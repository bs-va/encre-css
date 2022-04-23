use std::{env, time::Instant};
use wax::Glob;

fn main() {
    // TODO: Clap (multiple input paths, output file, config file, ...)
    let args: Vec<String> = env::args().collect();
    let files = Glob::new(&args[1]).expect("failed to find the path");

    let start = Instant::now();
    let css = tailwind_rust::gen_css_from_files(
        files.walk(".", usize::MAX).map(|e| e.unwrap().into_path()),
    );
    let duration = start.elapsed();

    println!("{}", css);
    println!("/* CSS generated in {:?} */", duration);
}
