# 🎵 MusicLI - Guide des arguments CLI

MusicLI supporte plusieurs commandes et options pour une utilisation flexible en ligne de commande.

## 📋 Commandes disponibles

### 1. Menu interactif (par défaut)

Lance le menu interactif avec recherche floue pour sélectionner et écouter une musique.

```bash
musicli
```

**Comportement** :
- Scanne le dossier `~/Musique`
- Affiche un menu interactif
- Permet de chercher une chanson
- Lance la lecture avec un spinner animé

---

### 2. Afficher la version

Affiche la version du programme.

```bash
musicli --version
# ou
musicli -V
# ou
musicli version
```

**Résultat** :
```
🎵 Bienvenue dans MusicLI 🎵

musicli 0.1.0
```

---

### 3. Télécharger une musique

Télécharge une musique depuis YouTube ou une autre source compatible avec `yt-dlp`.

#### Syntaxe 1 : Subcommande

```bash
musicli download "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

#### Syntaxe 2 : Flag

```bash
musicli --download "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
# ou
musicli -d "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

**Prérequis** : `yt-dlp` doit être installé

```bash
sudo apt-get install yt-dlp
```

**Comportement** :
- Télécharge la musique au format MP3
- Enregistre dans `~/Musique`
- Affiche la progression
- Gère les erreurs si `yt-dlp` n'est pas installé

**Sources supportées** :
- YouTube
- SoundCloud
- Spotify (avec limitations)
- Bandcamp
- Et 1000+ autres sources

---

## 🎯 Exemples d'utilisation

### Exemple 1 : Écouter une musique

```bash
musicli
```

Puis sélectionnez une chanson dans le menu.

### Exemple 2 : Vérifier la version

```bash
musicli --version
# Résultat : musicli 0.1.0
```

### Exemple 3 : Télécharger une musique depuis YouTube

```bash
musicli download "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

La musique sera téléchargée en MP3 et enregistrée dans `~/Musique/`.

### Exemple 4 : Utiliser le flag --download

```bash
musicli -d "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

Équivalent à l'exemple 3.

### Exemple 5 : Afficher l'aide

```bash
musicli --help
# ou
musicli -h
```

---

## 📖 Aide complète

Affiche toutes les commandes et options disponibles.

```bash
musicli --help
```

**Résultat** :
```
Lecteur audio interactif avec support du téléchargement

Usage: musicli [OPTIONS] [COMMAND]

Commands:
  version   Affiche la version du programme
  download  Télécharge une musique depuis une URL
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --download <DOWNLOAD>  Télécharger une musique depuis une URL (yt-dlp)
  -h, --help                 Print help
  -V, --version              Print version
```

---

## 🔧 Aide pour une commande spécifique

```bash
musicli download --help
```

**Résultat** :
```
Télécharge une musique depuis une URL

Usage: musicli download <URL>

Arguments:
  <URL>  URL de la vidéo/musique à télécharger

Options:
  -h, --help  Print help
```

---

## ⚙️ Configuration de yt-dlp

### Installation

```bash
# Ubuntu/Debian
sudo apt-get install yt-dlp

# macOS
brew install yt-dlp

# Avec pip
pip install yt-dlp
```

### Vérifier l'installation

```bash
yt-dlp --version
```

### Mettre à jour

```bash
sudo apt-get update && sudo apt-get upgrade yt-dlp
# ou
pip install --upgrade yt-dlp
```

---

## 🐛 Troubleshooting

### Erreur : "yt-dlp n'est pas installé"

**Cause** : `yt-dlp` n'est pas dans le PATH

**Solution** :
```bash
sudo apt-get install yt-dlp
```

### Erreur : "Erreur lors du téléchargement"

**Cause** : L'URL n'est pas valide ou le site n'est pas supporté

**Solution** :
1. Vérifiez que l'URL est correcte
2. Vérifiez que le site est supporté par `yt-dlp`
3. Mettez à jour `yt-dlp` : `pip install --upgrade yt-dlp`

### Le téléchargement est très lent

**Cause** : Connexion internet lente ou serveur surchargé

**Solution** :
- Attendez ou réessayez plus tard
- Vérifiez votre connexion internet

### La musique téléchargée n'apparaît pas dans le menu

**Cause** : Le dossier `~/Musique` n'existe pas ou le fichier n'a pas été créé

**Solution** :
```bash
mkdir -p ~/Musique
ls -la ~/Musique
```

---

## 📊 Comparaison des syntaxes

| Objectif | Commande |
|----------|----------|
| Menu interactif | `musicli` |
| Afficher la version | `musicli --version` ou `musicli -V` |
| Télécharger (subcommande) | `musicli download "URL"` |
| Télécharger (flag long) | `musicli --download "URL"` |
| Télécharger (flag court) | `musicli -d "URL"` |
| Afficher l'aide | `musicli --help` ou `musicli -h` |

---

## 🔗 Ressources

- [yt-dlp Documentation](https://github.com/yt-dlp/yt-dlp)
- [Clap Documentation](https://docs.rs/clap/)
- [Supported Sites](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)

---

## ✨ Résumé

```bash
# Menu interactif
musicli

# Afficher la version
musicli --version

# Télécharger une musique
musicli download "https://..."
musicli --download "https://..."
musicli -d "https://..."

# Afficher l'aide
musicli --help
```

Prêt à utiliser ! 🎵
