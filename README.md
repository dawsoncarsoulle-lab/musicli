# MusicLI - Lecteur Audio CLI en Rust

Une application en ligne de commande robuste et cross-platform pour écouter de la musique, avec une interface interactive, un feedback visuel élégant et support du téléchargement.

**Fonctionne sur** : Ubuntu, Debian, Windows, macOS

## Stack Technique

- **inquire** (v0.6) - Menus interactifs avec recherche floue
- **indicatif** (v0.17) - Barres de progression et spinners animés
- **colored** (v2.0) - Styling du texte en couleur
- **rodio** (v0.17) - Moteur audio cross-platform
- **walkdir** (v2) - Scan récursif des dossiers
- **tokio** (v1) - Runtime asynchrone
- **clap** (v4.4) - Parsing des arguments CLI avec derive
- **dirs** (v5.0) - Détection cross-platform des dossiers utilisateur

## Prérequis

### Linux (Ubuntu/Debian)

Rodio nécessite les librairies ALSA pour la lecture audio :

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev pkg-config
```

### Windows

Aucune dépendance système requise. Assurez-vous d'avoir :
- Windows 10 ou supérieur
- yt-dlp installé (optionnel, pour le téléchargement)

### macOS

```bash
brew install pkg-config
```

### Rust (tous les OS)

Assurez-vous que Rust est installé :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### yt-dlp (optionnel, pour le téléchargement)

**Linux/macOS** :
```bash
sudo apt-get install yt-dlp  # ou brew install yt-dlp
```

**Windows** :
```bash
choco install yt-dlp  # ou pip install yt-dlp
```

## Installation

### Option 1 : Installation via cargo install

```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo install --path .
```

L'exécutable `musicli` sera automatiquement ajouté à `~/.cargo/bin`, qui est généralement dans votre PATH.

Vérifiez :

```bash
which musicli
musicli
```

### Option 2 : Copie manuelle de l'exécutable

```bash
cp target/release/musicli ~/.local/bin/
chmod +x ~/.local/bin/musicli
```

Assurez-vous que `~/.local/bin` est dans votre PATH. Sinon, ajoutez-le à `~/.bashrc` ou `~/.zshrc` :

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Puis rechargez votre shell :

```bash
source ~/.bashrc  # ou source ~/.zshrc
```

### Option 3 : Compilation via Docker (Linux & Windows)

Pour compiler facilement pour Linux et Windows sans dépendances système complexes :

```bash
# Construire l'image Docker
docker build -t musicli-builder .

# Compiler pour Linux
docker run --rm -v $(pwd):/app musicli-builder cargo build --release

# Compiler pour Windows
docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu
```

Voir [DOCKER.md](DOCKER.md) pour les instructions détaillées.

## Utilisation

### Lancer l'application (menu interactif)

```bash
musicli
```

L'application détecte automatiquement votre dossier de musique :
- **Windows** : `C:\Users\YourName\Music`
- **macOS** : `~/Music`
- **Linux** : `~/Music` ou `~/Musique`

Le chemin scanné s'affiche au démarrage.

### Arguments CLI

#### Afficher la version

```bash
musicli --version
musicli -V
musicli version
```

#### Télécharger une musique

```bash
musicli download "https://www.youtube.com/watch?v=..."
musicli --download "https://www.youtube.com/watch?v=..."
musicli -d "https://www.youtube.com/watch?v=..."
```

**Fonctionnalités du téléchargement** :
- ✅ Spinner animé pendant le téléchargement
- ✅ Conversion automatique en MP3
- ✅ Enregistrement dans le dossier Musique détecté
- ✅ Gestion cross-platform (Windows, Linux, macOS)
- ✅ Gestion des erreurs complète

### Fonctionnalités

1. **Détection cross-platform** - Trouve automatiquement le dossier Musique de votre OS
2. **Scan automatique** - Scanne récursivement le dossier Musique
3. **Recherche floue** - Tapez le nom de la chanson pour la trouver rapidement
4. **Navigation** - Utilisez les flèches pour naviguer, Entrée pour sélectionner
5. **Feedback visuel** - Spinner animé et texte coloré
6. **Formats supportés** - MP3, WAV, FLAC, OGG
7. **Téléchargement** - Téléchargez des musiques depuis YouTube avec animation
8. **Cross-platform** - Fonctionne sur Windows, Linux et macOS

## Structure du Projet

```
musique/
├── Cargo.toml              # Configuration du projet
├── Dockerfile              # Configuration Docker pour cross-compilation
├── src/
│   ├── main.rs             # Point d'entrée et orchestration (CLI avec clap)
│   ├── file_manager.rs     # Gestion des fichiers audio
│   ├── ui.rs               # Interface utilisateur
│   ├── audio_engine.rs     # Moteur de lecture audio
│   └── downloader.rs       # Téléchargement de musiques (yt-dlp)
├── target/
│   ├── debug/              # Build de développement
│   └── release/            # Build optimisé
├── README.md               # Ce fichier
├── DOCKER.md               # Guide Docker
└── [autres fichiers de doc]
```

## Compilation

### Mode développement

```bash
cargo build
./target/debug/musicli
```

### Mode release (optimisé)

```bash
cargo build --release
./target/release/musicli
```

## Architecture

### Modules

- **file_manager.rs** - Structure `Track` et fonction `scan_music_folder()`
- **ui.rs** - Interface avec inquire, affichage coloré
- **audio_engine.rs** - Classe `AudioPlayer` avec spinner animé
- **main.rs** - Orchestration et gestion d'erreurs

## Développement futur

- Implémentation de `afficher_paroles()` pour afficher les paroles
- Playlist et historique
- Contrôles de lecture (pause, skip)
- Gestion du volume
- Thèmes personnalisables

## Troubleshooting

### Erreur : "Aucun périphérique audio disponible"

Vérifiez que ALSA est correctement installé :

```bash
aplay -l
```

### Erreur : "Aucune piste audio trouvée"

Vérifiez que le dossier `~/Musique` existe ou placez des fichiers audio dans le répertoire courant.

### Erreur de compilation

Assurez-vous que les dépendances système sont installées :

```bash
sudo apt-get install -y libasound2-dev pkg-config
```

## Licence

MIT
