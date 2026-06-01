# qrcc

Générateur de **QR code en ligne de commande** écrit en Rust. Il prend une URL
(ou n'importe quel texte) en entrée et affiche le QR code directement dans le
terminal grâce aux demi-blocs Unicode.

```
$ qrcc "https://example.com"
```

## Installation

Prérequis : [Rust](https://rustup.rs/) (édition 2024).

```sh
# Compiler en mode optimisé
cargo build --release

# Le binaire est alors disponible :
./target/release/qrcc "https://example.com"

# (Optionnel) installer dans ~/.cargo/bin pour l'avoir dans le PATH
cargo install --path .
```

Pendant le développement, on peut lancer directement avec `cargo run` :

```sh
cargo run -- "https://example.com"
```

## Utilisation

```
qrcc [OPTIONS] [CONTENT]
```

| Élément        | Description                                                            |
|----------------|------------------------------------------------------------------------|
| `CONTENT`      | Contenu à encoder. Si absent, lu depuis l'entrée standard (stdin).     |
| `--ecc <L\|M\|Q\|H>` | Niveau de correction d'erreur (défaut : `M`).                    |
| `--margin <N>` | Taille de la zone calme autour du QR, en modules (défaut : `2`).       |
| `--invert`     | Inverse sombre/clair (utile sur un terminal à fond clair).             |
| `-h, --help`   | Affiche l'aide.                                                        |
| `-V, --version`| Affiche la version.                                                    |

### Exemples

```sh
# Encoder une URL
qrcc "https://example.com"

# Lire le contenu depuis stdin
echo "texte via stdin" | qrcc

# Correction d'erreur maximale + marge plus large + couleurs inversées
qrcc "https://example.com" --ecc H --margin 4 --invert
```

## Architecture

```
src/
├── main.rs    # binaire : parse les args et imprime le résultat
├── lib.rs     # logique (run) + types d'erreur, testable
├── cli.rs     # définition des arguments (clap)
└── render.rs  # matrice QR → demi-blocs Unicode pour le terminal
tests/
└── integration.rs  # tests bout-en-bout
```

La logique vit dans la bibliothèque (`lib.rs`) ; le binaire (`main.rs`) n'est
qu'une fine couche d'entrée. Cette séparation rend la logique testable sans
lancer le processus CLI.

## Tests & qualité

```sh
cargo test      # tests d'intégration
cargo clippy    # lints
cargo fmt       # formatage
```

## Pistes d'évolution

- Export vers fichier PNG / SVG.
- Contenus typés (WiFi, vCard, email, SMS) avec des sous-commandes.
- Sortie ASCII pure pour les terminaux sans support Unicode.

## Licence

MIT — voir [LICENSE](LICENSE).
