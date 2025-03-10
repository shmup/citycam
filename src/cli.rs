use clap::{Parser, ValueEnum};

/// A tool to process webcam images and set them as wallpaper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Convert image to grayscale
    #[arg(short, long)]
    pub grayscale: bool,

    /// Apply color to sky based on time of day
    #[arg(short, long)]
    pub color_sky: bool,

    /// Type of noise to apply to the image
    #[arg(short, long, value_enum)]
    pub noise: Option<NoiseType>,

    /// Noise intensity (0-255 for Salt/Pepper, standard deviation for Gaussian)
    #[arg(short = 'i', long, default_value_t = 25.0)]
    pub noise_intensity: f64,

    /// Skip caching the image
    #[arg(long = "skip-cache")]
    pub skip_cache: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum NoiseType {
    /// Add Gaussian noise to the image
    Gaussian,
    /// Add Salt and Pepper noise to the image
    SaltPepper,
    /// Add Poisson noise to the image
    Poisson,
}
