use notify_rust::Notification;

pub fn show_notification(title: &str, artist: &str) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("🎵 MusicLI")
        .body(&format!("{}\nArtiste: {}", title, artist))
        .icon("media-playback-start")
        .show()?;

    Ok(())
}

pub fn show_notification_simple(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("🎵 MusicLI")
        .body(message)
        .icon("media-playback-start")
        .show()?;

    Ok(())
}
