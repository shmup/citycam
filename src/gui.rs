use iced::widget::{center, checkbox, column, scrollable, text, vertical_space};
use iced::{Center, Element, Fill};

pub fn run_gui() -> iced::Result {
    iced::run("citycam", CityCam::update, CityCam::view)
}

struct CityCam {
    message: String,
    is_grayscale: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CheckboxToggled(bool),
}

impl CityCam {
    fn new() -> Self {
        Self {
            message: String::new(),
            is_grayscale: false,
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
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            checkbox("Grayscale", self.is_grayscale).on_toggle(Message::CheckboxToggled),
            vertical_space().height(150),
            text(&self.message),
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
