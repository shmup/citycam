use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum NoiseType {
    /// Add Gaussian noise to the image
    Gaussian,
    /// Add Salt and Pepper noise to the image
    SaltPepper,
    /// Add Poisson noise to the image
    Poisson,
}

impl std::fmt::Display for NoiseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Gaussian => "Gaussian noise",
            Self::SaltPepper => "Salt & Pepper noise",
            Self::Poisson => "Poisson noise",
        })
    }
}
