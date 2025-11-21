use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::path::Path;

const ASCII_CHARS: &str = "@%#*+=-:. ";

pub fn display_ascii_art_from_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?
        .decode()?
        .resize(40, 20, image::imageops::FilterType::Lanczos3)
        .to_luma8();

    print_ascii_image(&img);
    Ok(())
}

pub fn display_ascii_art_from_bytes(
    image_data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()?
        .decode()?
        .resize(40, 20, image::imageops::FilterType::Lanczos3)
        .to_luma8();

    print_ascii_image(&img);
    Ok(())
}

fn print_ascii_image(img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>) {
    println!("\n{}", "═".repeat(42));
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            let char_index = (pixel as usize / 256) * ASCII_CHARS.len();
            let char_index = char_index.min(ASCII_CHARS.len() - 1);
            print!("{}", ASCII_CHARS.chars().nth(char_index).unwrap_or(' '));
        }
        println!();
    }
    println!("{}\n", "═".repeat(42));
}

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
