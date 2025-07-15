mod builtins;

use std::fs::read_to_string;
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
    /// Path to the test file
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let code = read_to_string(&args.file).expect("Failed to read tests.nix");

    let evaluation = Evaluation::builder_impure()
        .add_builtins(builtins::builtins())
        .mode(EvalMode::Strict)
        .disable_import()
        .build();

    let eval = evaluation.evaluate(code, Some(args.file));

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
