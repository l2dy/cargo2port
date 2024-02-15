use std::collections::BTreeSet;

use cargo_lock::{Lockfile, Package};

#[derive(PartialEq)]
pub enum AlignmentMode {
    Normal,
    Maxlen,
    Multiline,
}

pub fn read_packages_from_lockfiles(
    files: &Vec<String>,
) -> Result<Vec<Package>, cargo_lock::Error> {
    let lockfiles = read_lockfiles(files)?;
    let packageset = create_packageset(&lockfiles);
    let mut packages = Vec::new();

    for package in packageset {
        packages.push(package.clone())
    }

    packages.sort();

    Ok(packages)
}

pub fn format_cargo_crates(packages: Vec<Package>, mode: AlignmentMode) -> String {
    let mut output = String::new();

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

    output.push_str("cargo.crates");

    for package in packages {
        if let Some(checksum) = &package.checksum {
            output.push_str(" \\\n");

            let line = match mode {
                AlignmentMode::Maxlen => format!(
                    "    {:<name_width$}  {:<version_width$}  {}",
                    package.name,
                    package.version,
                    checksum,
                    name_width = name_min_width,
                    version_width = version_min_width
                ),
                AlignmentMode::Multiline => format!(
                    "    {} \\\n    {} \\\n    {}",
                    package.name, package.version, checksum
                ),
                AlignmentMode::Normal => format!(
                    "    {:<name_width$}  {:>version_width$}  {}",
                    package.name,
                    package.version,
                    checksum,
                    name_width = 28,
                    version_width = 8
                ),
            };

            output.push_str(&line);
        }
    }

    output
}

fn read_lockfiles(names: &Vec<String>) -> Result<Vec<Lockfile>, cargo_lock::Error> {
    let mut lockfiles: Vec<Lockfile> = vec![];

    for name in names {
        let lockfile = Lockfile::load(name)?;
        lockfiles.push(lockfile);
    }

    Ok(lockfiles)
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

    packageset
}
