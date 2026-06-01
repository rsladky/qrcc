# --- Stage build ---
# L'image rust:alpine a pour hôte la cible musl native (x86_64 ou aarch64
# selon la plateforme) qui lie statiquement par défaut : pas besoin de
# spécifier --target, le binaire produit est autonome et multi-arch.
FROM rust:1.88-alpine AS builder

# musl-dev fournit les en-têtes nécessaires au link statique
RUN apk add --no-cache musl-dev

WORKDIR /app

# 1) Mettre en cache la compilation des dépendances :
#    copier d'abord les manifestes, créer un main.rs/lib.rs factices, builder.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
 && echo 'fn main() {}' > src/main.rs \
 && echo '' > src/lib.rs \
 && cargo build --release \
 && rm -rf src

# 2) Copier le vrai code et builder le binaire final.
COPY src ./src
COPY tests ./tests
# `touch` force la recompilation du crate (mtime des manifestes inchangé)
RUN touch src/main.rs src/lib.rs \
 && cargo build --release

# --- Stage runtime ---
FROM scratch
COPY --from=builder /app/target/release/qrcc /qrcc
ENTRYPOINT ["/qrcc"]
