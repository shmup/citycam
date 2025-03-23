use iced::widget::{button, center, checkbox, column, scrollable, slider, text, vertical_space};
use iced::{Center, Element, Fill};

mod components;

mod types;
use types::CityCam;
use types::Message;

pub fn run_gui() -> iced::Result {
    iced::application("citycam", CityCam::update, CityCam::view)
        .window_size([600.0, 480.0])
        .centered()
        .theme(|_| iced::Theme::TokyoNight)
        .scale_factor(|_| 0.9)
        .run()
}

impl CityCam {
    fn new() -> Self {
        let args = crate::cli::Args::default();

        Self {
            message: String::new(),
            is_grayscale: args.grayscale,
            noise_intensity: args.noise_intensity,
            noise_type: args.noise,
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
            Message::NoiseTypeSelected(noise_type) => {
                self.noise_type = noise_type;
                match noise_type {
                    Some(noise_type) => {
                        self.message = format!("Noise type set to: {:?}", noise_type);
                    }
                    None => {
                        self.message = "No noise will be applied".to_string()
                    }
                }
            }
            Message::ApplyWallpaper => {
                self.message = "Applying wallpaper to desktop...".to_string();

                let is_grayscale = self.is_grayscale;
                let noise_intensity = self.noise_intensity;
                let noise_type = self.noise_type;

                std::thread::spawn(move || {
                    let mut args = crate::cli::Args::default();

                    args.grayscale = is_grayscale;
                    args.noise_intensity = noise_intensity;
                    args.noise = noise_type;

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
            checkbox("Grayscale", self.is_grayscale).on_toggle(Message::GrayscaleToggled),
            components::noise_selector_view(self.noise_type),
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
