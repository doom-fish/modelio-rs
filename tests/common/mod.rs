use std::fs;
use std::path::PathBuf;

pub fn fixture_obj() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/triangle.obj")
}

pub fn output_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/test-artifacts")
        .join(name);
    fs::create_dir_all(&dir).expect("create test artifact dir");
    dir
}
