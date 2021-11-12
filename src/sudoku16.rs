const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 17] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', '0'];
const DIVIDER: &str = "├─────────┼─────────┼─────────┼─────────┤\n";
const TOP: &str = "┌─────────┬─────────┬─────────┬─────────┐\n";
const BOTTOM: &str = "└─────────┴─────────┴─────────┴─────────┘\n";
const BAR: &str = "│ ";

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const NUM_LOCATIONS: usize = WIDTH * HEIGHT;
const MAX_VALUE: u8 = 16;
const EMPTY: u8 = 0;

#[derive(Clone)]
pub struct SudokuBoard16 {
    state: [[u8; WIDTH]; WIDTH]
}

struct BoxIterator<'a> {
    row: usize,
    col: usize,
    sentinel: usize,
    board: &'a SudokuBoard16,
}

pub struct GlobalIterator<'a> {
    row: usize,
    col: usize,
    board: &'a SudokuBoard16,
}

impl BoxIterator<'_> {
    fn new(x: usize, board: &SudokuBoard16) -> BoxIterator {
        let row = ((x / WIDTH) / 4) * 4;
        let col = ((x % WIDTH) / 4) * 4;
        let sentinel = row + 4;
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
        if (self.col) % 4 == 3 {
            self.col -= 3;
            self.row += 1;
        } else {
            self.col += 1;
        }
        result
    }
}

impl GlobalIterator<'_> {
    fn new(board: &SudokuBoard16) -> GlobalIterator {
        GlobalIterator {
            row: 0,
            col: 0,
            board,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let result = if self.row == HEIGHT {
            None
        } else {
            Some(self.board.state[self.row][self.col])
        };
        if self.col == 15 {
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

impl SudokuBoard16 {
    pub fn from_string(fen: &str) -> Result<SudokuBoard16, String> {
        if !SudokuBoard16::is_string_valid(fen) {
            Err(format!(
                "input string invalid (you may only use digits and dashes in your input): \"{}\"",
                fen
            ))
        } else {
            let mut out = SudokuBoard16 {
                state: [[0; WIDTH]; WIDTH]
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
                let digit = c.to_digit(16).expect("this should have been validated >:(") as u8;
                let digit = if digit == 0 {
                    MAX_VALUE
                } else {
                    digit
                };
                self.state[i / WIDTH][i % WIDTH] = digit;
            }
        }
    }

    fn is_string_valid(fen: &str) -> bool {
        const LEGALS: [char; 17] = ['-', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', '0'];
        fen.chars().all(|c| LEGALS.contains(&c))
    }

    pub fn show(&self) {
        let mut out = String::new();
        out.push('\n');
        out.push_str(TOP);
        for (i, row) in self.state.iter().enumerate() {
            out.push_str(BAR);
            for (j, val) in row.iter().enumerate() {
                out.push_str(format!("{} ", SYMBOLS[*val as usize]).as_str());
                if j % 4 == 3 && j != 15 {
                    out.push_str(BAR);
                }
            }
            out.push_str(BAR);
            out.push('\n');
            if i % 4 == 3 && i != 15 {
                out.push_str(DIVIDER);
            };
        }
        out.push_str(BOTTOM);
        println!("{}", out);
    }

    fn is_unassigned(&self, n: usize) -> bool {
        self.state[n / WIDTH][n % WIDTH] == 0
    }

    pub fn first_unassigned(&self) -> Option<usize> {
        for i in 0..NUM_LOCATIONS {
            if self.is_unassigned(i) {
                return Some(i);
            }
        }
        None
    }

    fn current_state_invalid(&mut self) -> bool {
        for i in 0..NUM_LOCATIONS {
            let n = self.state[i / WIDTH][i % WIDTH];
            if n != 0 {
                self.state[i / WIDTH][i % WIDTH] = 0;
                if !self.legal(i, n) {
                    return true;
                }
                self.state[i / WIDTH][i % WIDTH] = n;
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
        self.state[x / WIDTH].iter().all(|n| *n != num)
            && self.state.iter().all(|row| row[x % WIDTH] != num)
            && self.box_iter(x).all(|n| n != num)
    }

    fn options_for_loc(&self, x: usize) -> usize {
        let mut options = 0;
        for num in 1..=MAX_VALUE {
            if self.legal(x, num) {
                options += 1;
            }
        }
        options
    }

    fn is_loc_single_constrained(&self, x: usize) -> bool {
        let mut options = 0;
        for num in 1..=MAX_VALUE {
            if self.legal(x, num) {
                options += 1;
                if options > 1 {
                    return false;
                }
            }
        }
        options == 1
    }

    fn first_legal_for_loc(&self, x: usize) -> Option<u8> {
        for num in 1..=MAX_VALUE {
            if self.legal(x, num) {
                return Some(num);
            }
        }
        None
    }

    fn fill_trivial(&mut self) -> bool {
        // apply simple logical fill-ins of squares
        // i.e. if a square can only have one number, fill it with that number.
        // this is a preprocessing step to reduce the search space.
        for (x, v) in self.iter().enumerate() {
            if v != EMPTY {
                continue;
            }
            if self.is_loc_single_constrained(x) {
                self.state[x / WIDTH][x % WIDTH] = self.first_legal_for_loc(x).unwrap();
                return true;
            }
        }
        // return whether we mutated the board at all
        false
    }

    pub fn solve_dfs(&mut self) -> bool {
        let x = match self.first_unassigned() {
            Some(x) => x,
            None => return true,
        };
        
        for num in 1..=MAX_VALUE {
            if self.legal(x, num) {
                self.state[x / WIDTH][x % WIDTH] = num;
                if self.solve_dfs() {
                    return true;
                }
                self.state[x / WIDTH][x % WIDTH] = UNASSIGNED;
            }
        }
        false // this triggers backtracking
    }

    pub fn solve(&mut self) -> bool {
        while self.fill_trivial() {
            // keep filling in trivial squares until we can't do any more
        }
        self.show();
        self.solve_dfs()
    }
}
