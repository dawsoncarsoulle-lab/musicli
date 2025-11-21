mod audio_engine;
mod audio_engine_v2;
mod downloader;
mod file_manager;
mod keyboard;
mod progress;
mod ui;

use audio_engine::AudioPlayer;
use clap::{Parser, Subcommand};
use colored::*;
use downloader::download_music;
use file_manager::scan_music_folder;
use std::path::PathBuf;
use ui::{display_error, display_now_playing, display_success, select_track};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "musicli")]
#[command(about = "Lecteur audio interactif avec support du téléchargement", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    download: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Version,
    Download {
        url: String,
    },
}

/// Retourne le chemin du dossier de musique par défaut selon l'OS.
fn get_music_dir() -> PathBuf {
    if let Some(audio_dir) = dirs::audio_dir() {
        audio_dir
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

fn main() {
    println!("{}", "\n🎵 Bienvenue dans MusicLI 🎵\n".cyan().bold());

    let cli = Cli::parse();

    // Gestion des commandes
    match cli.command {
        Some(Commands::Version) => {
            println!("MusicLI version {}", VERSION);
            return;
        }
        Some(Commands::Download { url }) => {
            let music_dir = get_music_dir();
            if let Err(e) = download_music(&url, &music_dir) {
                display_error(&format!("Erreur de téléchargement: {}", e));
                std::process::exit(1);
            }
            return;
        }
        None => {
            if let Some(url) = cli.download {
                let music_dir = get_music_dir();
                if let Err(e) = download_music(&url, &music_dir) {
                    display_error(&format!("Erreur de téléchargement: {}", e));
                    std::process::exit(1);
                }
                return;
            }
        }
    }

    match run() {
        Ok(_) => {
            display_success("Merci d'avoir utilisé MusicLI!");
        }
        Err(e) => {
            display_error(&format!("Erreur: {}", e));
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = get_music_dir();

    println!(
        "{} Scan en cours dans : {}\n",
        "📁".cyan(),
        music_dir.display().to_string().cyan().bold()
    );

    let tracks = scan_music_folder(Some(music_dir.to_str().unwrap_or(".")))?;

    if tracks.is_empty() {
        return Err(format!(
            "Aucune piste audio trouvée dans {}",
            music_dir.display()
        )
        .into());
    }

    println!("{} {} pistes trouvées.\n", "✓".green(), tracks.len());

    let selected_index = select_track(&tracks)?;

    println!(
        "\n{} Contrôles: [ESPACE/P] Pause | [N] Suivant | [+/-] Volume | [Q] Quitter\n",
        "ℹ".cyan()
    );

    let mut player = audio_engine_v2::AdvancedAudioPlayer::new(tracks, selected_index);
    player.play_all()?;

    Ok(())
}
