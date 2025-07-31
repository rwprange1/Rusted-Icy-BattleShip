use iced::overlay::menu;

use iced::theme::Palette;
use iced::widget::{button, container, pick_list};
use iced::{Border, Color, Shadow};
use iced::window::Position::Default;
use crate::values::Board;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    };
}

pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);
pub const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);
pub const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0);

#[derive(Debug, Clone)]
pub enum Message {
    JOIN,
    JOINING,
    START,
    LEAVE,
    FIRE,
}

pub struct GameInfo {
    pub my_board: Board,
}

pub type BsBtn = fn(&iced::Theme, iced::widget::button::Status) -> button::Style;


pub fn btn_style_water(theme: &iced::Theme, _status: iced::widget::button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(BLUE)),
        text_color: rgb!(45., 45., 45.),
        border: Border {
            width: 1.0,
            color: Color::BLACK,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        
    }
}