# rust-sudoku
Sudoku solver in Rust

This solver has support for 4x4, 9x9, 16x16, and 25x25 sudokus.

Example usage:
```
$ cargo b --release
$ target/release/rust-sudoku -h
rust-sudoku 1.0.0
cosmobobak

USAGE:
    rust-sudoku [OPTIONS] --board <SUDOKU_STRING>

OPTIONS:
    -b, --board <SUDOKU_STRING>    The string describing the sudoku board
    -h, --help                     Print help information
    -s, --boxsize <BOXSIZE>        The side length of a box in the sudoku board. (2 for 4x4, 3 for
                                   9x9, 4 for 16x16, 5 for 25x25) [default: 3]
    -V, --version                  Print version information
$ target/release/rust-sudoku -b ..9......384...5......4.3.....1..27.2..3.4..5.48..6.....6.1......7...629.....5...

Your sudoku:

┌───────┬───────┬───────┐
│ . . 9 │ . . . │ . . . │
│ 3 8 4 │ . . . │ 5 . . │
│ . . . │ . 4 . │ 3 . . │
├───────┼───────┼───────┤
│ . . . │ 1 . . │ 2 7 . │
│ 2 . . │ 3 . 4 │ . . 5 │
│ . 4 8 │ . . 6 │ . . . │
├───────┼───────┼───────┤
│ . . 6 │ . 1 . │ . . . │
│ . . 7 │ . . . │ 6 2 9 │
│ . . . │ . . 5 │ . . . │
└───────┴───────┴───────┘

Your solved sudoku:

┌───────┬───────┬───────┐
│ 6 2 9 │ 5 3 1 │ 7 4 8 │
│ 3 8 4 │ 9 6 7 │ 5 1 2 │
│ 7 1 5 │ 8 4 2 │ 3 9 6 │
├───────┼───────┼───────┤
│ 9 6 3 │ 1 5 8 │ 2 7 4 │
│ 2 7 1 │ 3 9 4 │ 8 6 5 │
│ 5 4 8 │ 7 2 6 │ 9 3 1 │
├───────┼───────┼───────┤
│ 8 3 6 │ 2 1 9 │ 4 5 7 │
│ 1 5 7 │ 4 8 3 │ 6 2 9 │
│ 4 9 2 │ 6 7 5 │ 1 8 3 │
└───────┴───────┴───────┘


solved in 275.80µs!
```
