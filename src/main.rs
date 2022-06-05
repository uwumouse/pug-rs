use clap::Parser;
use colored::Colorize;
use pug::parse;
use std::{
    fs::{self, metadata, File, OpenOptions},
    io::{self, ErrorKind, Read, Write},
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

// TODO: Read file
fn parse_file(config: &ParsingConfig) {
    let path = PathBuf::from(&config.input_path);
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

    let out = fs::canonicalize(parent.join(name)).unwrap();
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

// fn parse_dir(dir_path: String) {
//     if let Ok(paths) = fs::read_dir(dir_path) {
//         for p in paths {
//             println!("{}", p.unwrap().file_name().into_string().unwrap());
//         }
//         return;
//     }

//     panic!("Unable to read derectory")
// }

fn main() {
    panic::set_hook(Box::new(|i| {
        // eprintln!("{:#?}", i);
        if let Some(m) = i.payload().downcast_ref::<&str>() {
            return eprintln!("{}: {}", "Error".red(), m);
        }

        eprintln!("{}", "Unknown error occured".red());
    }));

    let args = ParsingConfig::parse();

    match metadata(&args.input_path) {
        Ok(meta) => {
            if meta.is_dir() {
                // parse_dir(args.input_path);
            } else {
                parse_file(&args);
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("File or directory not found"),
            _ => panic!("{}", e.to_string()),
        },
    };
}
