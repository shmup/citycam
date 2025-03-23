use crate::cli::NoiseType;

pub struct CityCam {
    pub message: String,
    pub is_grayscale: bool,
    pub noise_intensity: f64,
    pub noise_type: Option<NoiseType>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CheckboxToggled(bool),
    NoiseIntensityChanged(f64),
    ApplyWallpaper,
    NoiseTypeSelected(Option<NoiseType>)
}
