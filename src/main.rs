use clap::Clap;
use regex::Regex;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Your Name")]
struct Opts {
    #[clap(short, long)]
    in_place: bool,
    #[clap(short, long)]
    backup: bool,
    #[clap(name = "FILE")]
    file: Option<String>,
}

fn main() -> io::Result<()> {
    let opt = Opts::parse();

    let mut buffer = String::new();

    // Input file specified or read from stdin
    match &opt.file {
        Some(file_name) => fs::read_to_string(file_name)?.read_to_string(&mut buffer)?,
        None => io::stdin().read_to_string(&mut buffer)?,
    };

    let re = Regex::new("\x1b\\[[0-9;]*m").unwrap();
    let result = re.replace_all(&buffer, "");

    // In-place edit
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
