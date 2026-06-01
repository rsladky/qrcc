//! Bibliothèque `qrcc` : la logique du générateur, indépendante du binaire.
//!
//! Séparer la logique dans une lib permet de la tester directement (voir
//! `tests/integration.rs`) sans lancer le processus CLI.

pub mod cli;
pub mod render;

use std::io::{self, Read};

use cli::Args;
use qrcode::QrCode;

/// Erreurs possibles lors de la génération.
#[derive(Debug)]
pub enum Error {
    /// Aucun contenu fourni (ni argument, ni entrée standard).
    EmptyInput,
    /// Échec de lecture de l'entrée standard.
    Io(io::Error),
    /// Le contenu n'a pas pu être encodé (ex. trop long pour un QR code).
    Encode(qrcode::types::QrError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyInput => write!(
                f,
                "aucun contenu à encoder (passez un argument ou redirigez du texte via stdin)"
            ),
            Error::Io(e) => write!(f, "erreur de lecture de l'entrée standard : {e}"),
            Error::Encode(e) => write!(f, "impossible d'encoder le contenu : {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<qrcode::types::QrError> for Error {
    fn from(e: qrcode::types::QrError) -> Self {
        Error::Encode(e)
    }
}

/// Détermine le contenu à encoder : l'argument s'il est présent, sinon stdin.
fn resolve_content(content: Option<String>) -> Result<String, Error> {
    let raw = match content {
        Some(s) => s,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(Error::EmptyInput);
    }
    Ok(trimmed.to_string())
}

/// Point d'entrée de la logique : transforme les arguments en QR code rendu.
///
/// Renvoie la chaîne à imprimer (sans `println!` ici, pour rester testable).
pub fn run(args: Args) -> Result<String, Error> {
    let content = resolve_content(args.content)?;
    let code = QrCode::with_error_correction_level(content.as_bytes(), args.ecc.into())?;
    Ok(render::render(&code, args.margin, args.invert))
}
