#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod sudoku;

use crate::sudoku::Board;
use clap::Parser;

// easy: ....345....89...3.3....27892.4..6815....4....8765..4.27523....6.1...79....942....
// medium: ...4.6.9......3..545.....866.2.74..1....9....9..56.7.871.....643..6......6.9.2...
// hard: 9.3..42..4.65.......28..........5..4.67.4.92.1..9..........87.......94.3..83..6.1
// evil: ..9......384...5......4.3.....1..27.2..3.4..5.48..6.....6.1......7...629.....5...
// RA comp: 3.68.....1.9..5.......7..2.4..7....19.......76....8..5.4..8.......2..1.6.....18.3
// 16x16: 3..0AF....61E..C.72B..694C..AD0..E6...5D2A...8F.9A...2....D...46D....4....C....AE..A81....203..4.80...3..4...CE..15....6E....72..D1....C6....3A..04...A..2...96.F..6D7....8A2..05....B....4....DC4...6....5...72.2B...43A1...FD..68D..75F9..403.1..328....E49..B

#[derive(Parser)]
#[clap(author, version, about)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// The side length of a box in the sudoku board. (2 for 4x4, 3 for 9x9, 4 for 16x16, 5 for 25x25)
    #[clap(short = 's', long, default_value = "3")]
    pub boxsize: usize,
    /// The string describing the sudoku board.
    #[clap(short, long, value_name = "SUDOKU_STRING")]
    pub board: String,
}

fn exec<const BOX_SIZE: usize>(board: &str) {
    // board is created
    let mut b = Board::<BOX_SIZE>::from_str(board).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    // show the user their initial board, to confirm to
    // them that they have entered the correct CLI string
    println!("\nYour sudoku:");
    println!("{b}");

    let start = std::time::Instant::now();

    let success = b.solve();

    // if the solve was unsuccessful, then the given sudoku was bad, and we exit early
    if !success {
        eprintln!("overconstrained sudoku (there is no pattern of digits that can validly fill the given sudoku).");
        std::process::exit(1);
    }

    let time_taken = start.elapsed();

    // show the solved sudoku
    println!("Your solved sudoku:");
    println!("{b}\n");
    println!("solved in {:.2?}!", time_taken);
}

fn main() {
    let cli = <Cli as Parser>::parse();
    
    match cli.boxsize {
        0 | 1 => {
            eprintln!("That's a silly square size. Try something bigger.");
            std::process::exit(1);
        }
        2 => exec::<2>(&cli.board),
        3 => exec::<3>(&cli.board),
        4 => exec::<4>(&cli.board),
        5 => exec::<5>(&cli.board),
        _ => {
            eprintln!("That's a silly square size. Try something smaller.");
            std::process::exit(1);
        }
    }
}
