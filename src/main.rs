//! Binaire `qrcc` : fine couche autour de la bibliothèque.
//!
//! Rôle : parser les arguments, appeler `qrcc::run`, imprimer le résultat ou
//! signaler l'erreur sur stderr avec un code de sortie non nul.

use std::process::ExitCode;

use clap::Parser;
use qrcc::cli::Args;

fn main() -> ExitCode {
    let args = Args::parse();
    match qrcc::run(args) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("qrcc : {err}");
            ExitCode::FAILURE
        }
    }
}
