#![allow(clippy::missing_errors_doc)]

use std::path::PathBuf;

use clap::Parser;

use crate::astgen::{parser::parse_code, traverser::Traverser};

pub mod astgen;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
    #[arg(short, long, default_value = "false")]
    test_all: bool,
}

fn lint_code(code: String) -> anyhow::Result<()> {
    let document = parse_code(code)?;

    println!("document: {document:#?}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut files = vec![];

    if cli.test_all {
        for f in std::fs::read_dir("tests").unwrap() {
            files.push(f.unwrap().path());
        }
    } else {
        files.push(cli.path);
    }

    for file in files {
        // println!("Processing {}", file.display());

        let code = std::fs::read_to_string(file)?;

        let stack_size = 32 * 1024 * 1024;
        let builder = std::thread::Builder::new().stack_size(stack_size);
        let handler = builder.spawn(move || -> anyhow::Result<()> {
            lint_code(code)?;
            Ok(())
        })?;
        handler.join().unwrap()?;
    }

    Ok(())
}
