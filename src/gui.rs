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
