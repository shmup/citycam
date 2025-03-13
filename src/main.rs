mod camera;
mod cli;
mod image_processing;
mod sky_detection;
mod stream;
mod utils;

use anyhow::{Context, Result};
use chrono::Local;
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

    let selected_camera = match &args.camera {
        Some(selector) => camera::find_camera(&cameras, selector).context(format!(
            "Failed to find camera: {}\n{}",
            selector,
            camera::list_cameras(&cameras)
        ))?,
        None => {
            // default to first camera
            println!("No camera specified, using: {}", cameras[0].name);
            cameras[0].clone()
        }
    };

    println!("Using camera: {}", selected_camera.name);

    let cache_dir = utils::get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;

    let original_image = stream::get_first_frame(&selected_camera)?;

    let output_path = if args.skip_cache {
        std::env::temp_dir().join("current_wallpaper.jpg")
    } else {
        let filename = Local::now().format("%Y%m%d-%H%M%S.jpg").to_string();
        cache_dir.join(filename)
    };

    let mut processed_image = original_image.clone();

    if args.grayscale {
        let gray_image = image::imageops::grayscale(&processed_image);
        processed_image = image_processing::convert_grayscale_to_rgb(&gray_image);
    }

    if args.color_sky {
        let sky_color = sky_detection::get_sky_color_for_time();
        let gray_for_sky = image::imageops::grayscale(&processed_image);
        let sky_mask = sky_detection::detect_sky_region_growing(&gray_for_sky);
        processed_image =
            sky_detection::apply_sky_color_with_gradient(&processed_image, &sky_mask, sky_color);
    }

    if let Some(noise_type) = args.noise {
        match noise_type {
            cli::NoiseType::Gaussian => {
                processed_image = image_processing::add_gaussian_noise_to_rgb(
                    &processed_image,
                    0.0,
                    args.noise_intensity,
                );
            }
            cli::NoiseType::SaltPepper => {
                let density = args.noise_intensity / 255.0;
                processed_image =
                    image_processing::add_salt_and_pepper_noise_to_rgb(&processed_image, density);
            }
            cli::NoiseType::Poisson => {
                processed_image = image_processing::add_poisson_noise_to_rgb(&processed_image);
            }
        }
    }

    processed_image.save(&output_path)?;
    utils::set_wallpaper(&output_path)?;

    Ok(())
}
