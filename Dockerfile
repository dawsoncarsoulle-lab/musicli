FROM rust:slim-bookworm

# 1. Installer les dépendances nécessaires
# - mingw-w64 : Le compilateur pour créer des .exe Windows depuis Linux
# - libasound2-dev : Nécessaire pour compiler la partie audio de Linux
# - pkg-config : Aide à trouver les librairies
RUN apt-get update && apt-get install -y \
    mingw-w64 \
    libasound2-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 2. Ajouter la cible de compilation Windows à Rust
RUN rustup target add x86_64-pc-windows-gnu

# 3. Configurer Cargo pour qu'il sache quel "linker" utiliser pour Windows
# (C'est l'astuce pour que ça marche sans fichier de config complexe)
ENV CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
ENV CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-C link-args=-no-pie"

# 4. Dossier de travail
WORKDIR /app
