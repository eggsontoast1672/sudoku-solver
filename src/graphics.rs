use raylib::prelude::*;

use crate::board::{Board, Entry, point_to_index};

const LINE_WIDTH: f32 = 10.0;
const FONT_SIZE: f32 = 32.0;
const FONT_SPACING: f32 = 1.0;

/// Draw the cell decoration.
fn draw_cell(d: &mut RaylibDrawHandle, rect: Rectangle, color: Color) {
    let padding_x = rect.width / 10.0;
    let padding_y = rect.height / 10.0;
    let inner_rect = Rectangle {
        x: rect.x + padding_x,
        y: rect.y + padding_y,
        width: rect.width - padding_x * 2.0,
        height: rect.height - padding_y * 2.0,
    };

    d.draw_rectangle_rec(rect, color);
    d.draw_rectangle_rec(inner_rect, Color::WHITE);
}

fn draw_cell_entry(d: &mut RaylibDrawHandle, rect: Rectangle, entry: Entry) {
    let font = d.get_font_default();
    let text = entry.to_string();
    let dimensions = font.measure_text(&text, FONT_SIZE, FONT_SPACING);

    d.draw_text_ex(
        font,
        &text,
        Vector2 {
            x: rect.x + (rect.width - dimensions.x) / 2.0,
            y: rect.y + (rect.height - dimensions.y) / 2.0,
        },
        FONT_SIZE,
        FONT_SPACING,
        Color::BLACK,
    );
}

/// Get the line width offset for the specified cell.
///
/// In order to get the cells lined up correctly inside of the grid, this function will account
/// for the line width and return the corrected position. That was a horrible way of explaining
/// it, but nobody is looking at this code anyway.
fn line_width_offset(cell_index: usize) -> f32 {
    (cell_index / 3 + 1) as f32 * LINE_WIDTH
}

pub struct GraphicsState {
    selected_square: Option<usize>,
}

impl GraphicsState {
    pub const fn new() -> Self {
        Self {
            selected_square: None,
        }
    }

    /// Draw the board outline.
    ///
    /// The outline helps to see the big cells. Without it, the small cells floating around on the
    /// screen are pretty hard to visually parse.
    fn draw_board_outline(d: &mut RaylibDrawHandle, rect: Rectangle) {
        // This looks odd, but it just makes sure that the lines are evenly spaced horizontally and
        // vertically.
        let x_jump = (rect.width - LINE_WIDTH) / 3.0;
        for x in 0..4 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: x as f32 * x_jump,
                    y: 0.0,
                    width: LINE_WIDTH,
                    height: rect.height,
                },
                Color::BLACK,
            );
        }

        let y_jump = (rect.height - LINE_WIDTH) / 3.0;
        for y in 0..4 {
            d.draw_rectangle_rec(
                Rectangle {
                    x: 0.0,
                    y: y as f32 * y_jump,
                    width: rect.width,
                    height: LINE_WIDTH,
                },
                Color::BLACK,
            );
        }
    }

    /// Render a Sudoku board.
    pub fn draw_board(&self, d: &mut RaylibDrawHandle, board_rect: Rectangle, board: &Board) {
        let cell_width: f32 = (board_rect.width - LINE_WIDTH * 4.0) / 9.0;
        let cell_height: f32 = (board_rect.height - LINE_WIDTH * 4.0) / 9.0;

        for y in 0..9 {
            for x in 0..9 {
                let index = (y * 9) + (x % 9);
                let cell_rect = Rectangle {
                    x: x as f32 * cell_width + line_width_offset(x),
                    y: y as f32 * cell_height + line_width_offset(y),
                    width: cell_width,
                    height: cell_width,
                };

                let mouse_pos = d.get_mouse_position();

                if let Some(selected_index) = self.selected_square
                    && selected_index == index
                {
                    draw_cell(d, cell_rect, Color::RED);
                } else if let Some(mouse_index) = point_to_index(board_rect, mouse_pos)
                    && mouse_index == index
                {
                    draw_cell(d, cell_rect, Color::LIGHTPINK);
                } else {
                    draw_cell(d, cell_rect, Color::RAYWHITE);
                }

                if let Some(entry) = board.get_cell_index(index) {
                    draw_cell_entry(d, cell_rect, entry);
                }

                // d.draw_rectangle_rec(cell_rect, color);
                // d.draw_text(
                //     &cell_entry.to_string(),
                //     cell_rect.x as i32,
                //     cell_rect.y as i32,
                //     48,
                //     Color::BLACK,
                // );
            }
        }

        Self::draw_board_outline(d, board_rect);
    }
}

fn center_text(d: &mut RaylibDrawHandle, text: &str, rect: Rectangle) -> Vector2 {
    let font = d.get_font_default();
    let size = font.measure_text(text, FONT_SIZE, FONT_SPACING);

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

    pub fn draw(&self, d: &mut RaylibDrawHandle, rect: Rectangle) {
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
            FONT_SIZE as i32,
            Color::BLACK,
        );
    }
}
