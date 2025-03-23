use crate::cli::NoiseType;
use crate::gui::types::Message;
use iced::widget::{column, radio, text};
use iced::Element;

pub fn view(selected: Option<NoiseType>) -> Element<'static, Message> {
    let none_radio = radio(
        "No noise",
        None::<NoiseType>,
        selected.map(Some),
        Message::NoiseTypeSelected,
    );

    let gaussian_radio = radio(
        "Gaussian noise",
        Some(NoiseType::Gaussian),
        selected.map(Some),
        Message::NoiseTypeSelected,
    );

    let salt_pepper_radio = radio(
        "Salt & Pepper noise",
        Some(NoiseType::SaltPepper),
        selected.map(Some),
        Message::NoiseTypeSelected,
    );

    let poisson_radio = radio(
        "Poisson noise",
        Some(NoiseType::Poisson),
        selected.map(Some),
        Message::NoiseTypeSelected,
    );

    column![
        text("Noise Type:").size(16),
        none_radio,
        gaussian_radio,
        salt_pepper_radio,
        poisson_radio,
    ]
    .spacing(5)
    .into()
}
