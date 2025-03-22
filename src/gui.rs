use iced::widget::{button, center, checkbox, column, scrollable, slider, text, vertical_space};
use iced::{Center, Element, Fill};

pub fn run_gui() -> iced::Result {
    iced::run("citycam", CityCam::update, CityCam::view)
}

struct CityCam {
    message: String,
    is_grayscale: bool,
    noise_intensity: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CheckboxToggled(bool),
    NoiseIntensityChanged(f32),
    ApplyWallpaper,
}

impl CityCam {
    fn new() -> Self {
        Self {
            message: String::new(),
            is_grayscale: false,
            noise_intensity: 25.0,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CheckboxToggled(is_checked) => {
                self.is_grayscale = is_checked;
                if self.is_grayscale {
                    self.message = "Grayscale mode enabled".to_string();
                } else {
                    self.message = "Grayscale mode disabled".to_string();
                }
            }
            Message::NoiseIntensityChanged(value) => {
                self.noise_intensity = value;
                self.message = format!("Noise intensity set to: {:.2}", value);
            }
            Message::ApplyWallpaper => {
                self.message = "Applying wallpaper to desktop...".to_string();

                let is_grayscale = self.is_grayscale;
                let noise_intensity = self.noise_intensity;

                std::thread::spawn(move || {
                    let args = crate::cli::Args {
                        grayscale: is_grayscale,
                        color_sky: false,
                        noise: Some(crate::cli::NoiseType::Gaussian),
                        noise_intensity: noise_intensity as f64,
                        skip_cache: false,
                        camera: Some("Traverse City".to_string()),
                        cams_file: None,
                        rotate: false,
                        rotation_interval: 30,
                        tint_color: None,
                        tint_intensity: 0.5,
                        gui: false,
                    };

                    let cache_dir = match crate::utils::get_cache_dir() {
                        Ok(dir) => dir,
                        Err(_) => return,
                    };

                    let cameras = match crate::camera::get_embedded_cameras() {
                        Ok(cams) => cams,
                        Err(_) => return,
                    };

                    let camera = match crate::camera::find_camera(&cameras, "Traverse City") {
                        Ok(cam) => cam,
                        Err(_) => return,
                    };

                    if let Ok(image) = crate::stream::get_first_frame(&camera) {
                        let _ = crate::image_processor::process_and_set_wallpaper(
                            image, &args, &cache_dir,
                        );
                    }
                });
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            checkbox("Grayscale", self.is_grayscale).on_toggle(Message::CheckboxToggled),
            slider(
                0.0..=100.0,
                self.noise_intensity,
                Message::NoiseIntensityChanged,
            ),
            vertical_space().height(150),
            text(&self.message),
            button("Apply Wallpaper").on_press(Message::ApplyWallpaper),
        ]
        .width(Fill)
        .align_x(Center)
        .spacing(10);

        center(scrollable(content)).into()
    }
}

impl Default for CityCam {
    fn default() -> Self {
        CityCam::new()
    }
}
