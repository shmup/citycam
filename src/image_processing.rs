use image::{GrayImage, Rgb, RgbImage};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

pub fn convert_grayscale_to_rgb(img: &GrayImage) -> RgbImage {
    let (width, height) = img.dimensions();
    let mut rgb_img = RgbImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let gray_val = pixel.0[0];
        rgb_img.put_pixel(x, y, Rgb([gray_val, gray_val, gray_val]));
    }

    rgb_img
}

pub fn add_gaussian_noise_to_rgb(img: &RgbImage, mean: f64, std_dev: f64) -> RgbImage {
    let width = img.width();
    let height = img.height();

    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::rng();

    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Add noise to each channel
            let r = ((pixel[0] as f64) + normal.sample(&mut rng))
                .max(0.0)
                .min(255.0) as u8;
            let g = ((pixel[1] as f64) + normal.sample(&mut rng))
                .max(0.0)
                .min(255.0) as u8;
            let b = ((pixel[2] as f64) + normal.sample(&mut rng))
                .max(0.0)
                .min(255.0) as u8;

            noisy_img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    noisy_img
}

pub fn add_salt_and_pepper_noise_to_rgb(img: &RgbImage, density: f64) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut rng = rand::rng();
    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let r: f64 = rng.random();
            if r < density / 2.0 {
                // Salt
                noisy_img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else if r < density {
                // Pepper
                noisy_img.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {
                noisy_img.put_pixel(x, y, *pixel);
            }
        }
    }

    noisy_img
}

pub fn add_poisson_noise_to_rgb(img: &RgbImage) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut rng = rand::rng();
    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Process each channel
            let r_lambda = pixel[0] as f64;
            let g_lambda = pixel[1] as f64;
            let b_lambda = pixel[2] as f64;

            // Generate Poisson random numbers for each channel
            let r_poisson = rand_distr::Poisson::new(r_lambda.max(1.0)).unwrap();
            let g_poisson = rand_distr::Poisson::new(g_lambda.max(1.0)).unwrap();
            let b_poisson = rand_distr::Poisson::new(b_lambda.max(1.0)).unwrap();

            let r = (r_poisson.sample(&mut rng) as f64).max(0.0).min(255.0) as u8;
            let g = (g_poisson.sample(&mut rng) as f64).max(0.0).min(255.0) as u8;
            let b = (b_poisson.sample(&mut rng) as f64).max(0.0).min(255.0) as u8;

            noisy_img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    noisy_img
}

pub fn apply_tint_to_rgb(
    img: &RgbImage,
    tint_color: &str, // Hex color like "#FF5500"
    intensity: f32,   // Value between 0.0 (no effect) and 1.0 (full tint)
) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut tinted_img = RgbImage::new(width, height);

    // Parse hex color
    let tint_rgb = hex_to_rgb(tint_color);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Blend original pixel with tint color based on intensity
            let new_r = blend(pixel[0], tint_rgb[0], intensity);
            let new_g = blend(pixel[1], tint_rgb[1], intensity);
            let new_b = blend(pixel[2], tint_rgb[2], intensity);

            tinted_img.put_pixel(x, y, Rgb([new_r, new_g, new_b]));
        }
    }

    tinted_img
}

fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    [r, g, b]
}

fn blend(original: u8, tint: u8, intensity: f32) -> u8 {
    ((original as f32) * (1.0 - intensity) + (tint as f32) * intensity) as u8
}
