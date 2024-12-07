#![allow(unused_imports)]

use {
    iced::{
        gradient::{ColorStop, Linear},
        widget::{checkbox, column, container, mouse_area, row, text},
        Alignment::Center,
        Background, Border, Color, Element, Gradient,
        Length::Fill,
        Theme,
    },
    iced_anim::{
        widget::button::{button, danger, primary, Status},
        AnimationBuilder, SpringMotion,
    },
    std::{f32::consts::PI, sync::LazyLock, time::Duration},
};

pub fn main() -> iced::Result {
    iced::application("Animate", State::update, State::view)
        .theme(|_| iced::Theme::CatppuccinFrappe)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    SetDeg(f32),
}

#[derive(Debug, Default)]
struct State {
    deg: f32,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::SetDeg(deg) => self.deg = deg,
        }
    }

    fn view(&self) -> Element<Message> {
        let animated_box =
            AnimationBuilder::new(self.deg, |deg| text(format!("Hello, world! {deg}")).into());

        container(
            column![
                animated_box,
                mouse_area(
                    button(text("Ok"))
                        .motion(SpringMotion::Custom {
                            response: Duration::from_millis(1000),
                            damping: SpringMotion::Smooth.damping(),
                        })
                        .style(primary)
                )
                .on_enter(Message::SetDeg(180.0))
                .on_exit(Message::SetDeg(0.0))
            ]
            .align_x(Center)
            .spacing(8)
            .padding(8),
        )
        .center(Fill)
        .into()
    }
}
