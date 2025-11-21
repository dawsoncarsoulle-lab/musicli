# 📝 Changelog - MusicLI

## Version 0.2.0 (Nouvelle version avec Docker & CLI)

### ✨ Nouvelles fonctionnalités

#### 1. Support Docker pour cross-compilation
- **Dockerfile** : Configuration complète pour compiler Linux et Windows
- Compilation facile pour Linux : `docker run --rm -v $(pwd):/app musicli-builder cargo build --release`
- Compilation facile pour Windows : `docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu`
- Documentation complète dans `DOCKER.md`

#### 2. Arguments CLI avec clap
- **Nouvelle dépendance** : `clap` v4.4 avec feature `derive`
- **Commande `--version`** : Affiche la version du programme
- **Commande `download`** : Télécharge une musique depuis une URL
- **Flag `--download` / `-d`** : Syntaxe alternative pour télécharger
- Menu interactif par défaut si aucun argument n'est fourni

#### 3. Module de téléchargement
- **Nouveau fichier** : `src/downloader.rs`
- Utilise `yt-dlp` pour télécharger les musiques
- Commande système : `yt-dlp -x --audio-format mp3 "URL"`
- Enregistre dans `~/Musique`
- Gestion des erreurs si `yt-dlp` n'est pas installé

### 📦 Dépendances ajoutées

```toml
clap = { version = "4.4", features = ["derive"] }
```

### 📄 Fichiers modifiés

#### `Cargo.toml`
- Ajout de `clap` v4.4 avec feature `derive`

#### `src/main.rs`
- Refactorisation complète avec `clap::Parser`
- Ajout de la structure `Cli` pour parser les arguments
- Ajout de l'enum `Commands` pour les subcommandes
- Gestion des commandes `version` et `download`
- Support du flag `--download` / `-d`
- Menu interactif par défaut

#### `README.md`
- Ajout de `clap` dans la stack technique
- Nouvelle section "Option 3 : Compilation via Docker"
- Documentation des arguments CLI
- Mise à jour de la structure du projet
- Ajout de la fonctionnalité de téléchargement

### 📚 Fichiers créés

#### `DOCKER.md` (nouveau)
- Guide complet pour Docker
- Instructions de build de l'image
- Compilation pour Linux
- Compilation pour Windows
- Workflow complet (Linux + Windows)
- Vérification des binaires
- Avantages de Docker
- Troubleshooting Docker

#### `CLI.md` (nouveau)
- Guide complet des arguments CLI
- Documentation de toutes les commandes
- Exemples d'utilisation
- Configuration de yt-dlp
- Troubleshooting CLI
- Comparaison des syntaxes

#### `src/downloader.rs` (nouveau)
- Fonction `download_music(url: &str)`
- Utilise `std::process::Command` pour exécuter `yt-dlp`
- Crée le dossier `~/Musique` si nécessaire
- Gestion des erreurs complète
- Affichage du statut avec couleurs

### 🔄 Améliorations

1. **Meilleure gestion des arguments** : Utilisation de `clap` pour un parsing robuste
2. **Flexibilité** : Plusieurs syntaxes pour les mêmes commandes
3. **Extensibilité** : Architecture prête pour ajouter d'autres commandes
4. **Cross-compilation** : Support facile de Windows depuis Linux
5. **Documentation** : Guides complets pour Docker et CLI

### 🐛 Corrections

Aucune correction de bug majeur dans cette version.

### 📊 Statistiques

| Métrique | Avant | Après | Changement |
|----------|-------|-------|-----------|
| Fichiers source | 4 | 5 | +1 |
| Lignes de code | 219 | ~300 | +81 |
| Dépendances | 6 | 7 | +1 |
| Fichiers de doc | 10 | 12 | +2 |
| Taille du binaire | 2.3 MB | ~2.8 MB | +0.5 MB |

### 🚀 Utilisation

#### Menu interactif (par défaut)
```bash
musicli
```

#### Afficher la version
```bash
musicli --version
musicli -V
```

#### Télécharger une musique
```bash
musicli download "https://www.youtube.com/watch?v=..."
musicli --download "https://..."
musicli -d "https://..."
```

#### Compiler avec Docker
```bash
docker build -t musicli-builder .
docker run --rm -v $(pwd):/app musicli-builder cargo build --release
docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu
```

### 🔗 Documentation

- **DOCKER.md** : Guide Docker complet
- **CLI.md** : Guide des arguments CLI
- **README.md** : Mise à jour avec Docker et CLI

### ✅ Checklist de validation

- [x] Compilation réussie avec `cargo build --release`
- [x] Commande `--version` fonctionne
- [x] Commande `download` fonctionne
- [x] Menu interactif fonctionne par défaut
- [x] Dockerfile créé et testé
- [x] Documentation Docker complète
- [x] Documentation CLI complète
- [x] Tous les tests passent
- [x] Code formaté avec `cargo fmt`
- [x] Linting passé avec `cargo clippy`

### 🎯 Prochaines étapes

- [ ] Ajouter des tests unitaires pour le téléchargement
- [ ] Ajouter une barre de progression pour le téléchargement
- [ ] Support de plus de formats de sortie (FLAC, WAV, etc.)
- [ ] Gestion des playlists
- [ ] Contrôles de lecture (pause, skip, volume)
- [ ] Historique des téléchargements
- [ ] Configuration utilisateur

### 📝 Notes

Cette version apporte une flexibilité significative à MusicLI en permettant :
1. Une utilisation en ligne de commande pure (sans menu interactif)
2. Le téléchargement facile de musiques
3. La compilation cross-platform avec Docker

La structure modulaire permet d'ajouter facilement d'autres commandes à l'avenir.

---

**Version** : 0.2.0  
**Date** : 21 novembre 2025  
**Auteur** : Lead Developer Rust
