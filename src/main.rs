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
    /// Verbose mode
    #[clap(short, long)]
    verbose: bool,
    /// Quiet mode
    #[clap(short, long)]
    quiet: bool,
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

    let combined_re = Regex::new("\x1b\\[[0-9;]*[mABCD]").unwrap();

    let result = if opt.clean_types.contains(&"all".to_string()) || (opt.clean_types.contains(&"color".to_string()) && opt.clean_types.contains(&"movement".to_string())) {
        clean_text(&opt, &combined_re, &buffer)
    } else {
        let mut temp_result = buffer.clone();
        if opt.clean_types.contains(&"color".to_string()) {
            let color_re = Regex::new("\x1b\\[[0-9;]*m").unwrap();
            temp_result = clean_text(&opt, &color_re, &temp_result);
        }
        if opt.clean_types.contains(&"movement".to_string()) {
            let movement_re = Regex::new("\x1b\\[[0-9;]*[ABCD]").unwrap();
            temp_result = clean_text(&opt, &movement_re, &temp_result);
        }
        temp_result
    };

    if opt.in_place {
        if let Some(file_name) = &opt.file {
            if opt.backup {
                fs::copy(file_name, format!("{}.bak", file_name))?;
            }
            fs::write(file_name, &result)?;
        }
    } else {
        io::stdout().write_all(result.as_bytes())?;
    }

    Ok(())
}

fn clean_text(opts: &Opts, re: &Regex, text: &str) -> String {
    re.replace_all(text, |caps: &regex::Captures| {
        if opts.verbose && !opts.quiet {
            println!("Removing ANSI sequence: {}", &caps[0]);
        }
        ""
    }).to_string()
}
