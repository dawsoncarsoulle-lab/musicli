# Guide de développement MusicLI

## Configuration de l'environnement de développement

### Prérequis

```bash
sudo apt-get install -y libasound2-dev pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Cloner et configurer

```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo build
```

## Structure du code

```
src/
├── main.rs              # Point d'entrée, orchestration
├── file_manager.rs      # Gestion des fichiers (Track, scan_music_folder)
├── ui.rs                # Interface utilisateur (inquire, colored)
└── audio_engine.rs      # Moteur audio (AudioPlayer, rodio)
```

## Workflow de développement

### 1. Développement itératif

```bash
cargo build
cargo run
```

### 2. Tests

```bash
cargo test
```

### 3. Vérification du code

```bash
cargo clippy
cargo fmt --check
```

### 4. Formatage automatique

```bash
cargo fmt
```

## Ajouter une nouvelle fonctionnalité

### Exemple : Ajouter le contrôle du volume

1. **Créer un nouveau module** `src/volume_control.rs` :

```rust
pub struct VolumeControl {
    level: f32,
}

impl VolumeControl {
    pub fn new() -> Self {
        VolumeControl { level: 1.0 }
    }

    pub fn set_volume(&mut self, level: f32) {
        self.level = (level).clamp(0.0, 1.0);
    }
}
```

2. **Importer dans main.rs** :

```rust
mod volume_control;
use volume_control::VolumeControl;
```

3. **Intégrer dans AudioPlayer** :

```rust
pub struct AudioPlayer {
    track: Track,
    volume: VolumeControl,
}
```

## Points d'extension

### 1. Affichage des paroles

Actuellement, `afficher_paroles()` est un placeholder vide.

**À implémenter** :

```rust
fn afficher_paroles(track_name: &str) {
    println!("Paroles de : {}", track_name);
    
}
```

### 2. Gestion de playlist

Créer un nouveau module `src/playlist.rs` :

```rust
pub struct Playlist {
    tracks: Vec<Track>,
    current_index: usize,
}

impl Playlist {
    pub fn new(tracks: Vec<Track>) -> Self {
        Playlist {
            tracks,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<&Track> {
        if self.current_index < self.tracks.len() - 1 {
            self.current_index += 1;
            Some(&self.tracks[self.current_index])
        } else {
            None
        }
    }
}
```

### 3. Contrôles de lecture

Ajouter des commandes de contrôle :

```rust
pub enum PlaybackControl {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
}
```

### 4. Historique

Créer `src/history.rs` :

```rust
pub struct History {
    tracks: Vec<(Track, DateTime<Utc>)>,
}

impl History {
    pub fn add(&mut self, track: Track) {
        self.tracks.push((track, Utc::now()));
    }
}
```

## Dépendances et versions

Voir `Cargo.toml` pour les versions exactes.

**Versions actuelles** :
- inquire 0.6
- indicatif 0.17
- colored 2.0
- rodio 0.17
- walkdir 2
- tokio 1

**Pour mettre à jour** :

```bash
cargo update
```

## Tests

### Créer un test unitaire

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_creation() {
        let track = Track::new(PathBuf::from("test.mp3"));
        assert_eq!(track.name, "test");
    }
}
```

### Exécuter les tests

```bash
cargo test
cargo test -- --nocapture
```

## Performance

### Profiling

```bash
cargo build --release
perf record ./target/release/musicli
perf report
```

### Optimisations

Le mode release active automatiquement les optimisations :

```bash
cargo build --release
```

## Debugging

### Logs de debug

```rust
eprintln!("Debug: {:?}", variable);
```

### Avec dbg! macro

```rust
let result = dbg!(some_function());
```

### Avec rust-gdb

```bash
cargo build
rust-gdb ./target/debug/musicli
```

## Conventions de code

### Nommage

- **Fonctions** : `snake_case`
- **Structures** : `PascalCase`
- **Constantes** : `SCREAMING_SNAKE_CASE`
- **Modules** : `snake_case`

### Documentation

Toutes les fonctions publiques doivent avoir une doc string :

```rust
/// Scanne récursivement un dossier et retourne les pistes audio.
///
/// Filtre les fichiers par extension audio (.mp3, .wav, .flac, .ogg).
pub fn scan_music_folder(folder_path: Option<&str>) -> Result<Vec<Track>> {
    
}
```

### Formatage

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Compilation et distribution

### Build de développement

```bash
cargo build
```

**Taille** : ~100 MB (avec debug info)

### Build release

```bash
cargo build --release
```

**Taille** : ~10 MB

### Strip des symboles

```bash
strip target/release/musicli
```

**Taille finale** : ~5 MB

## Dépannage du développement

### Erreur : "error: linker `cc` not found"

```bash
sudo apt-get install -y build-essential
```

### Erreur : "error: pkg-config not found"

```bash
sudo apt-get install -y pkg-config
```

### Erreur : "error: libasound2-dev not found"

```bash
sudo apt-get install -y libasound2-dev
```

### Compilation lente

La première compilation est lente car toutes les dépendances sont compilées.

Les compilations suivantes sont beaucoup plus rapides.

Pour accélérer :

```bash
cargo build -j 8
```

### Erreurs de type complexes

Utilisez `cargo expand` pour voir le code généré :

```bash
cargo install cargo-expand
cargo expand
```

## Ressources utiles

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rodio Documentation](https://docs.rs/rodio/)
- [Inquire Documentation](https://docs.rs/inquire/)
- [Indicatif Documentation](https://docs.rs/indicatif/)

## Contribution

1. Fork le projet
2. Créez une branche (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrez une Pull Request

## Licence

MIT
