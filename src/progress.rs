use std::time::Duration;

pub struct ProgressTracker {
    pub total_duration: Duration,
    pub current_position: Duration,
    pub is_playing: bool,
}

impl ProgressTracker {
    pub fn new(total_duration: Duration) -> Self {
        ProgressTracker {
            total_duration,
            current_position: Duration::ZERO,
            is_playing: true,
        }
    }

    pub fn format_time(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn get_progress_bar(&self, width: usize) -> String {
        let filled = if self.total_duration.as_secs() > 0 {
            (self.current_position.as_secs_f64() / self.total_duration.as_secs_f64() * width as f64)
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
        let current = Self::format_time(self.current_position);
        let total = Self::format_time(self.total_duration);
        let progress_bar = self.get_progress_bar(30);

        format!(
            "{} {} {} / {}",
            status, progress_bar, current, total
        )
    }
}
