use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AttemptLocation(usize);

/// Solve a Sudoku board.
///
/// This function will attempt to solve the supplied Sudoku board by mutating it. If the board was
/// able to be solved, then the board parameter will be mutated to a solved state and `true` is
/// returned. If the board could not be solved, then the passed board remains unchanged and `false`
/// is returned.
pub fn solve(board: &mut Board) -> bool {
    // What data is each stack frame holding? In other words, what data persists between changes to
    // the board (between recursive calls)?
    //
    // - entry  (unique for every stack frame)
    // - index

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
#[derive(Default)]
pub struct Solver {
    attempt_stack: Vec<AttemptLocation>,
    backtracking: bool,
}

impl Solver {
    /// Create a new solver.
    pub const fn new() -> Solver {
        Solver {
            attempt_stack: Vec::new(),
            backtracking: false,
        }
    }

    /// Step the solver once.
    pub fn step(&mut self, board: &mut Board) -> bool {
        if !board.is_valid() {
            // The last move was not valid
            let AttemptLocation(last_index) = self
                .attempt_stack
                .pop()
                .expect("The board you passed was invalid to begin with");

            let last_entry = board
                .get_cell_index(last_index)
                .expect("there should be a cell here");

            if last_entry != 9 {
                board.set_cell_index(last_index, Some(last_entry + 1));
                self.attempt_stack.push(AttemptLocation(last_index));
            } else {
                board.set_cell_index(last_index, None);
                self.backtracking = true;
            }

            return false;
        }

        if self.backtracking {
            let AttemptLocation(last_index) = self
                .attempt_stack
                .pop()
                .expect("The board you passed was invalid to begin with");

            let last_entry = board
                .get_cell_index(last_index)
                .expect("there should be a cell here");

            if last_entry != 9 {
                board.set_cell_index(last_index, Some(last_entry + 1));
                self.attempt_stack.push(AttemptLocation(last_index));
                self.backtracking = false;
            } else {
                board.set_cell_index(last_index, None);
                self.backtracking = true;
            }

            return false;
        }

        // At this point the last move was valid, so we move on to make another move. Search for
        // the first unfilled cell in the board. If the board only has filled cells, then it must
        // be solved since no invalid entry can be made.
        let Some(index) = board.first_unfilled_index() else {
            return true;
        };

        // If there is an unfilled square, we need to try to fill it. But with what? The current
        // attempt member tells us what we have previously tried. We want to try the next one after
        // that.
        board.set_cell_index(index, Some(1));
        self.attempt_stack.push(AttemptLocation(index));
        false
    }
}
