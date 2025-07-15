mod builtins;

use std::fs::read_to_string;

use snix_eval::{
    EvalMode,
    Evaluation,
};

fn main() {
    let code = read_to_string("tests.nix").expect("Failed to read tests.nix");

    let evaluation = Evaluation::builder_impure()
        .add_builtins(builtins::builtins())
        .mode(EvalMode::Strict)
        .disable_import()
        .build();

    let eval = evaluation.evaluate(code, None);

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
