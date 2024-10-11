use std::{
    ffi::OsString,
    fs::{self, DirEntry, ReadDir},
    path::PathBuf,
};

use clap::Parser;

/// Simple argument parser for getting a wallpaper name/path
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    ///Name of the file to use.
    #[arg(short, long)]
    name: Option<String>,
    ///Required directory location.
    #[arg(short, long)]
    directory: String,
}

fn random_name(dir: &Vec<String>, dirname: String) -> String {
    let n = fastrand::usize(..dir.len());
    let entry = &dir[n];
    let mut path = PathBuf::new();
    path.push(&dirname);
    path.push(&entry);
    path.to_string_lossy().to_string()
}

fn main() {
    let args = Args::parse();
    let wp_directory = fs::read_dir(&args.directory).unwrap();
    let names = wp_directory
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();
    let image_file = match args.name {
        Some(name) => {
            if names.contains(&name) {
                let mut p = PathBuf::new();
                p.push(&args.directory);
                p.push(&name);
                p.to_string_lossy().to_string()
            } else {
                random_name(&names, args.directory)
            }
        }
        None => random_name(&names, args.directory),
    };
    print!("{image_file}");
}
