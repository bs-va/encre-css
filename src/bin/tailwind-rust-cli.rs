use std::env;
use wax::Glob;

fn main() {
    // TODO: Clap
    // TODO: Multiple paths
    let args: Vec<String> = env::args().collect();
    let files = Glob::new(&args[1]).expect("failed to find the path");

    println!(
        "{}",
        tailwind_rust::gen_css_from_files(
            files.walk(".", usize::MAX).map(|e| e.unwrap().into_path())
        )
    );
}
