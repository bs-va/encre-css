use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encre_css::{generate, utils::value_matchers, Config};
use std::fs;

fn scan(c: &mut Criterion) {
    c.bench_function("scan", |b| b.iter(|| {
        generate([r#"<div class="w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#], &Config::default())
    }));

    let file_content = fs::read_to_string("tests/fixtures/index.js").unwrap();
    c.bench_function("scan_large_file", |b| {
        b.iter(|| generate([file_content.as_str()], &Config::default()))
    });
}

fn matchers(c: &mut Criterion) {
    c.bench_function("is_matching_shadow", |b| {
        b.iter(|| {
            value_matchers::is_matching_shadow(black_box("10px_10px_min(1px,2px)_10px_rgb(1,1,1)"));
        })
    });

    c.bench_function("is_matching_color", |b| {
        b.iter(|| {
            value_matchers::is_matching_color(black_box("rgba(12,12,12,0.12)"));
        })
    });
}

fn generation(c: &mut Criterion) {
    c.bench_function("scan and generate", move |b| {
        b.iter(move || {
            generate([r#"<div class="w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#], &Config::default());
        })
    });
}

criterion_group!(benches, scan, matchers, generation);
criterion_main!(benches);
