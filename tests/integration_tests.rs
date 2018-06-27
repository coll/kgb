extern crate kg;

use std::fs::remove_dir_all;
use std::path::Path;

fn teardown(dirname: &str) {
    if let Err(e) = remove_dir_all(dirname) {
        panic!("Error: {}", e.to_string());
    }
}

#[test]
fn basic() {
    kg::run("fixtures/basic");
    let txt_path = Path::new("_site/test.txt");
    assert!(txt_path.is_file());
    teardown("_site");
}

#[test]
fn markdown() {
    kg::run("fixtures/markdown");
    let html_path = Path::new("fixtures/out/markdown/cool.html");
    assert!(html_path.is_file());
    teardown("fixtures/out/markdown");
}
