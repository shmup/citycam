use anyhow::{anyhow, Result};
use chrono::Local;
use image::{GrayImage, ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use tempfile::NamedTempFile;

fn main() -> Result<()> {
    // Create cache directory
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;

    // Get first frame from stream
    let image = get_first_frame()?;

    // Create filename with timestamp
    let filename = Local::now().format("%Y%m%d-%H%M%S.jpg").to_string();
    let output_path = cache_dir.join(filename);

    // Convert to grayscale and add noise
    let gray_image = image::imageops::grayscale(&image);
    let noisy_image = add_gaussian_noise(&gray_image, 0.0, 35.0);

    // Save the image
    noisy_image.save(&output_path)?;

    // Set as wallpaper
    set_wallpaper(&output_path)?;

    Ok(())
}

fn get_cache_dir() -> Result<PathBuf> {
    let home = env::var("HOME").map_err(|_| anyhow!("HOME environment variable not set"))?;
    Ok(PathBuf::from(home).join(".cache").join("livecam"))
}

fn get_current_stream_url() -> Result<String> {
    let frame_url =
        "https://api.wetmet.net/widgets/stream/frame.php?uid=b1f85cdf621772894ff3300e78dd6035";
    let response = reqwest::blocking::get(frame_url)?.text()?;

    let re = Regex::new(r"var vurl = '(https://[^']+)'")?;
    if let Some(captures) = re.captures(&response) {
        Ok(captures[1].to_string())
    } else {
        Err(anyhow!(
            "Could not find stream URL in the frame.php response"
        ))
    }
}

fn get_first_frame() -> Result<RgbImage> {
    let m3u8_url = get_current_stream_url()?;

    // Get master playlist
    let response = reqwest::blocking::get(&m3u8_url)?.text()?;

    // Find chunks playlist
    let base_url = m3u8_url
        .rsplit_once('/')
        .map(|(base, _)| base)
        .unwrap_or("")
        .to_string()
        + "/";

    let chunks_playlist = response
        .lines()
        .find(|line| !line.starts_with('#') && line.contains(".m3u8"))
        .ok_or_else(|| anyhow!("No chunks playlist found"))?;

    let chunks_playlist_url = format!("{}{}", base_url, chunks_playlist);

    // Get chunks playlist
    let chunks_response = reqwest::blocking::get(&chunks_playlist_url)?.text()?;

    // Find first segment
    let chunks_base_url = chunks_playlist_url
        .rsplit_once('/')
        .map(|(base, _)| base)
        .unwrap_or("")
        .to_string()
        + "/";

    let segment = chunks_response
        .lines()
        .find(|line| !line.starts_with('#') && (line.contains(".ts") || line.contains("?")))
        .ok_or_else(|| anyhow!("No video segments found"))?;

    let segment_url = format!("{}{}", chunks_base_url, segment);

    // Download segment
    let segment_data = reqwest::blocking::get(&segment_url)?.bytes()?;

    // Save to temporary file
    let mut temp_file = NamedTempFile::new()?;
    std::io::copy(&mut segment_data.as_ref(), &mut temp_file)?;

    let temp_dir = tempfile::tempdir()?;
    let output_path = temp_dir.path().join("frame.jpg");
    let output_path_str = output_path.to_str().unwrap();

    let ffmpeg_output = Command::new("ffmpeg")
        .args(&[
            "-i",
            temp_file.path().to_str().unwrap(),
            "-vframes",
            "1",
            "-y",
            output_path_str,
        ])
        .output()?;

    if !ffmpeg_output.status.success() {
        return Err(anyhow!(
            "FFmpeg failed: {}",
            String::from_utf8_lossy(&ffmpeg_output.stderr)
        ));
    }

    // Check if the file exists and has content
    if fs::metadata(output_path_str)?.len() == 0 {
        return Err(anyhow!("FFmpeg produced an empty output file"));
    }

    // Load the image
    let img = image::open(output_path_str)?.to_rgb8();

    Ok(img)
}

fn add_gaussian_noise(img: &GrayImage, mean: f64, std_dev: f64) -> RgbImage {
    let width = img.width();
    let height = img.height();

    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::thread_rng();

    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0[0] as f64;
            let noise = normal.sample(&mut rng);
            let noisy_value = (pixel + noise).max(0.0).min(255.0) as u8;
            noisy_img.put_pixel(x, y, Rgb([noisy_value, noisy_value, noisy_value]));
        }
    }

    noisy_img
}

fn set_wallpaper(path: &Path) -> Result<()> {
    Command::new("feh")
        .args(&["--bg-scale", path.to_str().unwrap()])
        .output()?;

    Ok(())
}
