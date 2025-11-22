use crate::board::Board;

/// Solve a Sudoku board.
///
/// This function will attempt to solve the supplied Sudoku board by mutating it. If the board was
/// able to be solved, then the board parameter will be mutated to a solved state and `true` is
/// returned. If the board could not be solved, then the passed board remains unchanged and `false`
/// is returned.
pub fn solve(board: &mut Board) -> bool {
    let Some(index) = board.first_unfilled_index() else {
        return board.is_valid();
    };

    for entry in 1..=9 {
        board.set_cell_index(index, Some(entry));
        if !board.is_valid() {
            continue;
        }

        if solve(board) {
            return true;
        }
    }

    board.set_cell_index(index, None);
    false
}

/// Holds solving state.
///
/// To enable asynchronous solving, this structure holds the solving state so that solving can be
/// paused and resumed. This allows the UI to update between moves without using any truly async
/// code.
struct Solver {}
