use anyhow::{anyhow, Result};
use chrono::Local;
use ffmpeg_next as ffmpeg;
use image::{GrayImage, Rgb, RgbImage};
use m3u8_rs::Playlist;
use rand_distr::{Distribution, Normal};
use regex::Regex;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;

    let image = get_first_frame()?;

    let filename = Local::now().format("%Y%m%d-%H%M%S.jpg").to_string();
    let output_path = cache_dir.join(filename);

    let gray_image = image::imageops::grayscale(&image);
    let noisy_image = add_gaussian_noise(&gray_image, 0.0, 35.0);

    noisy_image.save(&output_path)?;
    set_wallpaper(&output_path)?;

    Ok(())
}

fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| anyhow!("Could not determine cache directory"))?
        .join(env!("CARGO_PKG_NAME"));
    Ok(cache_dir)
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
    ffmpeg::init()?;

    let m3u8_url = get_current_stream_url()?;
    let response = reqwest::blocking::get(&m3u8_url)?.text()?;

    let base_url = m3u8_url
        .rsplit_once('/')
        .map(|(base, _)| format!("{}/", base))
        .unwrap_or_default();

    let playlist = m3u8_rs::parse_playlist_res(response.as_bytes())
        .map_err(|e| anyhow!("Failed to parse m3u8: {:?}", e))?;

    let chunks_playlist_url = match playlist {
        Playlist::MasterPlaylist(master) => {
            let variant = master
                .variants
                .first()
                .ok_or_else(|| anyhow!("No variants in master playlist"))?;
            format!("{}{}", base_url, variant.uri)
        }
        Playlist::MediaPlaylist(_) => m3u8_url,
    };

    let chunks_response = reqwest::blocking::get(&chunks_playlist_url)?.text()?;
    let chunks_base_url = chunks_playlist_url
        .rsplit_once('/')
        .map(|(base, _)| format!("{}/", base))
        .unwrap_or_default();

    let media_playlist = match m3u8_rs::parse_playlist_res(chunks_response.as_bytes())
        .map_err(|e| anyhow!("Failed to parse media playlist: {:?}", e))?
    {
        Playlist::MediaPlaylist(media) => media,
        _ => return Err(anyhow!("Expected media playlist")),
    };

    let segment = media_playlist
        .segments
        .first()
        .ok_or_else(|| anyhow!("No segments in playlist"))?;

    let segment_url = format!("{}{}", chunks_base_url, segment.uri);
    let segment_data = reqwest::blocking::get(&segment_url)?.bytes()?;

    let mut temp_file = tempfile::NamedTempFile::new()?;
    std::io::copy(&mut Cursor::new(segment_data), &mut temp_file)?;
    let temp_path = temp_file.path();

    let mut input_ctx = ffmpeg::format::input(temp_path)?;
    let input_stream = input_ctx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or_else(|| anyhow!("No video stream found"))?;
    let stream_index = input_stream.index();

    let mut decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?
        .decoder()
        .video()?;

    let mut scaler = ffmpeg::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flag::Flags::BILINEAR,
    )?;

    let mut frame = ffmpeg::frame::Video::empty();

    for (stream, packet) in input_ctx.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet)?;
            if decoder.receive_frame(&mut frame).is_ok() {
                let mut rgb_frame = ffmpeg::frame::Video::new(
                    ffmpeg::format::Pixel::RGB24,
                    frame.width(),
                    frame.height(),
                );
                scaler.run(&frame, &mut rgb_frame)?;

                let width = rgb_frame.width() as u32;
                let height = rgb_frame.height() as u32;
                let data = rgb_frame.data(0).to_vec();

                let img = RgbImage::from_raw(width, height, data)
                    .ok_or_else(|| anyhow!("Failed to create image from raw data"))?;

                return Ok(img);
            }
        }
    }

    Err(anyhow!("No frames decoded"))
}

fn add_gaussian_noise(img: &GrayImage, mean: f64, std_dev: f64) -> RgbImage {
    let width = img.width();
    let height = img.height();

    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::rng();

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
    wallpaper::set_from_path(path.to_str().unwrap())
        .map_err(|e| anyhow!("Failed to set wallpaper: {}", e))?;

    Ok(())
}
