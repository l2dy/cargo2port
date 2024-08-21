use std::io::Write;

use cargo2port::{
    format_cargo_crates, lockfile_from_path, resolve_lockfile_packages, AlignmentMode,
};
use cargo_lock::Package;
use goldenfile::Mint;

fn lockfiles() -> Vec<Package> {
    let files = vec![
        "tests/support/multi_lockfile_one".to_string(),
        "tests/support/multi_lockfile_two".to_string(),
    ]
    .into_iter()
    .map(|f| lockfile_from_path(&f).unwrap())
    .collect();
    resolve_lockfile_packages(&files).unwrap()
}

#[test]
fn test_multi_file_normal_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("multi_file_normal").unwrap();

    let packages = lockfiles();
    let output = format_cargo_crates(packages, AlignmentMode::Normal);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_multi_file_maxlen_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("multi_file_maxlen").unwrap();

    let packages = lockfiles();
    let output = format_cargo_crates(packages, AlignmentMode::Maxlen);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_multi_file_multiline_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("multi_file_multiline").unwrap();

    let packages = lockfiles();
    let output = format_cargo_crates(packages, AlignmentMode::Multiline);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_multi_file_justify_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("multi_file_justify").unwrap();

    let packages = lockfiles();
    let output = format_cargo_crates(packages, AlignmentMode::Justify);

    writeln!(file, "{}", output).unwrap();
}
