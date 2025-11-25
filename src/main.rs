use raylib::prelude::*;

use sudoku_solver::board::{self, Board};
use sudoku_solver::solver::Solver;

const BOARD: &str = r"
    732 1__ _4_
    _9_ 726 _13
    __1 _54 9_7

    574 289 _31
    ___ _1_ 2__
    123 675 _98

    __8 _37 1_4
    _1_ 842 _59
    349 5__ _8_
";

const BOARD_RECT: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 600.0,
    height: 600.0,
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(BOARD_RECT.width as i32, BOARD_RECT.height as i32)
        .build();

    let mut board = BOARD.parse::<Board>().unwrap();
    let mut solver = Solver::new();

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        solver.step(&mut board);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        board::draw_board(&mut d, BOARD_RECT, &board);
    }
}
