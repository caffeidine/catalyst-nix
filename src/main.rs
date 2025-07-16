mod builtins;

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use snix_eval::{
    EvalMode,
    Evaluation,
};

/// Lightweight API testing tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the test file or directory
    ///
    /// If a directory is provided, all files with the .nix extension will be
    /// tested.
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let path = args.path;

    if !path.is_dir() && !path.is_file() {
        println!("Invalid path: {}", path.display());
        return;
    }

    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.extension() == Some(OsStr::new("nix")) {
                println!("Testing file: {}", path.display());
                test_file(path);
                println!()
            }
        }
    } else {
        test_file(path);
    }
}

fn test_file(path: PathBuf) {
    let code = fs::read_to_string(&path).expect("Failed to read tests.nix");

    let evaluation = Evaluation::builder_impure()
        .add_builtins(builtins::builtins())
        .mode(EvalMode::Strict)
        .disable_import()
        .build();

    let eval = evaluation.evaluate(code, Some(path));

    if !eval.errors.is_empty() {
        for error in eval.errors {
            println!("Error: {error:?}");
        }
        return;
    }

    if !eval.warnings.is_empty() {
        for warning in eval.warnings {
            println!("Warning: {warning:?}");
        }
    }
}
