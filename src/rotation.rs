use anyhow::Result;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::camera::Camera;
use crate::cli;
use crate::image_processor;
use crate::stream;

pub fn start_rotation(cameras: Vec<Camera>, args: &cli::Args, cache_dir: &Path) -> Result<()> {
    println!(
        "Starting camera rotation with interval of {} seconds",
        args.rotation_interval
    );

    let mut current_index = 0;

    loop {
        let camera = &cameras[current_index];
        println!("Rotating to camera: {}", camera.name);

        match stream::get_first_frame(camera) {
            Ok(original_image) => {
                if let Err(e) =
                    image_processor::process_and_set_wallpaper(original_image, args, cache_dir)
                {
                    eprintln!("Failed to process image for {}: {}", camera.name, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to get frame from {}: {}", camera.name, e);
            }
        }

        current_index = (current_index + 1) % cameras.len();
        thread::sleep(Duration::from_secs(args.rotation_interval));
    }
}
