# 🐳 Installation & Compilation via Docker

MusicLI peut être compilé facilement pour Linux et Windows en utilisant Docker, sans avoir besoin d'installer les dépendances système complexes sur votre machine.

## 📋 Prérequis

- Docker installé et en cours d'exécution
- Le Dockerfile à la racine du projet

## 🏗️ Étape 1 : Construire l'image Docker

Construisez l'image Docker "usine" qui contient tous les outils nécessaires pour compiler MusicLI :

```bash
cd /home/dawson/Documents/Projet/Rust/musique
docker build -t musicli-builder .
```

**Explication** :
- `docker build` : Construit une nouvelle image Docker
- `-t musicli-builder` : Nomme l'image "musicli-builder"
- `.` : Utilise le Dockerfile du répertoire courant

**Résultat** : Une image Docker contenant :
- Rust et Cargo
- MinGW-W64 (compilateur Windows)
- ALSA (pour la compilation Linux)
- La cible de compilation `x86_64-pc-windows-gnu`

---

## 🐧 Étape 2 : Compiler pour Linux

Compilez un binaire Linux optimisé en utilisant Docker :

```bash
docker run --rm -v $(pwd):/app musicli-builder cargo build --release
```

**Explication** :
- `docker run` : Exécute un conteneur
- `--rm` : Supprime le conteneur après l'exécution
- `-v $(pwd):/app` : Monte le répertoire courant dans `/app` du conteneur
- `musicli-builder` : Utilise l'image que nous avons construite
- `cargo build --release` : Compile en mode optimisé

**Résultat** : L'exécutable Linux sera créé à :
```
target/release/musicli (2.3 MB)
```

**Utilisation** :
```bash
./target/release/musicli
```

---

## 🪟 Étape 3 : Compiler pour Windows

Compilez un exécutable Windows (.exe) en utilisant Docker :

```bash
docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu
```

**Explication** :
- `--target x86_64-pc-windows-gnu` : Compile pour Windows 64-bit
- Le reste est identique à la compilation Linux

**Résultat** : L'exécutable Windows sera créé à :
```
target/x86_64-pc-windows-gnu/release/musicli.exe (2.5 MB)
```

**Utilisation sur Windows** :
```cmd
musicli.exe
```

---

## 📦 Récupérer les binaires compilés

Après la compilation, les binaires sont automatiquement disponibles sur votre machine hôte grâce au montage de volume (`-v`).

### Linux
```bash
ls -lh target/release/musicli
file target/release/musicli
```

### Windows
```bash
ls -lh target/x86_64-pc-windows-gnu/release/musicli.exe
file target/x86_64-pc-windows-gnu/release/musicli.exe
```

---

## 🚀 Workflow complet (Linux + Windows)

Pour compiler les deux versions en une seule commande :

```bash
#!/bin/bash
# Construire l'image
docker build -t musicli-builder .

# Compiler pour Linux
echo "Compilation pour Linux..."
docker run --rm -v $(pwd):/app musicli-builder cargo build --release

# Compiler pour Windows
echo "Compilation pour Windows..."
docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu

echo "✅ Compilation terminée !"
echo "Linux   : target/release/musicli"
echo "Windows : target/x86_64-pc-windows-gnu/release/musicli.exe"
```

Sauvegardez ce script dans `build-all.sh` et exécutez-le :

```bash
chmod +x build-all.sh
./build-all.sh
```

---

## 🔍 Vérifier les binaires

### Linux
```bash
file target/release/musicli
# Résultat : ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked

ldd target/release/musicli | grep libasound
# Résultat : libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2
```

### Windows
```bash
file target/x86_64-pc-windows-gnu/release/musicli.exe
# Résultat : PE32+ executable (console) x86-64 (stripped to external PDB), for MS Windows

strings target/x86_64-pc-windows-gnu/release/musicli.exe | head -20
```

---

## 📋 Avantages de Docker

✅ **Isolation** : Les dépendances ne polluent pas votre système  
✅ **Reproductibilité** : Compilation identique sur toutes les machines  
✅ **Cross-compilation** : Compilez pour Windows depuis Linux sans complexité  
✅ **Nettoyage facile** : Supprimez l'image quand vous n'en avez plus besoin  
✅ **CI/CD ready** : Parfait pour l'intégration continue  

---

## 🧹 Nettoyer Docker

### Supprimer l'image
```bash
docker rmi musicli-builder
```

### Supprimer les conteneurs arrêtés
```bash
docker container prune
```

### Supprimer tout (attention !)
```bash
docker system prune -a
```

---

## 🐛 Troubleshooting Docker

### Erreur : "Cannot connect to Docker daemon"

Assurez-vous que Docker est en cours d'exécution :

```bash
sudo systemctl start docker
# ou
sudo service docker start
```

### Erreur : "Permission denied while trying to connect to Docker daemon"

Ajoutez votre utilisateur au groupe Docker :

```bash
sudo usermod -aG docker $USER
newgrp docker
```

### Erreur : "target not installed"

Reconstruisez l'image Docker :

```bash
docker build --no-cache -t musicli-builder .
```

### La compilation est lente

C'est normal la première fois. Les dépendances sont mises en cache après.

Pour forcer une recompilation complète :

```bash
docker run --rm -v $(pwd):/app musicli-builder cargo clean
docker run --rm -v $(pwd):/app musicli-builder cargo build --release
```

---

## 📊 Comparaison : Compilation directe vs Docker

| Aspect | Compilation directe | Docker |
|--------|-------------------|--------|
| Installation | Complexe (dépendances système) | Simple (docker build) |
| Cross-compilation | Difficile | Facile |
| Isolation | Non | Oui |
| Reproductibilité | Variable | Garantie |
| Espace disque | Moins | Plus |
| Vitesse | Plus rapide | Légèrement plus lent |

---

## 🔗 Ressources

- [Documentation Docker](https://docs.docker.com/)
- [Rust in Docker](https://hub.docker.com/_/rust)
- [Cross-compilation Rust](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## ✅ Résumé

```bash
# 1. Construire l'image
docker build -t musicli-builder .

# 2. Compiler pour Linux
docker run --rm -v $(pwd):/app musicli-builder cargo build --release

# 3. Compiler pour Windows
docker run --rm -v $(pwd):/app musicli-builder cargo build --release --target x86_64-pc-windows-gnu

# 4. Récupérer les binaires
ls -lh target/release/musicli
ls -lh target/x86_64-pc-windows-gnu/release/musicli.exe
```

Prêt à compiler ! 🚀
