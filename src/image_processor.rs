use anyhow::Result;
use chrono::Local;
use image::RgbImage;
use std::path::Path;

use crate::cli;
use crate::image_processing;
use crate::sky_detection;
use crate::utils;

pub fn process_and_set_wallpaper(
    original_image: RgbImage,
    args: &cli::Args,
    cache_dir: &Path,
) -> Result<()> {
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
