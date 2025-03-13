mod camera;
mod cli;
mod image_processing;
mod image_processor;
mod rotation;
mod sky_detection;
mod stream;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let cameras = camera::load_cameras(&args.cams_file).context(format!(
        "Failed to load cameras from {}",
        args.cams_file.display()
    ))?;

    if cameras.is_empty() {
        return Err(anyhow::anyhow!("No cameras found in configuration file"));
    }

    let cache_dir = utils::get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;

    if args.rotate {
        return rotation::start_rotation(cameras, &args, &cache_dir);
    }

    let selected_camera = match &args.camera {
        Some(selector) => camera::find_camera(&cameras, selector).context(format!(
            "Failed to find camera: {}\n{}",
            selector,
            camera::list_cameras(&cameras)
        ))?,
        None => {
            println!("No camera specified, using: {}", cameras[0].name);
            cameras[0].clone()
        }
    };

    println!("Using camera: {}", selected_camera.name);

    let original_image = stream::get_first_frame(&selected_camera)?;
    image_processor::process_and_set_wallpaper(original_image, &args, &cache_dir)
}
