use iced::{alignment, widget::{column, container, row, text}, Background, border, Color, Element, Length};

use crate::Message;

pub fn card_view<'a>(title: &str, content: Element<'a, Message>, width: Length) -> Element<'a, Message> {
    container(
        column![
            row![text(title.to_string())
                .size(18)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Top),
            ]
            .width(width)
            .spacing(8),
            content
        ]
        .spacing(8),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(246, 247, 250))),
        border: border::rounded(2.),
        ..Default::default()
    })
    .padding(20)
    .into()
}