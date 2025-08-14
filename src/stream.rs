// ABOUTME: This module handles fetching and decoding video stream frames from cameras
// ABOUTME: It fetches m3u8 playlists and extracts the first frame from video segments

use anyhow::{anyhow, Result};
use ffmpeg_next as ffmpeg;
use image::RgbImage;
use m3u8_rs::Playlist;
use regex::Regex;
use std::io::Cursor;
use std::time::Duration;

use crate::camera::Camera;

pub async fn get_first_frame(camera: &Camera) -> Result<RgbImage> {
    ffmpeg::init()?;
    ffmpeg::log::set_level(ffmpeg::log::Level::Error);

    let m3u8_url = get_current_stream_url(&camera.url).await?;
    let segment_data = fetch_first_segment(&m3u8_url).await?;

    decode_first_frame(&segment_data)
}

async fn get_current_stream_url(frame_url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let response = client.get(frame_url).send().await?.text().await?;

    let re = Regex::new(r"var vurl = '(https://[^']+)'")?;
    if let Some(captures) = re.captures(&response) {
        Ok(captures[1].to_string())
    } else {
        Err(anyhow!(
            "Could not find stream URL in the frame.php response"
        ))
    }
}

async fn fetch_first_segment(m3u8_url: &str) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let response = client.get(m3u8_url).send().await?.text().await?;

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
        Playlist::MediaPlaylist(_) => m3u8_url.to_string(),
    };

    let chunks_response = client.get(&chunks_playlist_url).send().await?.text().await?;
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
    let segment_data = client.get(&segment_url).send().await?.bytes().await?.to_vec();

    Ok(segment_data)
}

fn decode_first_frame(segment_data: &[u8]) -> Result<RgbImage> {
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

pub fn get_first_frame_blocking(camera: &Camera) -> Result<RgbImage> {
    // Create a new tokio runtime for blocking context
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(get_first_frame(camera))
}

pub async fn get_frames_parallel(cameras: &[Camera]) -> Vec<Result<RgbImage>> {
    use futures::future::join_all;
    
    let futures = cameras.iter().map(|camera| get_first_frame(camera));
    join_all(futures).await
}