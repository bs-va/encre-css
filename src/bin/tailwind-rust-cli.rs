use std::{env, time::Instant};
use wax::Glob;
use tailwind_rs::TailwindGenerator;

fn main() {
    // TODO: Clap (multiple input paths, output file, config file, ...)
    let args: Vec<String> = env::args().collect();
    let (prefix, glob) = Glob::partitioned(&args[1]).unwrap();

    let start = Instant::now();
    let mut generator = TailwindGenerator::new();
    generator.scan_files(glob.walk(prefix, usize::MAX).map(|e| e.unwrap().into_path()));

    let css = generator.generate();
    let duration = start.elapsed();

    println!("{}", css);
    println!("/* CSS generated in {:?} */", duration);
}
