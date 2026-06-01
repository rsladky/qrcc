//! Rendu d'une matrice QR vers une chaîne affichable dans le terminal.
//!
//! Astuce de rendu : un module QR est carré, mais une cellule de terminal est
//! environ deux fois plus haute que large. Pour obtenir un rendu proche du
//! carré, on encode **deux modules verticaux par caractère** à l'aide des
//! demi-blocs Unicode :
//!
//! | haut   | bas    | glyphe |
//! |--------|--------|--------|
//! | sombre | sombre | `█`    |
//! | sombre | clair  | `▀`    |
//! | clair  | sombre | `▄`    |
//! | clair  | clair  | ` `    |

use qrcode::{Color, QrCode};

const FULL: char = '█'; // U+2588 — bloc plein (haut + bas sombres)
const UPPER: char = '▀'; // U+2580 — demi-bloc supérieur
const LOWER: char = '▄'; // U+2584 — demi-bloc inférieur
const EMPTY: char = ' '; // espace (haut + bas clairs)

/// Convertit un `QrCode` en chaîne prête à être imprimée dans le terminal.
///
/// - `margin` : nombre de modules clairs ajoutés sur les quatre côtés (zone calme).
/// - `invert` : échange le rôle des modules sombres et clairs.
pub fn render(code: &QrCode, margin: usize, invert: bool) -> String {
    let width = code.width();
    let colors = code.to_colors();

    // On copie la matrice dans une grille agrandie qui inclut la zone calme,
    // afin de gérer les bords uniformément (pas de cas particuliers).
    let padded = width + 2 * margin;
    let is_dark = |row: usize, col: usize| -> bool {
        // En dehors de la matrice d'origine → module clair (la marge).
        if row < margin || col < margin || row >= margin + width || col >= margin + width {
            return invert; // une marge « sombre » si on inverse
        }
        let dark = colors[(row - margin) * width + (col - margin)] == Color::Dark;
        dark ^ invert
    };

    // On parcourt les lignes deux par deux pour fusionner chaque paire en une
    // seule ligne de caractères. `step_by(2)` garantit qu'on traite (0,1),
    // (2,3), … même si la hauteur est impaire (la dernière ligne « bas » est
    // alors considérée comme claire).
    let mut out = String::with_capacity(padded * (padded / 2 + 1));
    for top in (0..padded).step_by(2) {
        for col in 0..padded {
            let upper = is_dark(top, col);
            let lower = top + 1 < padded && is_dark(top + 1, col);
            out.push(match (upper, lower) {
                (true, true) => FULL,
                (true, false) => UPPER,
                (false, true) => LOWER,
                (false, false) => EMPTY,
            });
        }
        out.push('\n');
    }
    out
}
