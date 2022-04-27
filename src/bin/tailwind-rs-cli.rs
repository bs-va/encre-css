use std::{env, iter, path::PathBuf, time::Instant};
use tailwind_rs::TailwindGenerator;
use wax::Glob;

fn main() {
    // TODO: Clap (multiple input paths, output file, config file, ...)
    let args: Vec<String> = env::args().collect();
    let mut generator = TailwindGenerator::new();

    let (prefix, glob) = Glob::partitioned(&args[1]).unwrap();
    let path = PathBuf::from(&args[1]);
    let start = Instant::now();

    if prefix == path {
        generator.scan_files(iter::once(path));
    } else {
        generator.scan_files(
            glob.walk(prefix, usize::MAX)
                .map(|e| e.unwrap().into_path()),
        );
    }

    let css = generator.generate();
    let duration = start.elapsed();

    println!("{}", css);
    println!("/* CSS generated in {:?} */", duration);
}
