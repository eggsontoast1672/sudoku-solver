use std::collections::HashSet;
use std::hash::Hash;

use raylib::prelude::*;

/// Convert a big index into a small index.
///
/// This function converts the index of a big cell into the index of a small cell by taking the
/// index of the upper-rightmost small cell of the big cell. The bevavior is not defined if the
/// supplied index is greater than 8, so do not rely on the output of the function in that case.
fn as_small_index(big_index: usize) -> usize {
    match big_index {
        0 | 1 | 2 => big_index * 3,
        3 | 4 | 5 => big_index * 3 + 18,
        6 | 7 | 8 => big_index * 3 + 36,
        _ => big_index,
    }
}

fn has_duplicates<I>(iterator: I) -> bool
where
    I: Iterator<Item: Eq + Hash>,
{
    let mut seen = HashSet::new();
    for item in iterator {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}

/// A Sudoku board.
///
/// The board contains 9 rows and 9 columns, grouped into a 3x3 grid. Each cell contains a digit
/// from 1 to 9. Boards have the important invariant that no digit can appear twice within the same
/// row, column, or 3x3 subgrid.
#[derive(Debug)]
pub struct Board {
    pub cells: [Option<u8>; 81],
}

impl Board {
    /// Creates a new empty board.
    pub const fn empty() -> Board {
        Board { cells: [None; 81] }
    }

    /// Retrieve the entry in a particular cell.
    ///
    /// If this function returns [`None`], that means that the cell at the specified row and column
    /// has not yet been filled in.
    ///
    /// # Panics
    ///
    /// If either the row or the column is at least 9 (meaning the cell is outside of the board),
    /// this function panics.
    pub const fn get_cell(&self, row: usize, column: usize) -> Option<u8> {
        // We can't use the get method on arrays since it's not enough that the index computation
        // doesn't overflow. We need the row and column to individually be valid. For example, if
        // row = 2 and column = 1000000, the index would be in range, but clearly the column is not
        // valid.
        if row < 9 && column < 9 {
            self.cells[(row * 9) + (column % 9)]
        } else {
            panic!("cell out of range")
        }
    }

    /// Retrieve an entire row.
    ///
    /// # Panics
    ///
    /// This function panics if the row is at least 9.
    pub fn get_row(&self, row: usize) -> Vec<Option<u8>> {
        (0..9).map(|x| self.cells[x + row * 9]).collect()
    }

    /// Retrieve an entire column.
    ///
    /// # Panics
    ///
    /// This function panics if the column is at least 9.
    pub fn get_column(&self, column: usize) -> Vec<Option<u8>> {
        (0..9).map(|x| self.cells[x * 9 + column]).collect()
    }

    /// Retrieve a big cell.
    ///
    /// In Sudoku, the board can be divided into 9 big cells, each 3x3 in size. This function will
    /// treat the board as if it is made up of big cells, and return the cell at the supplied
    /// index. Indices run along the width of the board first, then down the height.
    ///
    /// # Panics
    ///
    /// This function panics if the index is at least 9.
    pub fn get_big_cell(&self, index: usize) -> Vec<Option<u8>> {
        let small_index = as_small_index(index);
        vec![
            self.cells[small_index],
            self.cells[small_index + 1],
            self.cells[small_index + 2],
            self.cells[small_index + 9],
            self.cells[small_index + 10],
            self.cells[small_index + 11],
            self.cells[small_index + 18],
            self.cells[small_index + 19],
            self.cells[small_index + 20],
        ]
    }

    /// Set the cell at the target index to the specified value.
    ///
    /// The board has exactly 81 cells, so this function will do nothing if the index is greater
    /// than 80. Additionally, all cells must be in the range [1, 9], so if the supplied entry is
    /// not in that range, the funcion will do nothing. To clear the entry at the target index, you
    /// can pass [`None`].
    pub fn set_cell_index(&mut self, index: usize, entry: Option<u8>) {
        if index < self.cells.len() {
            if let Some(1..=9) | None = entry {
                self.cells[index] = entry;
            }
        }
    }

    /// Retrieve the index of the first unfilled cell.
    ///
    /// Imagine that the rows of the board are positioned one after another. The first unfilled
    /// cell is the first cell from the left which contains no entry. If there is no such cell,
    /// e.g. all cells have been filled, then [`None`] is returned.
    pub fn first_unfilled_index(&self) -> Option<usize> {
        self.cells
            .into_iter()
            .enumerate()
            .find(|(_, x)| x.is_none())
            .map(|(index, _)| index)
    }

    /// Check whether or not a board is valid.
    ///
    /// A board is valid if every row, column, and big cell contains every digit at most once. For
    /// instance, a board is not valid if a row contains two 2's.
    pub fn is_valid(&self) -> bool {
        let mut result = true;

        for index in 0..9 {
            let row = self.get_row(index);
            let column = self.get_column(index);
            let big_cell = self.get_big_cell(index);

            result = result && !has_duplicates(row.iter().filter_map(|&x| x));
            result = result && !has_duplicates(column.iter().filter_map(|&x| x));
            result = result && !has_duplicates(big_cell.iter().filter_map(|&x| x));
        }

        result
    }
}

impl std::str::FromStr for Board {
    type Err = ();

    /// Convert a string into a board.
    ///
    /// Strictly speaking, the string does not need a particular format. The function skips past
    /// any characters that are not the digits 1 through 9, or an underscore. All of the cells in
    /// the board are initialized one by one as digits are found in the string. For example, the
    /// strings "16_9____52___456_9_9__3_7_2 6____7_939___1___747_3_9__8
    /// 7_2_8_956__629___4__9_____1" and
    ///
    ///     +-------+-------+-------+
    ///     | 1 6 _ | 9 _ _ | _ _ 5 |
    ///     | 2 _ _ | _ 4 5 | 6 _ 9 |
    ///     | _ 9 _ | _ 3 _ | 7 _ 2 |
    ///     +-------+-------+-------+
    ///     | 6 _ _ | _ _ 7 | _ 9 3 |
    ///     | 9 _ _ | _ 1 _ | _ _ 7 |
    ///     | 4 7 _ | 3 _ 9 | _ _ 8 |
    ///     +-------+-------+-------+
    ///     | 7 _ 2 | _ 8 _ | 9 5 6 |
    ///     | _ _ 6 | 2 9 _ | _ _ 4 |
    ///     | _ _ 9 | _ _ _ | _ _ 1 |
    ///     +-------+-------+-------+
    ///
    /// parse to the same board.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::empty();
        let mut index = 0;
        for c in s.chars() {
            match c {
                '_' => {
                    board.cells[index] = None;
                    index += 1;
                }
                '1'..='9' => {
                    board.cells[index] = Some(c as u8 - '0' as u8);
                    index += 1;
                }
                _ => {}
            }
        }
        Ok(board)
    }
}

const COLOR_ONE: Color = Color::DARKRED;
const COLOR_TWO: Color = Color::ORANGE;
const COLOR_THREE: Color = Color::LIGHTBLUE;
const COLOR_FOUR: Color = Color::TURQUOISE;
const COLOR_FIVE: Color = Color::GREEN;
const COLOR_SIX: Color = Color::HOTPINK;
const COLOR_SEVEN: Color = Color::BLUE;
const COLOR_EIGHT: Color = Color::MAGENTA;
const COLOR_NINE: Color = Color::PURPLE;

/// Get the color of a digit.
///
/// Every digit from 1 to 9 has a particular color associated with it to help with visually parsing
/// the board. This function returns the color associated with that digit, or [`None`] if the given
/// number is outside of the range \[1, 9\].
const fn color_from_digit(digit: i32) -> Option<Color> {
    match digit {
        1 => Some(COLOR_ONE),
        2 => Some(COLOR_TWO),
        3 => Some(COLOR_THREE),
        4 => Some(COLOR_FOUR),
        5 => Some(COLOR_FIVE),
        6 => Some(COLOR_SIX),
        7 => Some(COLOR_SEVEN),
        8 => Some(COLOR_EIGHT),
        9 => Some(COLOR_NINE),
        _ => None,
    }
}

const LINE_WIDTH: f32 = 10.0;

/// Draw the board outline.
///
/// The outline helps to see the big cells. Without it, the small cells floating around on the
/// screen are pretty hard to visually parse.
fn draw_outline(d: &mut RaylibDrawHandle, rect: Rectangle) {
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

/// Get the line width offset for the specified cell.
///
/// In order to get the cells lined up correctly inside of the grid, this function will account for
/// the line width and return the corrected position. That was a horrible way of explaining it, but
/// nobody is looking at this code anyway.
fn line_width_offset(cell_index: usize) -> f32 {
    (cell_index / 3 + 1) as f32 * LINE_WIDTH
}

/// Render a Sudoku board.
pub fn draw_board(d: &mut RaylibDrawHandle, board_rect: Rectangle, board: &Board) {
    let cell_width: f32 = (board_rect.width - LINE_WIDTH * 4.0) / 9.0;
    let cell_height: f32 = (board_rect.height - LINE_WIDTH * 4.0) / 9.0;

    for y in 0..9 {
        for x in 0..9 {
            let Some(cell_entry) = board.get_cell(y, x) else {
                continue;
            };
            let color = color_from_digit(cell_entry as i32).unwrap();

            // println!("line_width_offset({x}) = {}", line_width_offset(x));
            // println!("line_width_offset({y}) = {}", line_width_offset(y));

            let cell_rect = Rectangle {
                x: x as f32 * cell_width + line_width_offset(x),
                y: y as f32 * cell_height + line_width_offset(y),
                width: cell_width,
                height: cell_width,
            };

            d.draw_rectangle_rec(cell_rect, color);
            d.draw_text(
                &cell_entry.to_string(),
                cell_rect.x as i32,
                cell_rect.y as i32,
                48,
                Color::BLACK,
            );
        }
    }

    draw_outline(d, board_rect);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn create_board() -> Board {
        Board::from_str(
            r"+-------+-------+-------+
              | 1 6 _ | 9 _ _ | _ _ 5 |
              | 2 _ _ | _ 4 5 | 6 _ 9 |
              | _ 9 _ | _ 3 _ | 7 _ 2 |
              +-------+-------+-------+
              | 6 _ _ | _ _ 7 | _ 9 3 |
              | 9 _ _ | _ 1 _ | _ _ 7 |
              | 4 7 _ | 3 _ 9 | _ _ 8 |
              +-------+-------+-------+
              | 7 _ 2 | _ 8 _ | 9 5 6 |
              | _ _ 6 | 2 9 _ | _ _ 4 |
              | _ _ 9 | _ _ _ | _ _ 1 |
              +-------+-------+-------+",
        )
        .unwrap()
    }

    #[test]
    fn test_get_row() {
        let board = create_board();

        #[rustfmt::skip]
        assert_eq!(
            board.get_row(0),
            vec![
                Some(1), Some(6), None,
                Some(9), None,    None,
                None,    None,    Some(5),
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_row(4),
            vec![
                Some(9), None,    None,
                None,    Some(1), None,
                None,    None,    Some(7),
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_row(6),
            vec![
                Some(7), None,    Some(2),
                None,    Some(8), None,
                Some(9), Some(5), Some(6),
            ]
        );
    }

    #[test]
    fn test_get_column() {
        let board = create_board();

        #[rustfmt::skip]
        assert_eq!(
            board.get_column(0),
            vec![
                Some(1), Some(2), None,
                Some(6), Some(9), Some(4),
                Some(7), None,    None,
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_column(1),
            vec![
                Some(6), None, Some(9),
                None,    None, Some(7),
                None,    None, None,
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_column(8),
            vec![
                Some(5), Some(9), Some(2),
                Some(3), Some(7), Some(8),
                Some(6), Some(4), Some(1),
            ]
        );
    }

    #[test]
    fn test_get_big_cell() {
        let board = create_board();

        #[rustfmt::skip]
        assert_eq!(
            board.get_big_cell(2),
            vec![
                None,    None, Some(5),
                Some(6), None, Some(9),
                Some(7), None, Some(2),
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_big_cell(5),
            vec![
                None, Some(9), Some(3),
                None, None,    Some(7),
                None, None,    Some(8),
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            board.get_big_cell(7),
            vec![
                None,    Some(8), None,
                Some(2), Some(9), None,
                None,    None,    None,
            ]
        );
    }

    #[test]
    fn test_is_valid() {
        let mut board = create_board();
        assert!(board.is_valid());
        board.set_cell_index(2, Some(6));
        assert!(!board.is_valid());
    }
}
