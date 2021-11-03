const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIVIDER: &str = "├───────┼───────┼───────┤\n";
const TOP: &str = "┌───────┬───────┬───────┐\n";
const BOTTOM: &str = "└───────┴───────┴───────┘\n";
const BAR: &str = "│ ";

#[derive(Clone)]
pub struct SudokuBoard {
    state: [[u8; 9]; 9],
    first_unassigned: usize,
}

struct BoxIterator<'a> {
    row: usize,
    col: usize,
    sentinel: usize,
    board: &'a SudokuBoard,
}

pub struct GlobalIterator<'a> {
    row: usize,
    col: usize,
    board: &'a SudokuBoard,
}

impl BoxIterator<'_> {
    fn new(x: usize, board: &SudokuBoard) -> BoxIterator {
        let row = ((x / 9) / 3) * 3;
        let col = ((x % 9) / 3) * 3;
        let sentinel = row + 3;
        BoxIterator {
            row,
            col,
            sentinel,
            board,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let result = if self.row == self.sentinel {
            None
        } else {
            Some(self.board.state[self.row][self.col])
        };
        if (self.col) % 3 == 2 {
            self.col -= 2;
            self.row += 1;
        } else {
            self.col += 1;
        }
        result
    }
}

impl GlobalIterator<'_> {
    fn new(board: &SudokuBoard) -> GlobalIterator {
        GlobalIterator {
            row: 0,
            col: 0,
            board,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let result = if self.row == 9 {
            None
        } else {
            Some(self.board.state[self.row][self.col])
        };
        if self.col == 8 {
            self.col = 0;
            self.row += 1;
        } else {
            self.col += 1;
        }
        result
    }
}

impl Iterator for BoxIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        BoxIterator::next(self)
    }
}

impl Iterator for GlobalIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        GlobalIterator::next(self)
    }
}

impl SudokuBoard {
    pub fn from_string(fen: &str) -> Result<SudokuBoard, String> {
        if !SudokuBoard::is_string_valid(fen) {
            Err(format!(
                "input string invalid (you may only use digits and dashes in your input): \"{}\"",
                fen
            ))
        } else {
            let mut out = SudokuBoard {
                state: [[0; 9]; 9],
                first_unassigned: 0,
            };
            out.set_from_string(fen);
            match out.current_state_invalid() {
                false => Ok(out),
                true => Err("input sudoku invalid (given problem has repeated digits in rows, columns, or squares).".to_string())
            }
        }
    }

    fn set_from_string(&mut self, fen: &str) {
        for (i, c) in fen.chars().enumerate() {
            if c != '-' {
                self.state[i / 9][i % 9] =
                    c.to_digit(10).expect("this should have been validated >:(") as u8;
            }
        }
    }

    fn is_string_valid(fen: &str) -> bool {
        fen.chars()
            .all(|c| c == '-' || c.is_ascii_digit() && c != '0')
    }

    pub fn show(&self) {
        println!();
        print!("{}", TOP);
        for (i, row) in self.state.iter().enumerate() {
            print!("{}", BAR);
            for (j, val) in row.iter().enumerate() {
                print!("{} ", SYMBOLS[*val as usize]);
                if j % 3 == 2 && j != 8 {
                    print!("{}", BAR);
                }
            }
            println!("{}", BAR);
            if i % 3 == 2 && i != 8 {
                print!("{}", DIVIDER);
            };
        }
        print!("{}", BOTTOM);
    }

    fn is_unassigned(&self, n: usize) -> bool {
        self.state[n / 9][n % 9] == 0
    }

    pub fn first_unassigned(&mut self) -> usize {
        for i in self.first_unassigned..81 {
            if self.is_unassigned(i) {
                self.first_unassigned = i;
                return i;
            }
        }
        81
    }

    fn current_state_invalid(&mut self) -> bool {
        for i in 0..81 {
            let n = self.state[i / 9][i % 9];
            if n != 0 {
                self.state[i / 9][i % 9] = 0;
                if !self.legal(i, n) {
                    return true;
                }
                self.state[i / 9][i % 9] = n;
            }
        }
        false
    }

    pub fn iter(&self) -> GlobalIterator {
        GlobalIterator::new(self)
    }

    fn box_iter(&self, x: usize) -> BoxIterator {
        BoxIterator::new(x, self)
    }

    fn legal(&self, x: usize, num: u8) -> bool {
        self.state[x / 9].iter().all(|n| *n != num)
            && self.state.iter().all(|row| row[x % 9] != num)
            && self.box_iter(x).all(|n| n != num)
    }

    pub fn solve(&mut self) -> bool {
        let x = self.first_unassigned();
        // If there is no unassigned location, we are done
        if x == 81 {
            return true; // success!
        }
        for num in 1..10 {
            if self.legal(x, num) {
                self.state[x / 9][x % 9] = num;
                if self.solve() {
                    return true;
                }
                self.state[x / 9][x % 9] = UNASSIGNED;
                self.first_unassigned = x;
            }
        }
        false // this triggers backtracking
    }
}
