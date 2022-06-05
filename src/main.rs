mod utils;
use clap::Parser;
use colored::Colorize;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use pug::parse;
use std::{
    ffi::OsStr,
    fs::{self, metadata, OpenOptions},
    io::{ErrorKind, Write},
    panic,
    path::PathBuf,
    sync::mpsc::channel,
    time::Duration,
};
use utils::{clear_screen, is_pug_file};
use walkdir::WalkDir;

use crate::utils::gray_text;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ParsingConfig {
    /// Path to the input .pug file or path to directory containing .pug files
    #[clap(name = "dir|path")]
    input_path: String,

    #[clap(
        short = 'o',
        long = "out",
        help = "Write outputs to specified directory (Default: same directory)"
    )]
    out_dir: Option<String>,

    #[clap(
        long,
        short,
        help = "Watch for file change and automatically rerender updated files"
    )]
    watch: bool,
}

fn render_file(path_str: String, config: &ParsingConfig) {
    let contents = fs::read_to_string(&path_str).unwrap();
    let mut b: Vec<u8> = Vec::new();
    parse(contents).unwrap().to_html(&mut b).unwrap();

    let mut path = PathBuf::from(&path_str).canonicalize().unwrap();
    path.set_extension("html");

    if let Some(out_dir) = &config.out_dir {
        let out_path = PathBuf::from(out_dir);
        if out_path.exists() && !out_path.is_dir() {
            panic!("Output path should be directory");
        }

        fs::create_dir_all(out_dir).expect("Failed to create output directory");

        // If initial input path was directory, we should keep all subdirectories in place for the
        // output directory
        if config.input_path != path_str {
            let input_dir = PathBuf::from(&config.input_path).canonicalize().unwrap();
            // Replaces first portion of the file path with output path
            // input/file.html -> output/file.html
            let out_str = path.display().to_string().replace(
                &input_dir.display().to_string(),
                &out_path.canonicalize().unwrap().display().to_string(),
            );
            path = PathBuf::from(out_str);
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent.display().to_string())
            .expect("Failed to create output directory");
    }

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)
        .unwrap();

    file.write_all(&b)
        .expect(&format!("Failed write file: {}", &path.display()));

    println!(
        "{} {}",
        gray_text("Rendered"),
        path.display().to_string().green()
    );
}

fn render_dir(config: &ParsingConfig) {
    for p in WalkDir::new(&config.input_path) {
        let p = p.unwrap();
        let p = p.path();
        if p.extension().and_then(OsStr::to_str).unwrap_or("") == "pug" {
            render_file(p.display().to_string(), config);
        }
    }
}

fn watch_files(config: &ParsingConfig) {
    let path = fs::canonicalize(PathBuf::from(config.input_path.clone())).unwrap();

    clear_screen();
    println!(
        "Watching for file change: {}\n",
        path.display().to_string().green()
    );

    if path.is_dir() {
        render_dir(&config);
    } else {
        render_file(config.input_path.clone(), &config);
    }

    let (tx, rx) = channel();
    // Watches every 1 second for file change
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) => {
                    println!(
                        "{} {}",
                        gray_text("Update"),
                        path.canonicalize().unwrap().display().to_string().blue()
                    );
                    if is_pug_file(&path) {
                        render_file(path.display().to_string(), config)
                    }
                }
                _ => {}
            },
            Err(e) => panic!("{}: {}", "\nWatch error".red(), e),
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|i| {
        println!("{:#?}", i);
        if let Some(m) = i.payload().downcast_ref::<&str>() {
            return eprintln!("{}: {}", "Error".red(), m);
        }

        eprintln!("{}", "Unknown error occured".red());
    }));

    let args = ParsingConfig::parse();

    match metadata(&args.input_path) {
        Ok(meta) => {
            if args.watch {
                return watch_files(&args);
            }

            if meta.is_dir() {
                render_dir(&args);
            } else {
                render_file(args.input_path.clone(), &args);
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("File or directory not found"),
            _ => panic!("{}", e.to_string()),
        },
    };
}
