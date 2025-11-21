use crate::file_manager::Track;
use colored::*;
use inquire::Select;

pub fn select_track(tracks: &[Track]) -> Result<Track, Box<dyn std::error::Error>> {
    if tracks.is_empty() {
        return Err("Aucune piste audio trouvée.".into());
    }

    let options: Vec<String> = tracks.iter().map(|t| t.name.clone()).collect();

    let selection = Select::new("Sélectionnez une musique:", options)
        .with_vim_mode(true)
        .with_page_size(10)
        .prompt()?;

    let selected_track = tracks
        .iter()
        .find(|t| t.name == selection)
        .ok_or("Piste non trouvée")?
        .clone();

    Ok(selected_track)
}

pub fn display_now_playing(track_name: &str) {
    println!("\n{} {}\n", "▶".cyan().bold(), track_name.green().bold());
}

pub fn display_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

pub fn display_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}
