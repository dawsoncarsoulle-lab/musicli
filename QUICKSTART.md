# Démarrage rapide - MusicLI

## ⚡ En 5 minutes

### 1. Installer les dépendances système (1 min)

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev pkg-config
```

### 2. Compiler (2 min)

```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo build --release
```

### 3. Installer (1 min)

```bash
mkdir -p ~/.local/bin
cp target/release/musicli ~/.local/bin/
chmod +x ~/.local/bin/musicli
```

### 4. Configurer le PATH (1 min)

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 5. Utiliser

```bash
musicli
```

---

## 🚀 Installation automatique (recommandée)

```bash
cd /home/dawson/Documents/Projet/Rust/musique
bash install.sh
musicli
```

---

## 📖 Documentation

- **README.md** - Guide utilisateur complet
- **INSTALLATION.md** - Instructions détaillées
- **ARCHITECTURE.md** - Architecture technique
- **DEVELOPMENT.md** - Guide de développement
- **SUMMARY.md** - Résumé complet

---

## 🎵 Utilisation

### Lancer l'application

```bash
musicli
```

### Sélectionner une musique

1. Tapez pour chercher
2. Utilisez ↑/↓ pour naviguer
3. Appuyez sur Entrée pour sélectionner

### Formats supportés

- MP3
- WAV
- FLAC
- OGG

---

## 🐛 Troubleshooting

### Erreur : "Aucun périphérique audio"

```bash
aplay -l
```

### Erreur : "Aucune piste trouvée"

```bash
mkdir -p ~/Musique
# Placez des fichiers audio dans ~/Musique
```

### Erreur : "command not found: musicli"

```bash
source ~/.bashrc
which musicli
```

---

## 📦 Contenu du projet

```
src/
├── main.rs              # Point d'entrée
├── file_manager.rs      # Gestion des fichiers
├── ui.rs                # Interface utilisateur
└── audio_engine.rs      # Moteur audio

Documentation/
├── README.md
├── INSTALLATION.md
├── ARCHITECTURE.md
├── DEVELOPMENT.md
└── SUMMARY.md
```

---

## ✨ Caractéristiques

✅ Scan récursif des dossiers  
✅ Recherche floue  
✅ Lecture audio fluide  
✅ Interface colorée  
✅ Spinner animé  
✅ Gestion d'erreurs robuste  

---

## 🔧 Commandes utiles

```bash
cargo build                    # Build de développement
cargo build --release          # Build optimisé
cargo run                      # Exécuter
cargo test                     # Tests
cargo clippy                   # Linting
cargo fmt                      # Formatage
```

---

## 📞 Besoin d'aide ?

1. Consultez **README.md**
2. Consultez **INSTALLATION.md**
3. Consultez **DEVELOPMENT.md**

---

**Prêt à utiliser** : `musicli`

Bon écoute ! 🎶
