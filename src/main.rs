use clap::Parser;
use colored::Colorize;
use pug::parse;
use std::{
    ffi::OsStr,
    fs::{self, metadata, OpenOptions},
    io::{ErrorKind, Write},
    panic,
    path::PathBuf,
};

/// Simple program to greet a person
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
}

fn parse_file(path: String, config: &ParsingConfig) {
    let path = PathBuf::from(path);
    let contents = fs::read_to_string(&path).unwrap();
    let mut b: Vec<u8> = Vec::new();
    parse(contents).unwrap().to_html(&mut b).unwrap();

    let mut parent = path.parent().unwrap().to_path_buf();

    if let Some(out) = &config.out_dir {
        parent = PathBuf::from(out);
        if !parent.is_dir() {
            panic!("Output path should be directory");
        }
    }

    let mut name = path
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    name.push_str(".html");

    let out = parent.join(name).canonicalize().unwrap();
    let out = out.to_str().unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&out)
        .unwrap();

    file.write_all(&b)
        .expect(&format!("Failed write file: {}", &out));

    println!("{} {}", "Rendered".truecolor(187, 196, 189), out.green());
}

fn parse_dir(config: &ParsingConfig) {
    if let Ok(paths) = fs::read_dir(&config.input_path) {
        for p in paths {
            let p = p.unwrap().path();
            if p.extension().and_then(OsStr::to_str).unwrap_or("") == "pug" {
                parse_file(p.display().to_string(), config);
            }
        }
        return;
    }

    panic!("Unable to read derectory")
}

fn main() {
    panic::set_hook(Box::new(|i| {
        eprintln!("{:#?}", i);
        if let Some(m) = i.payload().downcast_ref::<&str>() {
            return eprintln!("{}: {}", "Error".red(), m);
        }

        eprintln!("{}", "Unknown error occured".red());
    }));

    let args = ParsingConfig::parse();

    match metadata(&args.input_path) {
        Ok(meta) => {
            if meta.is_dir() {
                parse_dir(&args);
            } else {
                parse_file(args.input_path.clone(), &args);
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("File or directory not found"),
            _ => panic!("{}", e.to_string()),
        },
    };
}
