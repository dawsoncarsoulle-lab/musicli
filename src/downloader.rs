use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn download_music(
    url: &str,
    music_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "\n{} Téléchargement de la musique dans : {}\n",
        "⬇".cyan().bold(),
        music_dir.display().to_string().cyan().bold()
    );

    let check_cmd = if cfg!(target_os = "windows") {
        Command::new("where").arg("yt-dlp").output()
    } else {
        Command::new("which").arg("yt-dlp").output()
    };

    if check_cmd.is_err() || !check_cmd?.status.success() {
        return Err(
            "❌ yt-dlp n'est pas installé.\n\
             Linux/macOS : sudo apt-get install yt-dlp\n\
             Windows : choco install yt-dlp (ou pip install yt-dlp)"
                .into(),
        );
    }

    std::fs::create_dir_all(music_dir)?;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Téléchargement en cours...");

    let output_template = if cfg!(target_os = "windows") {
        format!("{}\\%(title)s.%(ext)s", music_dir.display())
    } else {
        format!("{}/%(title)s.%(ext)s", music_dir.display())
    };

    let mut child = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(&output_template)
        .arg(url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    loop {
        pb.tick();
        thread::sleep(Duration::from_millis(80));

        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    pb.finish_with_message("✓ Téléchargement réussi !".green().to_string());
                    println!();
                    return Ok(());
                } else {
                    pb.finish_with_message("❌ Erreur lors du téléchargement".red().to_string());
                    println!();
                    return Err("Le téléchargement a échoué. Vérifiez l'URL et votre connexion.".into());
                }
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                pb.finish_with_message("❌ Erreur".red().to_string());
                return Err(format!("Erreur lors du téléchargement : {}", e).into());
            }
        }
    }
}
