use colored::*;
use std::path::Path;

pub fn display_vinyl_art() {
    println!("\n{}", "╔════════════════════════════════════════╗".cyan());
    println!("{}", "║                                        ║".cyan());
    println!("{}", "║           ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉           ║".yellow().bold());
    println!("{}", "║         ◉               ◉             ║".yellow());
    println!("{}", "║       ◉      ╭─────╮      ◉           ║".yellow());
    println!("{}", "║      ◉      │  🎵  │      ◉          ║".yellow().bold());
    println!("{}", "║     ◉       │  ◎◎  │       ◉         ║".yellow());
    println!("{}", "║     ◉       ╰─────╯       ◉         ║".yellow());
    println!("{}", "║       ◉                 ◉             ║".yellow());
    println!("{}", "║         ◉             ◉               ║".yellow());
    println!("{}", "║           ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉           ║".yellow().bold());
    println!("{}", "║                                        ║".cyan());
    println!("{}", "╚════════════════════════════════════════╝".cyan());
    println!();
}

#[allow(dead_code)]
pub fn try_extract_cover_from_flac(
    path: &Path,
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    if path.extension().and_then(|s| s.to_str()) == Some("flac") {
        if let Ok(metaflac) = metaflac::Tag::read_from_path(path) {
            if let Some(picture) = metaflac.pictures().next() {
                return Ok(Some(picture.data.clone()));
            }
        }
    }
    Ok(None)
}

#[allow(dead_code)]
pub fn display_cover_if_available(
    track_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Essayer d'extraire la pochette d'un fichier FLAC
    if let Ok(Some(cover_data)) = try_extract_cover_from_flac(track_path) {
        let _ = display_ascii_art_from_bytes(&cover_data);
        return Ok(());
    }

    // Essayer de trouver une image cover.jpg ou cover.png dans le même répertoire
    if let Some(parent) = track_path.parent() {
        for cover_name in &["cover.jpg", "cover.png", "album.jpg", "album.png"] {
            let cover_path = parent.join(cover_name);
            if cover_path.exists() {
                let _ = display_ascii_art_from_file(&cover_path);
                return Ok(());
            }
        }
    }

    Ok(())
}

#[allow(dead_code, unused_variables)]
pub fn display_ascii_art_from_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[allow(dead_code)]
pub fn display_ascii_art_from_bytes(
    _image_data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
