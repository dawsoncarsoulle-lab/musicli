use std::path::PathBuf;
use walkdir::WalkDir;

/// Représente une piste audio avec son nom et son chemin.
#[derive(Clone, Debug)]
pub struct Track {
    pub name: String,
    pub path: PathBuf,
}

impl Track {
    /// Crée une nouvelle piste à partir d'un chemin.
    fn new(path: PathBuf) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Track { name, path }
    }
}

/// Scanne récursivement un dossier et retourne les pistes audio trouvées.
///
/// Filtre les fichiers par extension audio (.mp3, .wav, .flac, .ogg).
/// Si le chemin n'existe pas, utilise le répertoire courant.
pub fn scan_music_folder(folder_path: Option<&str>) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let path = match folder_path {
        Some(p) => PathBuf::from(p),
        None => {
            let home = std::env::var("HOME")?;
            PathBuf::from(home).join("Musique")
        }
    };

    let search_path = if path.exists() {
        path
    } else {
        std::env::current_dir()?
    };

    let audio_extensions = ["mp3", "wav", "flac", "ogg"];
    let mut tracks = Vec::new();

    for entry in WalkDir::new(&search_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if audio_extensions.contains(&ext_str.to_lowercase().as_str()) {
                    tracks.push(Track::new(file_path.to_path_buf()));
                }
            }
        }
    }

    tracks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tracks)
}
