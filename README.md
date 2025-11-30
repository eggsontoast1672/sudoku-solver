# Sudoku Solver

## Overview

This is a program for solving Sudoku puzzles (who could have guessed). You can
enjoy watching the computer crunching the numbers.

The solver implements a very simple recursive backtracking algorithm. The board
is searched linearly from left to right, then top to bottom. Once an unfilled
square is found, the solver will try every single digit from 1 to 9. If no
digit works, then it will backtrack as far as it needs to correct any mistakes.
As you might imagine, this algorithm has a pretty scary worst-case time
complexity ($O(b^n)$ for some base $b > 1$). I have some plans to implement
other algorithms which are faster, but nothing concrete.

## Compiling and Running

Since this is a Cargo project, this section will be very short. All you have to
do is clone the repository onto your computer, navigate to the directory into
which you cloned, then do a `cargo run`. Easy as that!

I try very hard in my projects to take on as few dependencies as possible, so
the only one I'm using for this is the set of Rust bindings for Raylib. They
are very pleasant to use and I have never had any problems with them (other
than the immediate-mode GUI functionality, but we won't talk about that).
Because of that, hopefully the code compiles quite quickly.

## Usage

In its current stage, the usability of the program is not as good as it could
be. To solve a Sudoku, you must enter the puzzle into a text file using a
particular format:

    7-- -48 -5-
    --- 7-1 6-9
    --- -9- 2--
    
    37- --4 9--
    6-- --- --4
    --4 9-- -37
    
    --1 -7- ---
    2-7 5-9 ---
    -3- 48- --2

Every group of three rows is separated by a blank line, and every group of
three columns is separated by a column of spaces. The unfilled squares are
marked with dashes. Suppose that is saved in a file `sudoku.txt`. To solve it,
you would run `cargo run -- /path/to/sudoku.txt`.

It would be really nice if one could run the program and then enter the board
into the GUI. I have plans to implement this feature, but it will probably not
make it into the first release build.

## License

This code is licensed under the MIT license.
