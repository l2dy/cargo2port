use std::collections::BTreeSet;
use std::env;
use std::path::Path;
use std::process;

use cargo_lock::Lockfile;
use cargo_lock::Package;

#[derive(PartialEq)]
enum AlignmentMode {
    Normal,
    Maxlen,
    Multiline,
}

fn read_lockfiles(names: &Vec<String>) -> Vec<Lockfile> {
    let mut lockfiles: Vec<Lockfile> = vec![];

    for name in names {
        match Lockfile::load(name) {
            Ok(lockfile) => lockfiles.push(lockfile),
            Err(e) => {
                eprintln!("Error: {}", e);
                print_usage(1);
            }
        }
    }

    return lockfiles;
}

fn create_packageset(lockfiles: &Vec<Lockfile>) -> BTreeSet<&Package> {
    let mut packageset: BTreeSet<&Package> = BTreeSet::new();

    for lockfile in lockfiles {
        for package in &lockfile.packages {
            if package.checksum.is_none() {
                continue;
            }

            packageset.insert(package);
        }
    }

    return packageset;
}

fn read_packages_from_lockfiles(files: &Vec<String>) -> Vec<Package> {
    let lockfiles = read_lockfiles(&files);
    let packageset = create_packageset(&lockfiles);
    let mut packages = Vec::new();

    for package in packageset {
        packages.push(package.clone())
    }

    packages.sort();

    return packages;
}

fn main() {
    let mut mode = AlignmentMode::Normal;
    let mut files: Vec<String> = vec![];

    for arg in env::args().skip(1) {
        match &arg[..] {
            "" => continue,
            "--help" => print_usage(0),
            "-?" => print_usage(0),
            "-h" => print_usage(0),
            "--align=maxlen" => mode = AlignmentMode::Maxlen,
            "--align=multiline" => mode = AlignmentMode::Multiline,
            _ => {
                let name = Path::new(&arg[..]);
                match name.try_exists() {
                    Ok(true) => files.push(arg),
                    Ok(false) => {
                        eprintln!("Error: cannot find file {arg}");
                        process::exit(1);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }

    if files.len() == 0 {
        files.push("Cargo.lock".to_string())
    }

    let packages = read_packages_from_lockfiles(&files);

    if packages.len() == 0 {
        eprintln!("No packages with checksums found.");
        process::exit(0);
    }

    let mut name_min_width = 0;
    let mut version_min_width = 0;

    if mode == AlignmentMode::Maxlen {
        for package in &packages {
            let name_len = package.name.as_str().len();
            if name_len > name_min_width {
                name_min_width = name_len;
            }

            let version_len = package.version.to_string().len();
            if version_len > version_min_width {
                version_min_width = version_len;
            }
        }
    }

    print!("cargo.crates");
    for package in &packages {
        if let Some(checksum) = &package.checksum {
            println!(" \\");

            match mode {
                AlignmentMode::Maxlen => print!(
                    "    {:<name_width$}  {:<version_width$}  {}",
                    package.name,
                    package.version,
                    checksum,
                    name_width = name_min_width,
                    version_width = version_min_width
                ),
                AlignmentMode::Multiline => print!(
                    "    {} \\\n    {} \\\n    {}",
                    package.name, package.version, checksum
                ),
                AlignmentMode::Normal => print!(
                    "    {:<name_width$}  {:>version_width$}  {}",
                    package.name,
                    package.version,
                    checksum,
                    name_width = 28,
                    version_width = 8
                ),
            }
        }
    }
    println!();
}

fn print_usage(code: i32) -> () {
    let arg0 = env::args().next().unwrap_or("cargo2port".to_owned());
    eprintln!(
        "Usage: {} [--align=maxlen|multiline] <path/to/Cargo.lock>...",
        arg0
    );
    process::exit(code);
}
