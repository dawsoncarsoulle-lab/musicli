use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
    PlayPause,
    Skip,
    VolumeUp,
    VolumeDown,
    Quit,
}

pub struct KeyboardListener {
    rx: Receiver<PlayerCommand>,
}

impl KeyboardListener {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            if let Err(e) = enable_raw_mode() {
                eprintln!("Erreur lors de l'activation du mode raw: {}", e);
                return;
            }

            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                        let command = match code {
                            KeyCode::Char(' ') | KeyCode::Char('p') | KeyCode::Char('P') => {
                                Some(PlayerCommand::PlayPause)
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') => Some(PlayerCommand::Skip),
                            KeyCode::Char('+') | KeyCode::Char('=') => Some(PlayerCommand::VolumeUp),
                            KeyCode::Char('-') => Some(PlayerCommand::VolumeDown),
                            KeyCode::Char('q') | KeyCode::Char('Q') => Some(PlayerCommand::Quit),
                            _ => None,
                        };

                        if let Some(cmd) = command {
                            if tx.send(cmd).is_err() {
                                break;
                            }
                        }
                    }
                }
            }

            let _ = disable_raw_mode();
        });

        Ok(KeyboardListener { rx })
    }

    pub fn try_recv(&self) -> Option<PlayerCommand> {
        self.rx.try_recv().ok()
    }
}

impl Drop for KeyboardListener {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
