pub struct CityCam {
    pub message: String,
    pub is_grayscale: bool,
    pub noise_intensity: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CheckboxToggled(bool),
    NoiseIntensityChanged(f64),
    ApplyWallpaper,
}
