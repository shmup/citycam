// ABOUTME: gallery view component that displays all camera feeds in a grid
// ABOUTME: allows monitoring multiple camera feeds simultaneously

use iced::widget::{button, checkbox, container, row, slider, text, Column, Row};
use iced::{Element, Length};

use crate::gui::components::camera_feed::camera_feed_view_with_size;
use crate::gui::types::{CameraFeed, CityCam, Message};

pub fn gallery_view(citycam: &CityCam) -> Element<'_, Message> {
    let feeds = &citycam.camera_feeds;
    let window_size = citycam.window_size;
    // Calculate optimal layout based on actual window size
    let num_cameras = feeds.len();
    let available_width = window_size.width;
    let available_height = window_size.height - 50.0; // Space for header

    // Adaptive column count based on window width - more responsive breakpoints
    let cols = if available_width < 500.0 {
        1 // Single column on very small screens
    } else if available_width < 800.0 {
        2 // Two columns on small-medium screens
    } else if available_width < 1200.0 {
        3 // Three columns on medium screens
    } else if available_width < 1600.0 {
        4 // Four columns on large screens
    } else {
        5 // Five columns on very large screens
    };

    let rows = (num_cameras as f32 / cols as f32).ceil();

    // Calculate card dimensions to fit the window perfectly without scrolling
    let _card_width = (available_width - 40.0) / cols as f32; // Account for padding/spacing
    let total_grid_height = available_height - 60.0; // Account for header and spacing
    let card_height = if rows > 0.0 {
        (total_grid_height / rows).max(80.0).min(300.0) // Ensure reasonable bounds
    } else {
        150.0
    };

    // Compact header
    let header = row![
        text("Camera Gallery").size(18),
        button("Config")
            .on_press(Message::SwitchToConfig)
            .padding(8),
        button("Refresh").on_press(Message::RefreshFeeds).padding(8),
        checkbox("Auto-refresh", citycam.auto_refresh_enabled)
            .on_toggle(Message::AutoRefreshToggled),
        text(format!("{}s", citycam.auto_refresh_interval)),
        slider(
            5.0..=60.0,
            citycam.auto_refresh_interval as f64,
            Message::AutoRefreshIntervalChanged
        )
        .width(100),
    ]
    .spacing(15)
    .align_y(iced::Alignment::Center);

    let mut content = Column::new().spacing(10).push(header);

    if feeds.is_empty() {
        content = content.push(
            container(text("No cameras configured").size(16))
                .width(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        );
    } else {
        // Create responsive grid layout that fills the window
        let mut grid = Column::new().spacing(8);
        let mut current_row = Row::new().spacing(8);

        for (index, feed) in feeds.iter().enumerate() {
            current_row = current_row.push(
                container(camera_feed_view_with_size(feed, card_height))
                    .width(Length::FillPortion(1)),
            );

            // Start a new row after every `cols` cameras
            if (index + 1) % cols == 0 {
                grid = grid.push(current_row);
                current_row = Row::new().spacing(8);
            }
        }

        // Add any remaining cameras in the last row
        if feeds.len() % cols != 0 {
            grid = grid.push(current_row);
        }

        // Add grid without scrollable wrapper to fit window exactly
        content = content.push(container(grid).width(Length::Fill).height(Length::Fill));
    }

    container(content)
        .padding(15)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
