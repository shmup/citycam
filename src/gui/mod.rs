use iced::widget::{button, center, checkbox, column, scrollable, slider, text, vertical_space};
use iced::{Center, Element, Fill, Subscription, Task};
use std::time::Instant;

mod components;
use components::focused_view::focused_view;
use components::gallery_view::gallery_view;
use components::noise_radio;

mod types;
use types::{CameraFeed, CityCam, Message, View};

pub fn run_gui() -> iced::Result {
    iced::application("citycam", CityCam::update, CityCam::view)
        .window_size([900.0, 600.0])
        .centered()
        .theme(|_| iced::Theme::TokyoNight)
        .scale_factor(|_| 0.9)
        .subscription(CityCam::subscription)
        .run()
}

impl CityCam {
    fn new() -> Self {
        let args = crate::cli::Args::default();

        // load all available cameras
        let cameras = crate::camera::get_embedded_cameras().unwrap_or_else(|_| Vec::new());

        // initialize camera feeds
        let camera_feeds = cameras
            .iter()
            .map(|camera| CameraFeed {
                camera: camera.clone(),
                last_image: None,
                last_update: Instant::now(),
                error: None,
            })
            .collect();

        Self {
            message: String::new(),
            is_grayscale: args.grayscale,
            noise_intensity: args.noise_intensity,
            noise_type: args.noise,
            current_view: View::Config,
            camera_feeds,
            window_size: iced::Size::new(1200.0, 800.0), // Default size
            auto_refresh_interval: 10,                   // Default 10 seconds
            auto_refresh_enabled: false,
            hide_error_feeds: true, // Default to hiding dead feeds (Unearth unchecked)
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let window_sub =
            iced::window::resize_events().map(|(_, size)| Message::WindowResized(size));

        if self.auto_refresh_enabled {
            let timer_sub =
                iced::time::every(std::time::Duration::from_secs(self.auto_refresh_interval))
                    .map(|_| Message::AutoRefreshTick);
            Subscription::batch([window_sub, timer_sub])
        } else {
            window_sub
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GrayscaleToggled(is_checked) => {
                self.is_grayscale = is_checked;
                if self.is_grayscale {
                    self.message = "Grayscale mode enabled".to_string();
                } else {
                    self.message = "Grayscale mode disabled".to_string();
                }
                Task::none()
            }
            Message::NoiseIntensityChanged(value) => {
                self.noise_intensity = value;
                self.message = format!("Noise intensity set to: {:.2}", value);
                Task::none()
            }
            Message::NoiseTypeSelected(noise_type) => {
                self.noise_type = noise_type;
                match noise_type {
                    Some(noise_type) => {
                        self.message = format!("Noise type set to: {:?}", noise_type);
                    }
                    None => self.message = "No noise will be applied".to_string(),
                }
                Task::none()
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

                    if let Ok(image) = crate::stream::get_first_frame_blocking(&camera) {
                        let _ = crate::image_processor::process_and_set_wallpaper(
                            image, &args, &cache_dir,
                        );
                    }
                });
                Task::none()
            }
            Message::SwitchToGallery => {
                self.current_view = View::Gallery;
                self.refresh_feeds()
            }
            Message::SwitchToConfig => {
                self.current_view = View::Config;
                Task::none()
            }
            Message::RefreshFeeds => self.refresh_feeds(),
            Message::ImageLoaded(index, result) => {
                if let Some(feed) = self.camera_feeds.get_mut(index) {
                    match result {
                        Ok(image_data) => {
                            feed.last_image =
                                Some(iced::widget::image::Handle::from_bytes(image_data));
                            feed.error = None;
                        }
                        Err(error) => {
                            feed.error = Some(error);
                            feed.last_image = None;
                        }
                    }
                    feed.last_update = Instant::now();
                }
                Task::none()
            }
            Message::WindowResized(size) => {
                self.window_size = size;
                Task::none()
            }
            Message::AutoRefreshToggled(enabled) => {
                self.auto_refresh_enabled = enabled;
                Task::none()
            }
            Message::AutoRefreshIntervalChanged(interval) => {
                self.auto_refresh_interval = interval as u64;
                Task::none()
            }
            Message::AutoRefreshTick => {
                if self.auto_refresh_enabled {
                    self.refresh_feeds()
                } else {
                    Task::none()
                }
            }
            Message::FeedClicked(index) => {
                if index < self.camera_feeds.len() {
                    self.current_view = View::FocusedFeed(index);
                }
                Task::none()
            }
            Message::HideErrorFeedsToggled(hide) => {
                self.hide_error_feeds = hide;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_view {
            View::Config => self.config_view(),
            View::Gallery => gallery_view(self),
            View::FocusedFeed(index) => focused_view(self, index),
        }
    }

    fn config_view(&self) -> Element<Message> {
        let content = column![
            button("View Camera Gallery").on_press(Message::SwitchToGallery),
            checkbox("Grayscale", self.is_grayscale).on_toggle(Message::GrayscaleToggled),
            noise_radio::view(self.noise_type),
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

    fn refresh_feeds(&mut self) -> Task<Message> {
        // Clear errors but keep existing images until replaced
        for feed in &mut self.camera_feeds {
            feed.error = None;
            // Don't clear last_image - keep it until we get a new one
        }

        // Create individual tasks that can complete independently
        let mut tasks = Vec::new();

        for (index, feed) in self.camera_feeds.iter().enumerate() {
            let camera = feed.camera.clone();

            let task = Task::perform(
                async move {
                    // Use async version directly since we're already in an async context
                    match crate::stream::get_first_frame(&camera).await {
                        Ok(image) => {
                            let mut buffer = Vec::new();
                            if image
                                .write_to(
                                    &mut std::io::Cursor::new(&mut buffer),
                                    image::ImageFormat::Png,
                                )
                                .is_ok()
                            {
                                (index, Ok(buffer))
                            } else {
                                (index, Err("Failed to encode image".to_string()))
                            }
                        }
                        Err(e) => (index, Err(e.to_string())),
                    }
                },
                |(index, result)| Message::ImageLoaded(index, result),
            );

            tasks.push(task);
        }

        Task::batch(tasks)
    }
}

impl Default for CityCam {
    fn default() -> Self {
        CityCam::new()
    }
}
