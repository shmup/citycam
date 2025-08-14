use crate::camera::Camera;
use crate::types::NoiseType;

pub struct CityCam {
    pub message: String,
    pub is_grayscale: bool,
    pub noise_intensity: f64,
    pub noise_type: Option<NoiseType>,
    pub current_view: View,
    pub camera_feeds: Vec<CameraFeed>,
    pub window_size: iced::Size,
    pub auto_refresh_interval: u64, // seconds
    pub auto_refresh_enabled: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    GrayscaleToggled(bool),
    NoiseIntensityChanged(f64),
    ApplyWallpaper,
    NoiseTypeSelected(Option<NoiseType>),
    SwitchToGallery,
    SwitchToConfig,
    RefreshFeeds,
    ImageLoaded(usize, Result<Vec<u8>, String>),
    WindowResized(iced::Size),
    AutoRefreshToggled(bool),
    AutoRefreshIntervalChanged(f64),
    AutoRefreshTick,
    FeedClicked(usize), // Index of the clicked camera feed
}
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Config,
    Gallery,
    FocusedFeed(usize), // Index of the focused camera feed
}

#[derive(Debug, Clone)]
pub struct CameraFeed {
    pub camera: Camera,
    pub last_image: Option<iced::widget::image::Handle>,
    pub last_update: std::time::Instant,
    pub error: Option<String>,
}
