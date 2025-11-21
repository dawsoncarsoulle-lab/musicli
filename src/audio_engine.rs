use crate::file_manager::Track;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Gère la lecture audio avec feedback visuel.
pub struct AudioPlayer {
    track: Track,
}

impl AudioPlayer {
    /// Crée un nouveau lecteur audio pour une piste donnée.
    pub fn new(track: Track) -> Self {
        AudioPlayer { track }
    }

    /// Lance la lecture de la piste avec un spinner animé.
    ///
    /// Bloque le thread principal jusqu'à la fin de la lecture.
    pub fn play(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(&self.track.path)?;
        let reader = BufReader::new(file);
        let source = rodio::Decoder::new(reader)?;

        let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&stream_handle)?;

        sink.append(source);

        let is_playing = Arc::new(Mutex::new(true));
        let is_playing_clone = Arc::clone(&is_playing);

        let spinner_thread = thread::spawn(move || {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                indicatif::ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner:.cyan} {msg}")
                    .unwrap(),
            );
            spinner.set_message("En cours de lecture...");

            while *is_playing_clone.lock().unwrap() {
                spinner.tick();
                thread::sleep(Duration::from_millis(100));
            }

            spinner.finish_and_clear();
        });

        sink.sleep_until_end();

        *is_playing.lock().unwrap() = false;
        let _ = spinner_thread.join();

        afficher_paroles(&self.track.name);

        Ok(())
    }
}

/// Placeholder pour afficher les paroles (à implémenter).
fn afficher_paroles(_track_name: &str) {
}
