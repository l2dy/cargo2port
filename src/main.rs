use std::env;
use std::process;

use cargo_lock::Lockfile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maxlen_mode = false;
    let mut multiline_mode = false;

    let lockfile = match args.len() {
        1 => "Cargo.lock",
        2 => &args[1],
        3 => if args[1] == "--align=maxlen" {
            maxlen_mode = true;
            &args[2]
        } else if args[1] == "--align=multiline" {
            multiline_mode = true;
            &args[2]
        } else {
            print_usage(1)
        },
        _ => print_usage(1),
    };

    let lockfile = Lockfile::load(lockfile);
    if let Err(e) = &lockfile {
        eprintln!("Error: {}", e);
        print_usage(1);
    }
    let lockfile = lockfile.unwrap();

    let mut name_min_width = 0;
    let mut version_min_width = 0;
    if maxlen_mode {
        for package in &lockfile.packages {
            if package.checksum.is_none() {
                continue;
            }

            let name_len = package.name.as_str().len();
            let version_len = package.version.to_string().len();

            if name_len > name_min_width {
                name_min_width = name_len;
            }
            if version_len > version_min_width {
                version_min_width = version_len;
            }
        }
    }

    print!("cargo.crates");
    for package in &lockfile.packages {
        if let Some(checksum) = &package.checksum {
            println!(" \\");
            if maxlen_mode {
                print!("    {:<name_width$}  {:<version_width$}  {}",
                       package.name,
                       package.version,
                       checksum,
                       name_width = name_min_width,
                       version_width = version_min_width);
            } else if multiline_mode {
                print!("    {} \\\n    {} \\\n    {}",
                       package.name,
                       package.version,
                       checksum);
            } else {
                print!("    {:<name_width$}  {:>version_width$}  {}",
                       package.name,
                       package.version,
                       checksum,
                       name_width = 28,
                       version_width = 8);
            }
        }
    }
    println!();
}

fn print_usage(code: i32) -> &'static str {
    let arg0 = env::args().next().unwrap_or("cargo2port".to_owned());
    eprintln!("Usage: {} [--align=maxlen|multiline] <path/to/Cargo.lock>", arg0);
    process::exit(code);
}
