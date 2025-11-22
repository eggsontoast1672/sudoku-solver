mod board;
mod solver;

use raylib::prelude::*;

use board::Board;

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

fn main() {
    let (mut rl, thread) = raylib::init().size(600, 600).build();
    let mut board = BOARD.parse::<Board>().unwrap();

    solver::solve(&mut board);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        board::draw_board(
            &mut d,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 600.0,
                height: 600.0,
            },
            &board,
        );
    }
}
