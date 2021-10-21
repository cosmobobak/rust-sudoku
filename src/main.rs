mod sudoku;

use sudoku::SudokuBoard;

// const easy: &str = "----345----89---3-3----27892-4--6815----4----8765--4-27523----6-1---79----942----";
// const medium: &str = "---4-6-9------3--545-----866-2-74--1----9----9--56-7-871-----643--6------6-9-2---";
// const hard: &str = "9-3--42--4-65-------28----------5--4-67-4-92-1--9----------87-------94-3--83--6-1";
// const evil: &str = "--9------384---5------4-3-----1--27-2--3-4--5-48--6-----6-1------7---629-----5---";
// comment for debug

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("no input string provided.");
        std::process::exit(0);
    }

    // object is created
    let mut b = match SudokuBoard::from_string(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    };

    // show the user their initial board, to confirm to
    // them that they have entered the correct CLI string
    println!("\nYour sudoku:");
    b.show();

    let start = std::time::Instant::now();

    let success = b.solve();

    // if the solve was unsuccessful, then the given sudoku was bad, and we exit early
    if !success {
        println!("overconstrained sudoku (there is no pattern of digits that can validly fill the given sudoku).");
        std::process::exit(0);
    }

    let time_taken = start.elapsed();
    
    // show the solved sudoku and exit
    println!("Your solved sudoku:");
    b.show();
    println!("\nsolved in {:.2?}!", time_taken);
    std::process::exit(0);
}
