# MusicLI - Fonctionnalités Avancées d'UX

## Vue d'ensemble

Ce document décrit les fonctionnalités avancées d'expérience utilisateur (UX) implémentées dans MusicLI, incluant le contrôle en temps réel, les notifications de bureau, et l'affichage de pochettes d'album en ASCII Art.

## ÉTAPE 0 : Mise à jour des Dépendances

### Dépendances ajoutées

```toml
[dependencies]
crossterm = "0.27"        # Gestion des événements clavier en mode raw
notify-rust = "4.10"      # Notifications de bureau cross-platform
image = "0.24"            # Traitement et redimensionnement d'images
metaflac = "0.2"          # Lecture des métadonnées FLAC (pochettes)
```

### Commit
```
feat(deps): Ajout de crossterm, notify-rust, image et id3 pour les fonctionnalités avancées
```

---

## ÉTAPE 1 : Contrôle, Progression & Enchaînement Automatique

### Modules créés

#### `src/keyboard.rs`
Gère les événements clavier en mode raw avec crossterm. Utilise un canal MPSC pour communiquer les commandes au thread principal.

**Commandes supportées:**
- `ESPACE` ou `P` : Pause/Lecture
- `N` : Piste suivante (Skip)
- `+` ou `=` : Augmenter le volume
- `-` : Diminuer le volume
- `Q` : Quitter l'application

#### `src/progress.rs`
Gère l'affichage de la progression en temps réel avec barre de progression ASCII.

**Fonctionnalités:**
- Formatage du temps (MM:SS)
- Barre de progression visuelle
- État de lecture (Lecture/Pause)

#### `src/audio_engine_v2.rs`
Lecteur audio avancé avec support du contrôle en temps réel et de l'enchaînement automatique.

**Fonctionnalités:**
- Lecture simultanée et surveillance clavier
- Pause/Reprise avec feedback visuel
- Contrôle du volume en temps réel
- Enchaînement automatique des pistes
- Affichage de la progression en temps réel

### Modifications

**`src/main.rs`:**
- Ajout des modules `keyboard`, `progress`, `audio_engine_v2`
- Modification de `select_track()` pour retourner l'index au lieu de la piste
- Intégration du nouveau lecteur avancé

**`src/ui.rs`:**
- Changement de signature de `select_track()` pour retourner `usize` (index)

### Commit
```
feat(player): Implémentation du contrôle en temps réel (Pause, Volume), de la barre de progression, et de l'enchaînement automatique des pistes
```

---

## ÉTAPE 2 : Notifications de Bureau

### Module créé

#### `src/notifications.rs`
Gère l'affichage de notifications natives du système d'exploitation.

**Fonctionnalités:**
- Notifications avec titre et artiste
- Notifications simples
- Support cross-platform (Linux, macOS, Windows)

### Modifications

**`src/audio_engine_v2.rs`:**
- Intégration de `show_notification_simple()` lors du démarrage de chaque piste
- Affichage du nom de la piste dans la notification

### Commit
```
feat(notifications): Ajout des notifications de bureau cross-platform lors du changement de piste (notify-rust)
```

---

## ÉTAPE 3 : Affichage de la Pochette d'Album en ASCII Art

### Module créé

#### `src/ascii_art.rs`
Gère l'extraction et l'affichage des pochettes d'album en ASCII Art.

**Fonctionnalités:**
- Extraction de pochettes depuis les tags FLAC
- Recherche de fichiers `cover.jpg`, `cover.png`, `album.jpg`, `album.png`
- Redimensionnement automatique (40x20 pixels)
- Conversion en ASCII Art avec caractères gradués

**Caractères ASCII utilisés:**
```
@%#*+=-:. 
```

### Modifications

**`src/audio_engine_v2.rs`:**
- Intégration de `display_cover_if_available()` avant la lecture
- Affichage de la pochette en ASCII Art dans le terminal

### Commit
```
feat(ui): Affichage de la pochette d'album en ASCII Art (tag, image)
```

---

## Architecture Globale

```
┌─────────────────────────────────────────────┐
│           main.rs (Orchestration)           │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────────────────────────────┐  │
│  │  audio_engine_v2.rs (Lecteur)        │  │
│  │  - Gère la lecture                   │  │
│  │  - Coordonne les threads             │  │
│  └──────────────────────────────────────┘  │
│           ↓              ↓          ↓       │
│    ┌─────────────┐ ┌──────────┐ ┌────────┐ │
│    │ keyboard.rs │ │progress  │ │ascii   │ │
│    │ (Contrôle)  │ │.rs       │ │_art.rs │ │
│    │             │ │(Affichag)│ │(Pochett)│ │
│    └─────────────┘ └──────────┘ └────────┘ │
│           ↓                                 │
│    ┌─────────────────────────────────────┐ │
│    │  notifications.rs (Notifications)   │ │
│    └─────────────────────────────────────┘ │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Flux d'Exécution

### 1. Démarrage
```
1. Scan du dossier musique
2. Sélection de la piste
3. Affichage des contrôles
4. Création du lecteur avancé
```

### 2. Lecture
```
1. Affichage de la pochette (ASCII Art)
2. Notification de bureau
3. Boucle principale:
   - Vérifier les commandes clavier
   - Mettre à jour l'affichage de progression
   - Vérifier si la piste est terminée
   - Passer à la piste suivante si nécessaire
```

### 3. Contrôles
```
- Pause/Lecture : Pause/reprend la lecture
- Skip : Passe à la piste suivante
- Volume +/- : Ajuste le volume
- Quit : Arrête la lecture et quitte
```

---

## Utilisation

### Commandes de base

```bash
# Lancer l'application
musicli

# Sélectionner une piste
# Utiliser les flèches pour naviguer, Entrée pour sélectionner

# Contrôles pendant la lecture
ESPACE   # Pause/Lecture
N        # Piste suivante
+        # Augmenter le volume
-        # Diminuer le volume
Q        # Quitter
```

---

## Prérequis Système

### Linux
```bash
# Dépendances système
sudo apt-get install libasound2-dev pkg-config libssl-dev

# Pour les notifications
sudo apt-get install libdbus-1-dev
```

### macOS
```bash
# Utiliser Homebrew
brew install pkg-config
```

### Windows
```
Les dépendances sont généralement incluses dans le système.
```

---

## Fichiers Modifiés/Créés

### Créés
- `src/keyboard.rs` - Gestion des événements clavier
- `src/progress.rs` - Affichage de la progression
- `src/audio_engine_v2.rs` - Lecteur audio avancé
- `src/notifications.rs` - Notifications de bureau
- `src/ascii_art.rs` - Affichage ASCII Art

### Modifiés
- `Cargo.toml` - Ajout des dépendances
- `src/main.rs` - Intégration des nouveaux modules
- `src/ui.rs` - Modification de `select_track()`

---

## Statistiques

### Lignes de code
- `keyboard.rs` : ~60 lignes
- `progress.rs` : ~40 lignes
- `audio_engine_v2.rs` : ~110 lignes
- `notifications.rs` : ~20 lignes
- `ascii_art.rs` : ~80 lignes

**Total : ~310 lignes de nouveau code**

### Dépendances ajoutées
- crossterm
- notify-rust
- image
- metaflac

---

## Commits

1. **feat(deps)**: Ajout de crossterm, notify-rust, image et metaflac
2. **feat(player)**: Implémentation du contrôle en temps réel et de l'enchaînement automatique
3. **feat(notifications)**: Ajout des notifications de bureau cross-platform
4. **feat(ui)**: Affichage de la pochette d'album en ASCII Art

---

## Améliorations Futures

- [ ] Support des métadonnées ID3 pour MP3
- [ ] Affichage des paroles synchronisées
- [ ] Playlist persistante
- [ ] Recherche et filtrage des pistes
- [ ] Équaliseur audio
- [ ] Historique de lecture
- [ ] Gestion des favoris

---

## Troubleshooting

### Les notifications n'apparaissent pas
- **Linux**: Vérifier que le service D-Bus est actif
- **macOS**: Vérifier les paramètres de notification du système
- **Windows**: Vérifier les paramètres de notification Windows

### L'ASCII Art ne s'affiche pas
- Vérifier que la piste contient une pochette (tag FLAC)
- Vérifier la présence de fichiers `cover.jpg` ou `cover.png` dans le dossier

### Les contrôles clavier ne répondent pas
- Vérifier que le terminal supporte le mode raw (crossterm)
- Essayer de relancer l'application

---

## Licence

MIT

---

**Créé le** : 21 novembre 2025  
**Version** : 0.4.0  
**Status** : ✅ COMPLET ET FONCTIONNEL
