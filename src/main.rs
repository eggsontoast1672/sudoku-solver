//! A program for solving Sudoku puzzles. See the top-level README.md for more information.

#![warn(missing_docs)]

use raylib::prelude::*;

use sudoku_solver::board::Board;
use sudoku_solver::graphics::{GraphicsState, SolvingStatus};
use sudoku_solver::solver::Solver;

fn load_board() -> Board {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    let Some(path) = args.next() else {
        eprintln!("Usage: {program} <board>");
        std::process::exit(1);
    };

    match std::fs::read_to_string(&path) {
        Ok(contents) => contents.parse().unwrap(),
        Err(err) => {
            eprintln!("{program}: failed to read {path:?} to string: {err}");
            std::process::exit(1);
        }
    }
}

fn main() {
    // I'm putting this before the call to raylib::init since if there is an error on the CLI
    // level, I do not want raylib to be initialized at all.
    let mut board = load_board();

    let mut board_rect = Rectangle::new(0.0, 0.0, 512.0, 563.2);
    let (mut rl, thread) = raylib::init()
        .size(board_rect.width as i32, board_rect.height as i32)
        .title("Sudoku Solver")
        .resizable()
        .build();

    let mut solver = Solver::new();
    let mut status = SolvingStatus::Stopped;
    let graphics_state = GraphicsState::new();

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            status = status.toggled();
        }

        if let SolvingStatus::Going = status {
            solver.step(&mut board);
        }

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        let smaller = std::cmp::min(screen_width, screen_height);
        board_rect.width = smaller as f32;
        board_rect.height = smaller as f32;

        let active_area_width = board_rect.width;
        let active_area_height = active_area_width / 10.0;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        graphics_state.draw_board(&mut d, board_rect, &board);

        status.draw(
            &mut d,
            Rectangle {
                x: board_rect.x,
                y: board_rect.y + board_rect.width,
                width: active_area_width,
                height: active_area_height,
            },
        );
    }
}
