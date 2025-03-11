use image::{GrayImage, Rgb, RgbImage};
use citycam::image_processing::{
    add_gaussian_noise_to_rgb, add_salt_and_pepper_noise_to_rgb,
    convert_grayscale_to_rgb,
};

#[test]
fn test_convert_grayscale_to_rgb() {
    let mut gray_img = GrayImage::new(3, 3);
    for x in 0..3 {
        for y in 0..3 {
            gray_img.put_pixel(x, y, image::Luma([100]));
        }
    }

    let rgb_img = convert_grayscale_to_rgb(&gray_img);

    for x in 0..3 {
        for y in 0..3 {
            let pixel = rgb_img.get_pixel(x, y);
            assert_eq!(pixel.0, [100, 100, 100]);
        }
    }
}

#[test]
fn test_gaussian_noise_changes_image() {
    let mut rgb_img = RgbImage::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            rgb_img.put_pixel(x, y, Rgb([128, 128, 128]));
        }
    }

    let noisy_img = add_gaussian_noise_to_rgb(&rgb_img, 0.0, 50.0);

    // Check that pixels have changed
    let mut all_same = true;
    for x in 0..10 {
        for y in 0..10 {
            let original = rgb_img.get_pixel(x, y);
            let noisy = noisy_img.get_pixel(x, y);
            if original.0 != noisy.0 {
                all_same = false;
                break;
            }
        }
    }

    assert!(!all_same, "Gaussian noise should change pixel values");
}

#[test]
fn test_salt_and_pepper_noise() {
    let mut rgb_img = RgbImage::new(20, 20);
    for x in 0..20 {
        for y in 0..20 {
            rgb_img.put_pixel(x, y, Rgb([128, 128, 128]));
        }
    }

    let noisy_img = add_salt_and_pepper_noise_to_rgb(&rgb_img, 0.5);

    // Count salt (255) and pepper (0) pixels
    let mut salt_count = 0;
    let mut pepper_count = 0;

    for x in 0..20 {
        for y in 0..20 {
            let pixel = noisy_img.get_pixel(x, y);
            if pixel.0 == [255, 255, 255] {
                salt_count += 1;
            } else if pixel.0 == [0, 0, 0] {
                pepper_count += 1;
            }
        }
    }

    // With density 0.5, roughly 25% should be salt and 25% pepper
    assert!(salt_count > 0, "Should have some salt pixels");
    assert!(pepper_count > 0, "Should have some pepper pixels");
    assert!(salt_count + pepper_count > 0, "Should have noise applied");
}
