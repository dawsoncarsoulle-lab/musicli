# MusicLI v0.4.0 - Guide de Démarrage Rapide des Fonctionnalités Avancées

## 🚀 Démarrage Rapide

### 1. Compilation
```bash
cd /home/dawson/Documents/Projet/Rust/musique
cargo build --release
```

### 2. Lancer l'application
```bash
./target/release/musicli
```

### 3. Sélectionner une piste
- Utilisez les **flèches** pour naviguer
- Tapez pour **rechercher** (recherche floue)
- Appuyez sur **Entrée** pour sélectionner

### 4. Contrôles pendant la lecture

| Touche | Action |
|--------|--------|
| `ESPACE` | Pause/Lecture |
| `P` | Pause/Lecture (alternative) |
| `N` | Piste suivante |
| `+` | Volume + |
| `-` | Volume - |
| `Q` | Quitter |

---

## 📺 Affichage

Pendant la lecture, vous verrez:

```
┌──────────────────────────────────────────────┐
│                                              │
│     Pochette d'album en ASCII Art            │
│     (si disponible - FLAC ou cover.jpg)      │
│                                              │
└──────────────────────────────────────────────┘

🎵 Lecture: Nom de la piste

▶ Lecture [==============================          ] 01:23 / 03:45
```

---

## 🔔 Notifications

Une notification de bureau apparaît à chaque changement de piste:

```
🎵 MusicLI
Lecture: Nom de la piste
```

---

## 🎨 Fonctionnalités Avancées

### Contrôle en Temps Réel
- Pause/Lecture sans interruption
- Contrôle du volume en direct
- Skip vers la piste suivante

### Barre de Progression
- Affichage du temps écoulé et total
- Barre ASCII animée
- Mise à jour en temps réel

### Enchaînement Automatique
- Les pistes s'enchaînent automatiquement
- Fin propre de la playlist

### Notifications de Bureau
- Notification à chaque changement de piste
- Support cross-platform

### Pochette d'Album
- Affichage en ASCII Art
- Extraction depuis tags FLAC
- Recherche de fichiers cover.jpg

---

## 📁 Formats Supportés

- MP3
- WAV
- FLAC (avec pochette)
- OGG

---

## 🔍 Où sont les fichiers?

### Modules Principaux
```
src/keyboard.rs        - Gestion des contrôles clavier
src/progress.rs        - Affichage de la progression
src/audio_engine_v2.rs - Lecteur audio avancé
src/notifications.rs   - Notifications de bureau
src/ascii_art.rs       - Affichage ASCII Art
```

### Documentation
```
ADVANCED_UX.md                    - Documentation complète
CODE_SUMMARY.md                   - Résumé du code
IMPLEMENTATION_LOG.md             - Journal d'implémentation
FINAL_IMPLEMENTATION_SUMMARY.md   - Résumé final
```

---

## 🐛 Problèmes Courants

### Les notifications n'apparaissent pas
**Linux**: Vérifier que D-Bus est actif
```bash
systemctl --user status dbus
```

**macOS**: Vérifier les paramètres de notification du système

**Windows**: Vérifier les paramètres de notification Windows

### L'ASCII Art ne s'affiche pas
- Vérifier que la piste a une pochette (tag FLAC)
- Vérifier la présence de `cover.jpg` ou `cover.png` dans le dossier

### Les contrôles ne répondent pas
- Vérifier que le terminal supporte le mode raw
- Essayer de relancer l'application

---

## 📚 Documentation Complète

Pour plus de détails, consultez:

1. **ADVANCED_UX.md** - Documentation complète des fonctionnalités
2. **CODE_SUMMARY.md** - Résumé technique du code
3. **IMPLEMENTATION_LOG.md** - Journal détaillé de l'implémentation
4. **README.md** - Guide utilisateur général

---

## 🔗 Liens Utiles

- **GitHub**: https://github.com/dawsoncarsoulle-lab/musicli
- **Cargo**: https://crates.io/crates/musique
- **Rust**: https://www.rust-lang.org/

---

## 💡 Conseils

### Pour une meilleure expérience
1. Utilisez un terminal moderne (iTerm2, Windows Terminal, GNOME Terminal)
2. Assurez-vous que votre dossier Musique contient des fichiers audio
3. Pour les pochettes, placez `cover.jpg` dans le dossier de la musique

### Raccourcis Utiles
- `Ctrl+C` pour arrêter l'application (si Q ne fonctionne pas)
- `Ctrl+L` pour rafraîchir l'écran (si l'affichage est corrompu)

---

## 🎯 Prochaines Étapes

1. Essayer les contrôles clavier
2. Tester avec différents formats audio
3. Vérifier les notifications
4. Consulter la documentation pour les détails techniques

---

## ✨ Fonctionnalités Implémentées

- ✅ Contrôle en temps réel (Pause, Volume, Skip)
- ✅ Barre de progression avec temps
- ✅ Enchaînement automatique des pistes
- ✅ Notifications de bureau
- ✅ Affichage de pochettes en ASCII Art
- ✅ Support cross-platform (Windows, Linux, macOS)

---

## 📞 Support

Si vous rencontrez des problèmes:

1. Vérifier les prérequis système
2. Consulter le troubleshooting
3. Vérifier les logs de compilation
4. Ouvrir une issue sur GitHub

---

**Version**: 0.4.0  
**Date**: 21 novembre 2025  
**Status**: ✅ COMPLET ET FONCTIONNEL

Bon écoute! 🎵
