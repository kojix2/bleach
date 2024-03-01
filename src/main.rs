use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Perform in-place cleaning
    #[clap(short, long)]
    in_place: bool,
    /// Create a backup file before cleaning
    #[clap(short, long)]
    backup: bool,
    /// Specify types of ANSI sequences to clean
    #[clap(short, long, default_value = "all", value_name = "color|movement|all")]
    clean_types: Vec<String>,
    /// Input file to clean
    #[clap(name = "FILE")]
    file: Option<String>,
}

fn main() -> io::Result<()> {
    let opt = Opts::parse();
    let mut buffer = String::new();

    match &opt.file {
        Some(file_name) => {
            fs::File::open(file_name)?.read_to_string(&mut buffer)?;
        }
        None => {
            io::stdin().read_to_string(&mut buffer)?;
        }
    };

    let color_re = Regex::new("\x1b\\[[0-9;]*m").unwrap();
    let movement_re = Regex::new("\x1b\\[[0-9;]*[ABCD]").unwrap();

    let mut result = buffer.clone();

    if opt.clean_types.contains(&"color".to_string()) {
        let color_matches = color_re.find_iter(&result).count();
        if color_matches > 0 {
            eprintln!("Removing {} color ANSI sequences", color_matches);
        }
        result = color_re.replace_all(&result, "").into_owned();
    }
    if opt.clean_types.contains(&"movement".to_string()) {
        let movement_matches = movement_re.find_iter(&result).count();
        if movement_matches > 0 {
            eprintln!("Removing {} movement ANSI sequences", movement_matches);
        }
        result = movement_re.replace_all(&result, "").into_owned();
    }
    if opt.clean_types.contains(&"all".to_string()) {
        let color_matches = color_re.find_iter(&result).count();
        let movement_matches = movement_re.find_iter(&result).count();
        if color_matches > 0 || movement_matches > 0 {
            eprintln!(
                "Removing {} color and {} movement ANSI sequences",
                color_matches, movement_matches
            );
        }
        result = color_re.replace_all(&result, "").into_owned();
        result = movement_re.replace_all(&result, "").into_owned();
    }

    if opt.in_place {
        if let Some(file_name) = &opt.file {
            if opt.backup {
                fs::copy(file_name, format!("{}.bak", file_name))?;
            }
            fs::write(file_name, result.as_bytes())?;
        }
    } else {
        io::stdout().write_all(result.as_bytes())?;
    }

    Ok(())
}
