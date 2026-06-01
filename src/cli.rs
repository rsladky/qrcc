//! Définition des arguments de la ligne de commande.
//!
//! On utilise `clap` en mode « derive » : on décrit les arguments avec une
//! structure annotée, et `clap` génère automatiquement le parsing, l'aide
//! (`--help`) et la validation.

use clap::{Parser, ValueEnum};
use qrcode::EcLevel;

/// Génère un QR code et l'affiche dans le terminal.
#[derive(Parser, Debug)]
#[command(name = "qrcc", version, about, long_about = None)]
pub struct Args {
    /// Contenu à encoder (URL, texte, …).
    ///
    /// Optionnel : si absent, le contenu est lu depuis l'entrée standard
    /// (utile pour `echo "..." | qrcc`).
    pub content: Option<String>,

    /// Niveau de correction d'erreur du QR code.
    ///
    /// Plus le niveau est élevé, plus le QR reste lisible s'il est abîmé,
    /// mais plus il devient dense.
    #[arg(long, value_enum, default_value_t = Ecc::M)]
    pub ecc: Ecc,

    /// Taille de la zone calme (marge claire) autour du QR, en modules.
    ///
    /// Une marge est nécessaire pour que la plupart des scanners reconnaissent
    /// le code (4 est la valeur recommandée par la spec, 2 suffit souvent).
    #[arg(long, default_value_t = 2)]
    pub margin: usize,

    /// Inverse les couleurs (sombre ↔ clair).
    ///
    /// Par défaut on dessine les modules « sombres » comme pleins, ce qui
    /// convient à un terminal au texte clair sur fond sombre. Sur un terminal
    /// clair, `--invert` peut améliorer le rendu/scan.
    #[arg(long)]
    pub invert: bool,
}

/// Niveaux de correction d'erreur exposés sur la CLI.
///
/// On définit notre propre enum (plutôt que d'exposer directement celui de
/// `qrcode`) pour pouvoir dériver `ValueEnum` et contrôler les valeurs CLI.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Ecc {
    /// ~7 % de récupération.
    L,
    /// ~15 % de récupération (défaut).
    M,
    /// ~25 % de récupération.
    Q,
    /// ~30 % de récupération.
    H,
}

impl From<Ecc> for EcLevel {
    fn from(ecc: Ecc) -> Self {
        match ecc {
            Ecc::L => EcLevel::L,
            Ecc::M => EcLevel::M,
            Ecc::Q => EcLevel::Q,
            Ecc::H => EcLevel::H,
        }
    }
}
