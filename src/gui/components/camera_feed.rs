// ABOUTME: camera feed component for displaying a single camera's image
// ABOUTME: shows the camera name, last image, and error state if any

use iced::widget::{button, container, text, Column, Image};
use iced::{Element, Length, Theme};

use crate::gui::types::{CameraFeed, Message};

pub fn camera_feed_view_with_size(
    feed: &CameraFeed,
    card_height: f32,
    feed_index: usize,
) -> Element<'_, Message> {
    // Create the main feed content area - always the same size
    let feed_content = if let Some(handle) = &feed.last_image {
        container(
            Image::new(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().background.into()),
            border: iced::Border::default().rounded(4.0),
            ..Default::default()
        })
    } else if let Some(error) = &feed.error {
        // Simplify error message - extract key information
        let simplified_error = if error.contains("Failed to parse m3u8") {
            "Failed to parse m3u8"
        } else if error.contains("Connection") || error.contains("timeout") {
            "Connection failed"
        } else if error.contains("404") {
            "Camera not found"
        } else if error.contains("403") || error.contains("401") {
            "Access denied"
        } else {
            "Loading failed"
        };

        container(
            text(simplified_error)
                .size(10)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().danger),
                }),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().danger.scale_alpha(0.1).into()),
            border: iced::Border::default().rounded(4.0),
            ..Default::default()
        })
    } else {
        container(text("●").size(16).style(|theme: &Theme| text::Style {
            color: Some(theme.palette().primary),
        }))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.palette().background.scale_alpha(0.7).into()),
            border: iced::Border::default().rounded(4.0),
            ..Default::default()
        })
    };

    // Create the complete card with consistent sizing and background
    let card_content = Column::new()
        .spacing(4)
        .push(
            container(feed_content)
                .width(Length::Fill)
                .height(Length::Fixed(card_height - 30.0)), // Reserve space for label
        )
        .push(
            container(
                text(&feed.camera.name)
                    .size(10)
                    .style(|theme: &Theme| text::Style {
                        color: Some(theme.palette().text),
                    }),
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .padding([4, 6])
            .style(|theme: &Theme| container::Style {
                background: Some(theme.palette().background.scale_alpha(0.8).into()),
                border: iced::Border::default().rounded(4.0),
                ..Default::default()
            }),
        );

    button(
        container(card_content)
            .width(Length::Fill)
            .padding(4)
            .style(|theme: &Theme| container::Style {
                background: Some(theme.palette().background.scale_alpha(0.9).into()),
                border: iced::Border::default().rounded(6.0),
                shadow: iced::Shadow {
                    color: theme.palette().background.scale_alpha(0.3),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            }),
    )
    .on_press(Message::FeedClicked(feed_index))
    .style(|theme: &Theme, _status| button::Style {
        background: None,
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
        text_color: theme.palette().text,
    })
    .width(Length::Fill)
    .into()
}
