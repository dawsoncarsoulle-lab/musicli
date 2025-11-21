# Guide d'installation complet de MusicLI

## Prérequis système

### Ubuntu/Debian

#### 1. Installer les dépendances audio (ALSA)

Rodio nécessite les librairies ALSA pour fonctionner :

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev pkg-config
```

**Explication** :
- `libasound2-dev` : Headers et librairies ALSA (Advanced Linux Sound Architecture)
- `pkg-config` : Utilitaire pour localiser les librairies lors de la compilation

#### 2. Installer Rust (si nécessaire)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Vérifiez l'installation :

```bash
rustc --version
cargo --version
```

## Installation de MusicLI

### Méthode 1 : Installation automatique (recommandée)

```bash
cd /home/dawson/Documents/Projet/Rust/musique
bash install.sh
```

Le script :
1. Installe les dépendances système
2. Compile en mode release
3. Copie l'exécutable dans `~/.local/bin`
4. Configure le PATH si nécessaire

### Méthode 2 : Installation manuelle

#### Étape 1 : Compiler en mode release

```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo build --release
```

**Temps estimé** : 10-15 secondes (première compilation peut être plus longue)

#### Étape 2 : Copier l'exécutable

```bash
mkdir -p ~/.local/bin
cp target/release/musicli ~/.local/bin/
chmod +x ~/.local/bin/musicli
```

#### Étape 3 : Ajouter au PATH

Vérifiez si `~/.local/bin` est dans votre PATH :

```bash
echo $PATH
```

Si `~/.local/bin` n'apparaît pas, ajoutez-le à votre fichier de configuration shell.

**Pour Bash** (~/.bashrc) :

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Pour Zsh** (~/.zshrc) :

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Pour Fish** (~/.config/fish/config.fish) :

```bash
echo 'set -gx PATH $HOME/.local/bin $PATH' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish
```

#### Étape 4 : Vérification

```bash
which musicli
musicli
```

### Méthode 3 : Installation via cargo install

```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo install --path .
```

L'exécutable sera installé dans `~/.cargo/bin/musicli` (généralement déjà dans le PATH).

## Utilisation

### Lancer l'application

```bash
musicli
```

### Première utilisation

1. L'application scanne `~/Musique` ou le répertoire courant
2. Un menu interactif affiche les pistes trouvées
3. Tapez pour chercher une chanson (recherche floue)
4. Appuyez sur Entrée pour sélectionner
5. La musique se lance avec un spinner animé

### Raccourcis clavier

| Touche | Action |
|--------|--------|
| ↑ / ↓ | Navigation |
| / | Recherche |
| Entrée | Sélectionner |
| Esc | Quitter |
| j / k | Navigation (vim mode) |

## Compilation

### Mode développement

Pour tester pendant le développement :

```bash
cargo build
./target/debug/musicli
```

**Avantages** : Compilation rapide
**Inconvénients** : Performance réduite

### Mode release

Pour une utilisation en production :

```bash
cargo build --release
./target/release/musicli
```

**Avantages** : Performance optimale
**Inconvénients** : Compilation plus lente

## Désinstallation

### Si installé via install.sh ou manuellement

```bash
rm ~/.local/bin/musicli
```

### Si installé via cargo install

```bash
cargo uninstall musique
```

## Troubleshooting

### Erreur : "Aucun périphérique audio disponible"

**Cause** : ALSA n'est pas correctement configuré

**Solution** :

```bash
aplay -l
```

Si aucun périphérique n'apparaît, vérifiez votre configuration audio système.

### Erreur : "Aucune piste audio trouvée"

**Cause** : Pas de fichiers audio dans `~/Musique` ou le répertoire courant

**Solution** :

1. Créez le dossier `~/Musique` :

```bash
mkdir -p ~/Musique
```

2. Placez des fichiers audio (MP3, WAV, FLAC, OGG) dedans

3. Relancez `musicli`

### Erreur : "command not found: musicli"

**Cause** : Le PATH n'est pas correctement configuré

**Solution** :

1. Vérifiez où se trouve l'exécutable :

```bash
find ~ -name musicli -type f 2>/dev/null
```

2. Ajoutez le répertoire au PATH comme décrit ci-dessus

3. Rechargez votre shell :

```bash
source ~/.bashrc  # ou ~/.zshrc
```

### Erreur de compilation : "error: linker `cc` not found"

**Cause** : Les outils de compilation C ne sont pas installés

**Solution** :

```bash
sudo apt-get install -y build-essential
```

### Erreur de compilation : "error: pkg-config not found"

**Cause** : pkg-config n'est pas installé

**Solution** :

```bash
sudo apt-get install -y pkg-config
```

### Erreur de compilation : "error: libasound2-dev not found"

**Cause** : Les headers ALSA ne sont pas installés

**Solution** :

```bash
sudo apt-get install -y libasound2-dev
```

## Vérification de l'installation

Après l'installation, vérifiez que tout fonctionne :

```bash
musicli --help 2>/dev/null || echo "Pas d'aide disponible"
which musicli
file $(which musicli)
```

## Mise à jour

Pour mettre à jour MusicLI :

```bash
cd /home/dawson/Documents/Projet/Rust/musique
git pull
cargo build --release
cp target/release/musicli ~/.local/bin/
```

## Support

Pour les problèmes :

1. Vérifiez les prérequis système
2. Consultez la section Troubleshooting
3. Vérifiez les logs de compilation
4. Consultez la documentation de rodio et inquire

## Fichiers importants

```
~/.local/bin/musicli          # Exécutable installé
~/.bashrc / ~/.zshrc          # Configuration du shell
~/Musique/                    # Dossier des musiques
```
