use rand::prelude::*;
use image::{GrayImage, Rgb, RgbImage};
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

pub fn add_gaussian_noise(img: &GrayImage, mean: f64, std_dev: f64) -> RgbImage {
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

pub fn add_salt_and_pepper_noise(img: &GrayImage, density: f64) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut rng = rand::rng();
    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0[0];

            let r: f64 = rng.random();
            let noisy_value = if r < density / 2.0 {
                0 // salt
            } else if r < density {
                255 // pepper
            } else {
                pixel
            };

            noisy_img.put_pixel(x, y, Rgb([noisy_value, noisy_value, noisy_value]));
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

pub fn add_poisson_noise(img: &GrayImage) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut rng = rand::rng();
    let mut noisy_img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_value = img.get_pixel(x, y).0[0] as f64;

            // Scale factor for Poisson noise
            let lambda = pixel_value.max(1.0);

            // Generate Poisson random number
            let poisson = rand_distr::Poisson::new(lambda).unwrap();
            let noise = poisson.sample(&mut rng) as f64;

            // Scale back to 0-255 range
            let noisy_value = (noise).max(0.0).min(255.0) as u8;

            noisy_img.put_pixel(x, y, Rgb([noisy_value, noisy_value, noisy_value]));
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
