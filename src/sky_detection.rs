use chrono::{Local, Timelike};
use image::{GrayImage, Rgb, RgbImage};

pub fn get_sky_color_for_time() -> Rgb<u8> {
    let now = Local::now();
    let hour = now.hour();

    match hour {
        // Night (10 PM - 5 AM)
        22..=23 | 0..=4 => Rgb([40, 45, 70]), // Muted dark blue for historical night sky

        // Dawn/Dusk (5-7 AM, 6-9 PM)
        5..=6 | 18..=21 => {
            // Softer, more muted orange/amber for historical sunrise/sunset
            let intensity = if hour <= 6 {
                // Dawn: increasing intensity
                (hour - 5) as f32 / 2.0
            } else {
                // Dusk: decreasing intensity
                (22 - hour) as f32 / 4.0
            };

            let r = (205.0 * intensity + 70.0 * (1.0 - intensity)) as u8;
            let g = (140.0 * intensity + 80.0 * (1.0 - intensity)) as u8;
            let b = (110.0 * intensity + 120.0 * (1.0 - intensity)) as u8;

            Rgb([r, g, b])
        }

        // Morning (7-10 AM)
        7..=9 => Rgb([170, 190, 215]), // Softer light blue for historical morning

        // Midday (10 AM - 5 PM)
        10..=17 => Rgb([180, 200, 220]), // Muted sky blue for historical daytime

        // Fallback
        _ => Rgb([180, 200, 220]),
    }
}

pub fn detect_and_color_sky(img: &GrayImage, sky_color: Rgb<u8>) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut result = RgbImage::new(width, height);

    // Convert grayscale to RGB first
    for y in 0..height {
        for x in 0..width {
            let gray_val = img.get_pixel(x, y).0[0];
            result.put_pixel(x, y, Rgb([gray_val, gray_val, gray_val]));
        }
    }

    // Create a gradient map to help identify sky regions
    let gradients = calculate_gradients(img);

    // Apply sky coloring
    color_sky_regions(img, &gradients, &mut result, sky_color);

    result
}

fn calculate_gradients(img: &GrayImage) -> Vec<Vec<f32>> {
    let width = img.width();
    let height = img.height();
    let mut gradients = vec![vec![0.0f32; width as usize]; height as usize];

    // Calculate vertical gradients
    for y in 1..height {
        for x in 0..width {
            let pixel_above = img.get_pixel(x, y - 1).0[0] as f32;
            let pixel_current = img.get_pixel(x, y).0[0] as f32;
            let gradient = (pixel_current - pixel_above).abs();
            gradients[y as usize][x as usize] = gradient;
        }
    }

    gradients
}

fn color_sky_regions(
    img: &GrayImage,
    gradients: &Vec<Vec<f32>>,
    result: &mut RgbImage,
    sky_color: Rgb<u8>,
) {
    let width = img.width();
    let height = img.height();

    // Thresholds for sky detection
    let brightness_threshold = 130;
    let gradient_threshold = 5.0;

    // Start from top and work down - sky is typically at the top
    for x in 0..width {
        let mut sky_bottom = 0;

        // Find where sky ends for this column
        for y in 0..height / 2 {
            // Only search top half of image
            let pixel = img.get_pixel(x, y).0[0];
            let gradient = gradients[y as usize][x as usize];

            // If we hit a significant gradient change or dark pixel, mark end of sky
            if pixel < brightness_threshold || gradient > gradient_threshold {
                break;
            }
            sky_bottom = y;
        }

        // Color the sky in this column
        for y in 0..=sky_bottom {
            let pixel = img.get_pixel(x, y);
            let gray_val = pixel.0[0] as f32 / 255.0;

            // Blend with sky color - stronger at top, gradually fading
            let blend_factor = 0.8 - (y as f32 / sky_bottom as f32) * 0.3;
            let r = (sky_color.0[0] as f32 * blend_factor + gray_val * 255.0 * (1.0 - blend_factor))
                as u8;
            let g = (sky_color.0[1] as f32 * blend_factor + gray_val * 255.0 * (1.0 - blend_factor))
                as u8;
            let b = (sky_color.0[2] as f32 * blend_factor + gray_val * 255.0 * (1.0 - blend_factor))
                as u8;

            result.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
}

// New sky detection approach using region growing
pub fn detect_sky_region_growing(img: &GrayImage) -> Vec<Vec<bool>> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut sky_mask = vec![vec![false; width]; height];

    // Start seeds from top row
    let mut seeds = Vec::new();
    for x in 0..width {
        if img.get_pixel(x as u32, 0).0[0] > 120 {
            seeds.push((x, 0));
            sky_mask[0][x] = true;
        }
    }

    // Region growing
    while !seeds.is_empty() {
        let (x, y) = seeds.pop().unwrap();
        let pixel_val = img.get_pixel(x as u32, y as u32).0[0];

        // Check 4-connected neighbors
        for (dx, dy) in &[(0, 1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if !sky_mask[ny][nx] {
                    let neighbor_val = img.get_pixel(nx as u32, ny as u32).0[0];
                    if (neighbor_val as i32 - pixel_val as i32).abs() < 15 && neighbor_val > 100 {
                        sky_mask[ny][nx] = true;
                        seeds.push((nx, ny));
                    }
                }
            }
        }
    }

    // Apply horizontal smoothing
    smooth_sky_boundary(&mut sky_mask);

    sky_mask
}

// Smooth the sky boundary
fn smooth_sky_boundary(sky_mask: &mut Vec<Vec<bool>>) {
    let width = sky_mask[0].len();
    let height = sky_mask.len();

    // Find the lowest sky pixel in each column
    let mut sky_bottom = vec![0; width];
    for x in 0..width {
        for y in 0..height {
            if sky_mask[y][x] {
                sky_bottom[x] = y;
            }
        }
    }

    // Apply median filter to smooth boundary
    let window_size = 7;
    let half_window = window_size / 2;
    let mut smoothed_bottom = vec![0; width];

    for x in 0..width {
        let mut window = Vec::new();
        for wx in x.saturating_sub(half_window)..std::cmp::min(x + half_window + 1, width) {
            window.push(sky_bottom[wx]);
        }
        window.sort();
        smoothed_bottom[x] = window[window.len() / 2]; // median
    }

    // Update mask with smoothed boundary
    for x in 0..width {
        for y in 0..height {
            sky_mask[y][x] = y <= smoothed_bottom[x];
        }
    }
}

// Gradient-based sky detection with color information
pub fn detect_sky_with_color(img: &RgbImage) -> Vec<Vec<bool>> {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut sky_mask = vec![vec![false; width]; height];

    // Calculate sky probability for each pixel
    let mut sky_prob = vec![vec![0.0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x as u32, y as u32).0;

            // Sky tends to be more blue than red or green
            let blue_bias = pixel[2] as f32 / (pixel[0] as f32 + pixel[1] as f32 + 1.0);

            // Sky tends to be bright
            let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;

            // Sky tends to be at the top
            let vertical_bias = 1.0 - (y as f32 / (height as f32 / 2.0)).min(1.0);

            // Combine factors
            sky_prob[y][x] = blue_bias * 0.4 + (brightness / 255.0) * 0.3 + vertical_bias * 0.3;
        }
    }

    // Threshold with hysteresis (similar to Canny edge detection)
    let high_threshold = 0.6;
    let low_threshold = 0.4;

    // First pass: mark strong sky pixels
    for y in 0..height {
        for x in 0..width {
            if sky_prob[y][x] > high_threshold {
                sky_mask[y][x] = true;
            }
        }
    }

    // Second pass: grow regions from strong sky pixels
    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..height {
            for x in 0..width {
                if !sky_mask[y][x] && sky_prob[y][x] > low_threshold {
                    // Check if any neighbor is sky
                    for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            if sky_mask[ny as usize][nx as usize] {
                                sky_mask[y][x] = true;
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    sky_mask
}

// Apply a gradient blend at sky boundary for smoother transition
pub fn apply_sky_color_with_gradient(
    img: &RgbImage,
    sky_mask: &Vec<Vec<bool>>,
    sky_color: Rgb<u8>,
) -> RgbImage {
    let width = img.width();
    let height = img.height();
    let mut result = img.clone();

    // Find transition zone (pixels near boundary)
    let transition_width = 5;
    let mut transition_mask = vec![vec![0.0; width as usize]; height as usize];

    for x in 0..width as usize {
        // Find sky boundary for this column
        let mut sky_bottom = 0;
        for y in 0..height as usize {
            if sky_mask[y][x] {
                sky_bottom = y;
            }
        }

        // Mark transition zone
        for y in 0..height as usize {
            if sky_mask[y][x] {
                // Inside sky region
                transition_mask[y][x] = 1.0;
            } else if y < sky_bottom + transition_width {
                // In transition zone
                let distance = y as i32 - sky_bottom as i32;
                transition_mask[y][x] = 1.0 - (distance as f32 / transition_width as f32);
            }
        }
    }

    // Apply coloring with transition
    for y in 0..height {
        for x in 0..width {
            let blend_factor = transition_mask[y as usize][x as usize];

            if blend_factor > 0.0 {
                let original = img.get_pixel(x, y).0;

                // Blend original with sky color
                let r = (sky_color.0[0] as f32 * blend_factor
                    + original[0] as f32 * (1.0 - blend_factor)) as u8;
                let g = (sky_color.0[1] as f32 * blend_factor
                    + original[1] as f32 * (1.0 - blend_factor)) as u8;
                let b = (sky_color.0[2] as f32 * blend_factor
                    + original[2] as f32 * (1.0 - blend_factor)) as u8;

                result.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
    }

    result
}
