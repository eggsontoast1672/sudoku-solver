use raylib::prelude::*;

use crate::ui::{self, Widget};

fn center_text(d: &mut RaylibDrawHandle, text: &str, rect: Rectangle) -> Vector2 {
    let font = d.get_font_default();
    let size = font.measure_text(text, ui::FONT_SIZE, ui::FONT_SPACING);

    Vector2 {
        x: rect.x + (rect.width - size.x) / 2.0,
        y: rect.y + (rect.height - size.y) / 2.0,
    }
}

pub enum SolvingStatus {
    Going,
    Stopped,
}

impl SolvingStatus {
    pub const fn toggled(&self) -> Self {
        match self {
            Self::Going => Self::Stopped,
            Self::Stopped => Self::Going,
        }
    }
}

impl Widget for SolvingStatus {
    fn draw(&self, d: &mut RaylibDrawHandle, rect: Rectangle) {
        let (text, color) = match self {
            Self::Going => ("Going...", Color::GREEN),
            Self::Stopped => ("Stopped", Color::RED),
        };
        let pos = center_text(d, text, rect);

        d.draw_rectangle_rec(rect, color);
        d.draw_text(
            text,
            pos.x as i32,
            pos.y as i32,
            ui::FONT_SIZE as i32,
            Color::BLACK,
        );
    }
}
