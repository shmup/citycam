use anyhow::Result;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::camera::Camera;
use crate::cli;
use crate::image_processing;
use crate::sky_detection;
use crate::stream;
use crate::utils;

pub fn start_rotation(cameras: Vec<Camera>, args: &cli::Args, cache_dir: &Path) -> Result<()> {
    println!(
        "Starting camera rotation with interval of {} seconds",
        args.rotation_interval
    );

    let mut current_index = 0;

    loop {
        let camera = &cameras[current_index];
        println!("Rotating to camera: {}", camera.name);
        process_camera(camera, args, cache_dir)?;
        current_index = (current_index + 1) % cameras.len();
        thread::sleep(Duration::from_secs(args.rotation_interval));
    }
}

fn process_camera(camera: &Camera, args: &cli::Args, cache_dir: &Path) -> Result<()> {
    let original_image = stream::get_first_frame(camera)?;

    let output_path = if args.skip_cache {
        std::env::temp_dir().join("current_wallpaper.jpg")
    } else {
        let filename = chrono::Local::now().format("%Y%m%d-%H%M%S.jpg").to_string();
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

    if let Some(noise_type) = &args.noise {
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
