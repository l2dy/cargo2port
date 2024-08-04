use std::io::Write;

use cargo2port::{format_cargo_crates, read_packages_from_lockfiles, AlignmentMode};
use goldenfile::Mint;

fn lockfiles() -> Vec<String> {
    vec!["Cargo.lock".to_string(), "Cargo.lock".to_string()]
}

#[test]
fn test_duplicate_file_normal_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("one_file_normal").unwrap();

    let packages = read_packages_from_lockfiles(&lockfiles()).unwrap();
    let output = format_cargo_crates(packages, AlignmentMode::Normal);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_duplicate_file_maxlen_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("one_file_maxlen").unwrap();

    let packages = read_packages_from_lockfiles(&lockfiles()).unwrap();
    let output = format_cargo_crates(packages, AlignmentMode::Maxlen);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_duplicate_file_multiline_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("one_file_multiline").unwrap();

    let packages = read_packages_from_lockfiles(&lockfiles()).unwrap();
    let output = format_cargo_crates(packages, AlignmentMode::Multiline);

    writeln!(file, "{}", output).unwrap();
}

#[test]
fn test_duplicate_file_justify_mode() {
    let mut mint = Mint::new("tests/support");
    let mut file = mint.new_goldenfile("one_file_justify").unwrap();

    let packages = read_packages_from_lockfiles(&lockfiles()).unwrap();
    let output = format_cargo_crates(packages, AlignmentMode::Justify);

    writeln!(file, "{}", output).unwrap();
}
