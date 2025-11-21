# Architecture de MusicLI

## Vue d'ensemble

MusicLI est structuré en 4 modules principaux qui communiquent via le fichier `main.rs`.

```
┌─────────────────────────────────────────────────────┐
│                    main.rs                          │
│              (Orchestration)                        │
└────────────┬──────────────┬──────────────┬──────────┘
             │              │              │
      ┌──────▼──┐    ┌──────▼──┐    ┌─────▼────┐
      │file_    │    │   ui    │    │  audio_  │
      │manager  │    │         │    │  engine  │
      └─────────┘    └─────────┘    └──────────┘
```

## Modules détaillés

### 1. file_manager.rs

**Responsabilité** : Gestion des fichiers audio

**Structures principales** :
- `Track` : Représente une piste audio
  - `name: String` - Nom du fichier (sans extension)
  - `path: PathBuf` - Chemin complet du fichier

**Fonctions principales** :
- `scan_music_folder(folder_path: Option<&str>) -> Result<Vec<Track>>`
  - Scanne récursivement un dossier
  - Filtre les extensions : .mp3, .wav, .flac, .ogg
  - Retourne les pistes triées alphabétiquement
  - Utilise `walkdir` pour la traversée efficace

**Flux** :
```
scan_music_folder()
  ├─ Résout le chemin (~/Musique ou courant)
  ├─ Parcourt récursivement avec WalkDir
  ├─ Filtre par extension audio
  ├─ Crée des Track
  └─ Retourne Vec<Track> trié
```

### 2. ui.rs

**Responsabilité** : Interface utilisateur interactive

**Fonctions principales** :
- `select_track(tracks: &[Track]) -> Result<Track>`
  - Menu interactif avec inquire
  - Recherche floue activée
  - Navigation au clavier (vim mode)
  - Retourne la piste sélectionnée

- `display_now_playing(track_name: &str)`
  - Affiche le nom de la piste en cours
  - Utilise colored pour le style (cyan, vert, gras)

- `display_error(message: &str)`
  - Affiche un message d'erreur en rouge

- `display_success(message: &str)`
  - Affiche un message de succès en vert

**Dépendances** :
- `inquire` pour les menus interactifs
- `colored` pour le styling

### 3. audio_engine.rs

**Responsabilité** : Lecture audio et feedback visuel

**Structures principales** :
- `AudioPlayer` : Gère la lecture d'une piste
  - `track: Track` - La piste à jouer

**Méthodes principales** :
- `new(track: Track) -> Self` - Crée un lecteur
- `play(&self) -> Result<()>` - Lance la lecture
  - Ouvre le fichier audio
  - Crée un OutputStream et Sink
  - Lance un thread avec spinner animé
  - Bloque jusqu'à la fin de la lecture
  - Appelle `afficher_paroles()` après

**Spinner animé** :
- Utilise `indicatif::ProgressBar`
- Style personnalisé avec caractères Braille
- Mise à jour toutes les 100ms
- Couleur cyan

**Fonction placeholder** :
- `afficher_paroles(track_name: &str)` - À implémenter

**Dépendances** :
- `rodio` pour la lecture audio
- `indicatif` pour le spinner
- `std::thread` pour la gestion du spinner

### 4. main.rs

**Responsabilité** : Orchestration et gestion d'erreurs

**Flux principal** :
```
main()
  ├─ Affiche le titre coloré
  ├─ Appelle run()
  └─ Gère les erreurs
    
run()
  ├─ Scanne les fichiers (file_manager)
  ├─ Affiche le nombre de pistes
  ├─ Affiche le menu (ui)
  ├─ Récupère la sélection
  ├─ Affiche "Now Playing" (ui)
  ├─ Lance la lecture (audio_engine)
  └─ Retourne le résultat
```

## Flux de données

```
1. Démarrage
   └─> main() affiche le titre

2. Scan
   └─> scan_music_folder()
       └─> Vec<Track>

3. Sélection
   └─> select_track(tracks)
       └─> Track sélectionnée

4. Lecture
   └─> AudioPlayer::play()
       ├─> Spinner animé (thread)
       ├─> Lecture audio (rodio)
       └─> afficher_paroles() (placeholder)

5. Fin
   └─> Message de succès
```

## Gestion des erreurs

Tous les modules retournent `Result<T, Box<dyn std::error::Error>>` pour une gestion flexible des erreurs.

**Erreurs possibles** :
- Fichier non trouvé
- Pas de périphérique audio
- Format audio non supporté
- Aucune piste trouvée

## Points d'extension

1. **afficher_paroles()** - Implémenter l'affichage des paroles
2. **Playlist** - Ajouter une gestion de playlist
3. **Contrôles** - Pause, skip, volume
4. **Historique** - Mémoriser les pistes jouées
5. **Thèmes** - Personnaliser les couleurs

## Dépendances externes

| Crate | Version | Usage |
|-------|---------|-------|
| inquire | 0.6 | Menus interactifs |
| indicatif | 0.17 | Spinners et barres |
| colored | 2.0 | Styling du texte |
| rodio | 0.17 | Lecture audio |
| walkdir | 2 | Scan récursif |
| tokio | 1 | Runtime async (optionnel) |

## Performance

- **Scan** : O(n) où n = nombre de fichiers
- **Tri** : O(n log n) par nom
- **Recherche floue** : Gérée par inquire
- **Lecture** : Streaming via rodio

## Sécurité

- Pas d'injection de commande
- Validation des chemins
- Gestion des erreurs exhaustive
- Pas d'accès non autorisé aux fichiers
