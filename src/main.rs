#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap[parse(from_os_str)]]
    path: std::path::PathBuf
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    
    grrs::find_matches(&content, &args.pattern, handle);
    
    Ok(())
}
