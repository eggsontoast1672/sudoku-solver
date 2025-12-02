//! Functions and structures related to Sudoku boards. Some of those items include structures for
//! representing cells on the board and the board itself, as well as board manipulation
//! functionality.

use std::collections::HashSet;
use std::hash::Hash;

use raylib::prelude::*;

/// An entry for a cell of the Sudoku board.
///
/// Each square of the board can contain a digit from 1 to 9. This enum ensures that no invalid
/// digit can be represented inside of the board. I would hope that the individual members do not
/// need their own documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Entry {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Entry {
    /// Get the successor of an entry.
    ///
    /// An entry is just a number, so this function retrieves the Peano-style successor. Naturally,
    /// there is no valid entry larger than 9, so attempting to get the successor of 9 will return
    /// [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_solver::board::Entry;
    ///
    /// assert_eq!(Entry::One.successor(), Some(Entry::Two));
    /// assert_eq!(Entry::Five.successor(), Some(Entry::Six));
    /// assert_eq!(Entry::Nine.successor(), None);
    /// ```
    pub fn successor(&self) -> Option<Self> {
        let number: i32 = self.clone().into();
        Self::try_from(number + 1).ok()
    }
}

impl TryFrom<i32> for Entry {
    type Error = ();

    /// Attempt to convert a number to an [`Entry`].
    ///
    /// Since the board entries represent numbers, it is natural to want to convert to an entry
    /// from a number. However, not all integers represent valid entries (in particular, only the
    /// digits 1-9 represent valid entries). If the integer passed is in that range, then the
    /// corresponding entry is returned. Otherwise, `Err(())` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_solver::board::Entry;
    ///
    /// assert_eq!(Entry::try_from(1), Ok(Entry::One));
    /// assert_eq!(Entry::try_from(7), Ok(Entry::Seven));
    /// assert_eq!(Entry::try_from(0), Err(()));
    /// assert_eq!(Entry::try_from(10), Err(()));
    /// ```
    fn try_from(value: i32) -> Result<Entry, Self::Error> {
        match value {
            1 => Ok(Entry::One),
            2 => Ok(Entry::Two),
            3 => Ok(Entry::Three),
            4 => Ok(Entry::Four),
            5 => Ok(Entry::Five),
            6 => Ok(Entry::Six),
            7 => Ok(Entry::Seven),
            8 => Ok(Entry::Eight),
            9 => Ok(Entry::Nine),
            _ => Err(()),
        }
    }
}

impl Into<i32> for Entry {
    fn into(self) -> i32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Into::<i32>::into(*self).fmt(f)
    }
}

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
    /// The cells of the board.
    ///
    /// Each square of a Sudoku board is either empty, or occupied by a digit in the range 1-9.
    /// Since these details are adequately reflected in the type of this field, it makes sense for
    /// it to be public. This may change in the future.
    pub cells: [Option<Entry>; 81],
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
    pub const fn get_cell(&self, row: usize, column: usize) -> Option<Entry> {
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

    pub const fn get_cell_index(&self, index: usize) -> Option<Entry> {
        self.cells[index]
    }

    /// Retrieve an entire row.
    ///
    /// # Panics
    ///
    /// This function panics if the row is at least 9.
    pub fn get_row(&self, row: usize) -> Vec<Option<Entry>> {
        (0..9).map(|x| self.cells[x + row * 9]).collect()
    }

    /// Retrieve an entire column.
    ///
    /// # Panics
    ///
    /// This function panics if the column is at least 9.
    pub fn get_column(&self, column: usize) -> Vec<Option<Entry>> {
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
    pub fn get_big_cell(&self, index: usize) -> Vec<Option<Entry>> {
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
    pub fn set_cell_index(&mut self, index: usize, entry: Option<Entry>) {
        if index < self.cells.len() {
            self.cells[index] = entry;
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
    /// +-------+-------+-------+
    /// | 1 6 _ | 9 _ _ | _ _ 5 |
    /// | 2 _ _ | _ 4 5 | 6 _ 9 |
    /// | _ 9 _ | _ 3 _ | 7 _ 2 |
    /// +-------+-------+-------+
    /// | 6 _ _ | _ _ 7 | _ 9 3 |
    /// | 9 _ _ | _ 1 _ | _ _ 7 |
    /// | 4 7 _ | 3 _ 9 | _ _ 8 |
    /// +-------+-------+-------+
    /// | 7 _ 2 | _ 8 _ | 9 5 6 |
    /// | _ _ 6 | 2 9 _ | _ _ 4 |
    /// | _ _ 9 | _ _ _ | _ _ 1 |
    /// +-------+-------+-------+
    ///
    /// parse to the same board.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::empty();
        let mut index = 0;
        for c in s.chars() {
            match c {
                '-' => {
                    board.cells[index] = None;
                    index += 1;
                }
                '1'..='9' => {
                    let entry = Entry::try_from(c as i32 - '0' as i32).unwrap();
                    board.cells[index] = Some(entry);
                    index += 1;
                }
                _ => {}
            }
        }
        Ok(board)
    }
}

/// Convert a cell's position to an index.
///
/// In board space, points are pairs of integers 0-8. In other words, a point is a pair of indices
/// for rows and columns.
fn cell_pos_to_index(x: usize, y: usize) -> Option<usize> {
    if x < 9 && y < 9 {
        Some(y * 9 + x)
    } else {
        None
    }
}

/// Convert a point in screen space to a board index.
pub fn point_to_index(rect: Rectangle, point: Vector2) -> Option<usize> {
    let x = ((point.x - rect.x) / 9.0) as usize;
    let y = ((point.y - rect.y) / 9.0) as usize;
    cell_pos_to_index(x, y)
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

        assert_eq!(
            board.get_row(0),
            vec![
                Some(Entry::One),
                Some(Entry::Six),
                None,
                Some(Entry::Nine),
                None,
                None,
                None,
                None,
                Some(Entry::Five),
            ]
        );

        assert_eq!(
            board.get_row(4),
            vec![
                Some(Entry::Nine),
                None,
                None,
                None,
                Some(Entry::One),
                None,
                None,
                None,
                Some(Entry::Seven),
            ]
        );

        assert_eq!(
            board.get_row(6),
            vec![
                Some(Entry::Seven),
                None,
                Some(Entry::Two),
                None,
                Some(Entry::Eight),
                None,
                Some(Entry::Nine),
                Some(Entry::Five),
                Some(Entry::Six),
            ]
        );
    }

    #[test]
    fn test_get_column() {
        let board = create_board();

        assert_eq!(
            board.get_column(0),
            vec![
                Some(Entry::One),
                Some(Entry::Two),
                None,
                Some(Entry::Six),
                Some(Entry::Nine),
                Some(Entry::Four),
                Some(Entry::Seven),
                None,
                None,
            ]
        );

        assert_eq!(
            board.get_column(1),
            vec![
                Some(Entry::Six),
                None,
                Some(Entry::Nine),
                None,
                None,
                Some(Entry::Seven),
                None,
                None,
                None,
            ]
        );

        assert_eq!(
            board.get_column(8),
            vec![
                Some(Entry::Five),
                Some(Entry::Nine),
                Some(Entry::Two),
                Some(Entry::Three),
                Some(Entry::Seven),
                Some(Entry::Eight),
                Some(Entry::Six),
                Some(Entry::Four),
                Some(Entry::One),
            ]
        );
    }

    #[test]
    fn test_get_big_cell() {
        let board = create_board();

        assert_eq!(
            board.get_big_cell(2),
            vec![
                None,
                None,
                Some(Entry::Five),
                Some(Entry::Six),
                None,
                Some(Entry::Nine),
                Some(Entry::Seven),
                None,
                Some(Entry::Two),
            ]
        );

        assert_eq!(
            board.get_big_cell(5),
            vec![
                None,
                Some(Entry::Nine),
                Some(Entry::Three),
                None,
                None,
                Some(Entry::Seven),
                None,
                None,
                Some(Entry::Eight),
            ]
        );

        assert_eq!(
            board.get_big_cell(7),
            vec![
                None,
                Some(Entry::Eight),
                None,
                Some(Entry::Two),
                Some(Entry::Nine),
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_is_valid() {
        let mut board = create_board();
        assert!(board.is_valid());
        board.set_cell_index(2, Some(Entry::Six));
        assert!(!board.is_valid());
    }
}
