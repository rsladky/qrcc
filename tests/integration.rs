//! Tests d'intégration de la logique `qrcc::run`.
//!
//! On appelle directement la bibliothèque (pas le binaire) en construisant des
//! `Args` à la main. Le champ `content` est `Some(...)` pour éviter de lire
//! stdin pendant les tests.

use qrcc::Error;
use qrcc::cli::{Args, Ecc};

/// Construit des `Args` avec un contenu donné et des valeurs par défaut.
fn args_with(content: &str, margin: usize) -> Args {
    Args {
        content: Some(content.to_string()),
        ecc: Ecc::M,
        margin,
        invert: false,
    }
}

#[test]
fn encode_url_produces_blocks() {
    let out = qrcc::run(args_with("https://example.com", 2)).expect("doit réussir");
    assert!(!out.is_empty());
    // Le rendu doit contenir au moins un module sombre (demi-bloc ou bloc plein).
    assert!(out.contains('█') || out.contains('▀') || out.contains('▄'));
}

#[test]
fn larger_margin_produces_larger_output() {
    let small = qrcc::run(args_with("hello", 1)).unwrap();
    let large = qrcc::run(args_with("hello", 6)).unwrap();
    assert!(
        large.len() > small.len(),
        "une marge plus grande doit allonger la sortie"
    );
}

#[test]
fn empty_content_is_an_error() {
    let err = qrcc::run(args_with("   ", 2)).unwrap_err();
    assert!(matches!(err, Error::EmptyInput));
}

#[test]
fn overly_long_content_is_an_error() {
    // Un QR code a une capacité maximale ; un contenu énorme doit échouer
    // proprement plutôt que paniquer.
    let huge = "A".repeat(10_000);
    let err = qrcc::run(args_with(&huge, 2)).unwrap_err();
    assert!(matches!(err, Error::Encode(_)));
}
