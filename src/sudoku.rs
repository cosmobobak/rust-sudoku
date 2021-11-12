const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIVIDER: &str = "├───────┼───────┼───────┤\n";
const TOP: &str = "┌───────┬───────┬───────┐\n";
const BOTTOM: &str = "└───────┴───────┴───────┘\n";
const BAR: &str = "│ ";

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const NUM_LOCATIONS: usize = WIDTH * HEIGHT;
const MAX_VALUE: u8 = 9;

#[derive(Clone)]
pub struct SudokuBoard {
    state: [[u8; WIDTH]; WIDTH]
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
        let row = ((x / WIDTH) / 3) * 3;
        let col = ((x % WIDTH) / 3) * 3;
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
        let result = if self.row == HEIGHT {
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
                state: [[0; WIDTH]; WIDTH],
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
                self.state[i / WIDTH][i % WIDTH] =
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

    fn is_unassigned(&self, x: usize) -> bool {
        self.state[x / WIDTH][x % WIDTH] == 0
    }

    fn score_unassigned(&self, x: usize) -> usize {
        let num_constraints_in_row = self.state[x / WIDTH].iter().filter(|v| **v != 0).count();
        let num_constraints_in_col = self.state.iter().map(|row| row[x % WIDTH]).filter(|v| *v != 0).count();
        num_constraints_in_row + num_constraints_in_col
    }

    pub fn first_unassigned(&self) -> Option<usize> {
        for i in 0..NUM_LOCATIONS {
            if self.is_unassigned(i) {
                return Some(i);
            }
        }
        None
    }

    pub fn choose_unassigned_smart(&mut self) -> Option<usize> {
        let mut max = 0;
        let mut loc = None;
        for i in 0..NUM_LOCATIONS {
            if self.is_unassigned(i) && (self.score_unassigned(i) > max || loc.is_none()) {
                max = self.score_unassigned(i);
                loc = Some(i);
            }
        }
        loc
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
            if v != 0 {
                continue;
            }
            if self.options_for_loc(x) == 1 {
                self.state[x / WIDTH][x % WIDTH] = self.first_legal_for_loc(x).unwrap();
                return true;
            }
        }
        // return whether we mutated the board at all
        false
    }

    pub fn solve_dfs(&mut self) -> bool {
        let x = match self.choose_unassigned_smart() {
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

        self.solve_dfs()
    }
}
