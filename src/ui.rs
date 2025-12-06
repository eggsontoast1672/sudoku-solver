//! This module contains tools related to UI widgets.

use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};

use crate::geometry;

pub const LINE_WIDTH: f32 = 10.0;
pub const FONT_SIZE: f32 = 32.0;
pub const FONT_SPACING: f32 = 1.0;

/// Represents a UI widget.
///
/// The discrete parts of this application can be split up into logical widgets, and this trait
/// allows for manipulation of abstract widgets.
pub trait Widget {
    /// Draw a widget.
    ///
    /// This is the core method for displaying widgets on the screen. Since widgets should not be
    /// aware of their position and size, the draw method takes a rectangle into which the widget
    /// should fit. This way, the main function can manage the placement and size of widgets.
    fn draw(&self, d: &mut RaylibDrawHandle, rect: Rectangle);
}

/// Determine where the given point would be without the grid lines.
///
/// If you have ever watched JoJo Part 4, this function is basically The Hand for the grid lines.
/// This function essentially gets rid of all the grid lines, scooting all of the points inward and
/// upward. This function is very important since many of the UI calculations are` much easier with
/// the grid lines gone.
///
/// If the point passed to the function lands wither outside of the board or on top of one of the
/// grid lines, the function returns [`None`] since there is not really another reasonable answer.
///
/// # Examples
///
/// ```
/// use raylib::math::Vector2;
///
/// fn go(point: Vector2) -> Vector2 {
///     let board_size = Vector2::new(100.0, 100.0);
///     sudoku_solver::ui::without_gridlines(board_size, point)
/// }
///
/// assert_eq!(go(Vector2::new(20.0, 20.0)), Some(Vector2::new(10.0, 10.0)));
/// assert_eq!(go(Vector2::new(43.0, 13.0)), Some(Vector2::new(3.0, 3.0)));
/// assert_eq!(go(Vector2::new(42.0, 69.0)), None);
/// ```
pub fn without_gridlines(board_size: Vector2, point: Vector2) -> Option<Vector2> {
    fn single_axis(coordinate: f32, cell_size: f32) -> Option<f32> {
        let mut retval = coordinate - LINE_WIDTH;
        let mut nth_cell = 0;
        while retval >= cell_size {
            retval -= cell_size + LINE_WIDTH;
            nth_cell += 1;
        }

        if retval >= 0.0 {
            Some(retval + cell_size * nth_cell as f32)
        } else {
            None
        }
    }

    let board_rect = Rectangle::new(0.0, 0.0, board_size.x, board_size.y);
    if geometry::rect_contains_point(board_rect, point) {
        let cell_size = Vector2 {
            x: (board_size.x - LINE_WIDTH * 4.0) / 3.0,
            y: (board_size.y - LINE_WIDTH * 4.0) / 3.0,
        };

        let x = single_axis(point.x, cell_size.x)?;
        let y = single_axis(point.y, cell_size.y)?;

        Some(Vector2::new(x, y))
    } else {
        return None;
    }
}
