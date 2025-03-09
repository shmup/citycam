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
