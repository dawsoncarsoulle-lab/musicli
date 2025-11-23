use std::time::{Duration, Instant};

pub struct ProgressTracker {
    pub total_duration: Duration,
    pub accumulated_duration: Duration,
    pub is_playing: bool,
    playback_start: Option<Instant>,
    pub duration_unknown: bool,
}

impl ProgressTracker {
    pub fn new(total_duration: Duration) -> Self {
        let duration_unknown = total_duration.as_secs() >= 3600; // Si >= 1h, c'est probablement la durée par défaut
        ProgressTracker {
            total_duration,
            accumulated_duration: Duration::ZERO,
            is_playing: true,
            playback_start: Some(Instant::now()),
            duration_unknown,
        }
    }

    pub fn pause(&mut self) {
        if let Some(start) = self.playback_start {
            self.accumulated_duration += start.elapsed();
            self.playback_start = None;
        }
        self.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.playback_start = Some(Instant::now());
        self.is_playing = true;
    }

    pub fn update_position(&mut self) {
        if self.is_playing {
            if let Some(start) = self.playback_start {
                self.accumulated_duration += start.elapsed();
                self.playback_start = Some(Instant::now());
            }
        }
    }

    pub fn get_current_position(&self) -> Duration {
        let mut current = self.accumulated_duration;
        if self.is_playing {
            if let Some(start) = self.playback_start {
                current += start.elapsed();
            }
        }
        current
    }

    pub fn format_time(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn get_progress_bar(&self, width: usize) -> String {
        let current = self.get_current_position();
        let filled = if self.total_duration.as_secs() > 0 {
            (current.as_secs_f64() / self.total_duration.as_secs_f64() * width as f64)
                as usize
        } else {
            0
        };

        let empty = width.saturating_sub(filled);
        let bar = format!(
            "[{}{}]",
            "=".repeat(filled),
            " ".repeat(empty)
        );
        bar
    }

    pub fn get_status_line(&self) -> String {
        let status = if self.is_playing { "▶ Lecture" } else { "⏸ Pause" };
        let current = Self::format_time(self.get_current_position());
        let total = if self.duration_unknown {
            "Durée Inconnue".to_string()
        } else {
            Self::format_time(self.total_duration)
        };
        let progress_bar = self.get_progress_bar(30);

        format!(
            "{} {} {} / {}",
            status, progress_bar, current, total
        )
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        self.get_current_position() >= self.total_duration
    }
}
