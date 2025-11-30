use raylib::prelude::*;

use sudoku_solver::board::Board;
use sudoku_solver::graphics::GraphicsState;
use sudoku_solver::solver::Solver;

fn load_board<P>(path: P) -> Board
where
    P: AsRef<std::path::Path>,
{
    let contents = std::fs::read_to_string(path).unwrap();
    contents.parse().unwrap()
}

fn main() {
    let mut board_rect = Rectangle::new(0.0, 0.0, 512.0, 512.0);

    let (mut rl, thread) = raylib::init()
        .size(board_rect.width as i32, board_rect.height as i32)
        .resizable()
        .build();

    let mut board = load_board("boards/medium-1.txt");
    let mut solver = Solver::new();
    let graphics_state = GraphicsState::new();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        solver.step(&mut board);

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        let smaller = std::cmp::min(screen_width, screen_height);
        board_rect.width = smaller as f32;
        board_rect.height = smaller as f32;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        graphics_state.draw_board(&mut d, board_rect, &board);

        // draw_ui(&mut d);
    }
}
