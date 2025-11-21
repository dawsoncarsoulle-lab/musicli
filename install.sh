#!/bin/bash

set -e

echo "🎵 Installation de MusicLI"
echo "=========================="
echo ""

echo "1️⃣  Vérification des dépendances système..."
if ! command -v pkg-config &> /dev/null; then
    echo "   Installation de pkg-config..."
    sudo apt-get update
    sudo apt-get install -y pkg-config
fi

if ! dpkg -l | grep -q libasound2-dev; then
    echo "   Installation de libasound2-dev..."
    sudo apt-get update
    sudo apt-get install -y libasound2-dev
fi

echo "   ✓ Dépendances système OK"
echo ""

echo "2️⃣  Compilation en mode release..."
cargo build --release
echo "   ✓ Compilation réussie"
echo ""

echo "3️⃣  Installation de l'exécutable..."
mkdir -p ~/.local/bin
cp target/release/musicli ~/.local/bin/
chmod +x ~/.local/bin/musicli

if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
    echo "   ✓ ~/.local/bin est déjà dans le PATH"
else
    echo "   ⚠️  Ajout de ~/.local/bin au PATH..."
    if [[ -f ~/.bashrc ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    fi
    if [[ -f ~/.zshrc ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    fi
    echo "   Veuillez recharger votre shell : source ~/.bashrc ou source ~/.zshrc"
fi

echo ""
echo "✅ Installation terminée!"
echo ""
echo "Utilisez : musicli"
