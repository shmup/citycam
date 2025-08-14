// ABOUTME: focused view component that displays a single camera feed in full view
// ABOUTME: allows detailed viewing of a single camera with navigation controls

use iced::widget::{button, container, row, text, Column, Image};
use iced::{Element, Length, Theme};

use crate::gui::types::{CityCam, Message};

pub fn focused_view(citycam: &CityCam, focused_index: usize) -> Element<'_, Message> {
    let feeds = &citycam.camera_feeds;

    if focused_index >= feeds.len() {
        return container(text("Camera not found").size(24))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
    }

    let focused_feed = &feeds[focused_index];
    let window_size = citycam.window_size;

    // Calculate large size for focused view, leaving space for header
    let available_height = window_size.height - 80.0; // Space for header
    let focused_height = available_height * 0.9; // Use most of the available space

    // Header with navigation controls
    let header = row![
        button("← Back to Gallery")
            .on_press(Message::SwitchToGallery)
            .padding(12),
        text(format!("Camera: {}", focused_feed.camera.name)).size(20),
        text(format!("({} of {})", focused_index + 1, feeds.len())).size(16),
    ]
    .spacing(20)
    .align_y(iced::Alignment::Center);

    // Create the focused feed view
    let focused_content = if let Some(handle) = &focused_feed.last_image {
        container(
            Image::new(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fixed(focused_height))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().background.into()),
            border: iced::Border::default().rounded(8.0),
            ..Default::default()
        })
    } else if let Some(error) = &focused_feed.error {
        container(
            text(format!("Error: {}", error))
                .size(18)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().danger),
                }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(focused_height))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().danger.scale_alpha(0.1).into()),
            border: iced::Border::default().rounded(8.0),
            ..Default::default()
        })
    } else {
        container(
            text("Loading camera feed...")
                .size(18)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().primary),
                }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(focused_height))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().background.scale_alpha(0.7).into()),
            border: iced::Border::default().rounded(8.0),
            ..Default::default()
        })
    };

    // Wrap the focused content in a clickable button to go back to gallery
    let clickable_content = button(focused_content)
        .on_press(Message::SwitchToGallery)
        .style(|theme: &Theme, _status| button::Style {
            background: None,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            text_color: theme.palette().text,
        })
        .width(Length::Fill);

    let content = Column::new()
        .spacing(15)
        .push(header)
        .push(clickable_content);

    container(content)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
