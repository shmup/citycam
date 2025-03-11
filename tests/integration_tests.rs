use citycam::sky_detection::{apply_sky_color_with_gradient, detect_sky_region_growing};
use image::{GrayImage, Rgb, RgbImage};

#[test]
fn test_sky_detection_simple_gradient() {
    // Create a simple gradient image (lighter at top, darker at bottom)
    let mut gray_img = GrayImage::new(10, 10);
    for y in 0..10 {
        let value = 200 - (y * 20) as u8; // 200 at top, 20 at bottom
        for x in 0..10 {
            gray_img.put_pixel(x, y, image::Luma([value]));
        }
    }

    let sky_mask = detect_sky_region_growing(&gray_img);

    // Top rows should be detected as sky
    assert!(sky_mask[0][5], "Top row should be detected as sky");
    assert!(sky_mask[1][5], "Second row should be detected as sky");

    // Bottom rows should not be sky
    assert!(!sky_mask[9][5], "Bottom row should not be detected as sky");
}

#[test]
fn test_sky_color_application() {
    // Create a test image
    let mut img = RgbImage::new(10, 10);
    for x in 0..10 {
        for y in 0..10 {
            img.put_pixel(x, y, Rgb([100, 100, 100]));
        }
    }

    // Create a sky mask where top half is sky
    let mut sky_mask = vec![vec![false; 10]; 10];
    for y in 0..5 {
        for x in 0..10 {
            sky_mask[y][x] = true;
        }
    }

    let sky_color = Rgb([200, 150, 100]);
    let result = apply_sky_color_with_gradient(&img, &sky_mask, sky_color);

    // Check top pixels have sky color
    let top_pixel = result.get_pixel(5, 0);
    assert_eq!(top_pixel.0[0], 200, "Red channel should match sky color");
    assert_eq!(top_pixel.0[1], 150, "Green channel should match sky color");
    assert_eq!(top_pixel.0[2], 100, "Blue channel should match sky color");

    // Check bottom pixels are unchanged
    let bottom_pixel = result.get_pixel(5, 9);
    assert_eq!(
        bottom_pixel.0,
        [100, 100, 100],
        "Bottom pixels should be unchanged"
    );
}
