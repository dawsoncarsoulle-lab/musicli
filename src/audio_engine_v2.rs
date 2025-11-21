use crate::file_manager::Track;
use crate::keyboard::{KeyboardListener, PlayerCommand};
use crate::notifications::show_notification_simple;
use crate::progress::ProgressTracker;
use colored::*;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

pub struct AdvancedAudioPlayer {
    tracks: Vec<Track>,
    current_index: usize,
}

impl AdvancedAudioPlayer {
    pub fn new(tracks: Vec<Track>, start_index: usize) -> Self {
        AdvancedAudioPlayer {
            tracks,
            current_index: start_index,
        }
    }

    pub fn play_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let keyboard = KeyboardListener::new()?;

        while self.current_index < self.tracks.len() {
            let track = &self.tracks[self.current_index];
            println!(
                "\n{} Lecture: {}\n",
                "🎵".cyan(),
                track.name.bold()
            );

            if !self.play_track(track, &keyboard)? {
                break; // Quit command received
            }

            self.current_index += 1;
        }

        println!("\n{} Fin de la playlist.\n", "✓".green());
        Ok(())
    }

    fn play_track(
        &self,
        track: &Track,
        keyboard: &KeyboardListener,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let file = File::open(&track.path)?;
        let reader = BufReader::new(file);
        let source = rodio::Decoder::new(reader)?;

        let total_duration = source.total_duration().unwrap_or(Duration::from_secs(0));
        let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&stream_handle)?;

        sink.append(source);

        // Afficher une notification de bureau
        let _ = show_notification_simple(&format!("Lecture: {}", track.name));

        let mut progress = ProgressTracker::new(total_duration);
        let mut last_display = std::time::Instant::now();

        loop {
            // Vérifier les commandes clavier
            if let Some(cmd) = keyboard.try_recv() {
                match cmd {
                    PlayerCommand::PlayPause => {
                        if sink.is_paused() {
                            sink.play();
                            progress.is_playing = true;
                            println!("{}", "▶ Reprise de la lecture".green());
                        } else {
                            sink.pause();
                            progress.is_playing = false;
                            println!("{}", "⏸ Pause".yellow());
                        }
                    }
                    PlayerCommand::Skip => {
                        println!("{}", "⏭ Piste suivante".cyan());
                        sink.stop();
                        return Ok(true);
                    }
                    PlayerCommand::VolumeUp => {
                        let current = sink.volume();
                        sink.set_volume((current + 0.1).min(1.0));
                        println!("{} Volume: {:.0}%", "🔊", sink.volume() * 100.0);
                    }
                    PlayerCommand::VolumeDown => {
                        let current = sink.volume();
                        sink.set_volume((current - 0.1).max(0.0));
                        println!("{} Volume: {:.0}%", "🔊", sink.volume() * 100.0);
                    }
                    PlayerCommand::Quit => {
                        sink.stop();
                        return Ok(false);
                    }
                }
            }

            // Mettre à jour l'affichage de la progression
            if last_display.elapsed() > Duration::from_millis(500) {
                print!("\r{}", progress.get_status_line());
                std::io::Write::flush(&mut std::io::stdout())?;
                last_display = std::time::Instant::now();
            }

            // Vérifier si la piste est terminée
            if sink.empty() {
                println!("\n{} Piste terminée", "✓".green());
                return Ok(true);
            }

            thread::sleep(Duration::from_millis(100));
        }
    }
}
