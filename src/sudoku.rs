const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIVIDER: &str = "├───────┼───────┼───────┤\n";
const TOP: &str = "┌───────┬───────┬───────┐\n";
const BOTTOM: &str = "└───────┴───────┴───────┘\n";
const BAR: &str = "│ ";

const BOX_SIZE: usize = 3;
const WIDTH: usize = BOX_SIZE * BOX_SIZE;
const HEIGHT: usize = BOX_SIZE * BOX_SIZE;
const NUM_LOCATIONS: usize = WIDTH * HEIGHT;
const MAX_VALUE: u8 = (BOX_SIZE * BOX_SIZE) as u8;
const EMPTY: u8 = 0;

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
        let row = ((x / WIDTH) / BOX_SIZE) * BOX_SIZE;
        let col = ((x % WIDTH) / BOX_SIZE) * BOX_SIZE;
        let sentinel = row + BOX_SIZE;
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
        if (self.col) % BOX_SIZE == (BOX_SIZE-1) {
            self.col -= BOX_SIZE - 1;
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
        if self.col == (WIDTH - 1) {
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
        const LEGALS: [char; 10] = ['-', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        fen.chars().all(|c| LEGALS.contains(&c))
    }

    pub fn show(&self) {
        let mut out = String::with_capacity(WIDTH*WIDTH*10);
        out.push('\n');
        out.push_str(TOP);
        for (i, row) in self.state.iter().enumerate() {
            out.push_str(BAR);
            for (j, val) in row.iter().enumerate() {
                out.push_str(format!("{} ", SYMBOLS[*val as usize]).as_str());
                if j % BOX_SIZE == (BOX_SIZE - 1) && j != (WIDTH - 1) {
                    out.push_str(BAR);
                }
            }
            out.push_str(BAR);
            out.push('\n');
            if i % BOX_SIZE == (BOX_SIZE - 1) && i != (WIDTH - 1) {
                out.push_str(DIVIDER);
            };
        }
        out.push_str(BOTTOM);
        println!("{}", out);
    }

    fn is_unassigned(&self, loc: usize) -> bool {
        self.state[loc / WIDTH][loc % WIDTH] == 0
    }

    fn score_unassigned(&self, loc: usize) -> usize {
        let num_constraints_in_row = self.state[loc / WIDTH].iter().filter(|v| **v != 0).count();
        let num_constraints_in_col = self.state.iter().map(|row| row[loc % WIDTH]).filter(|v| *v != 0).count();
        num_constraints_in_row + num_constraints_in_col
    }

    pub fn first_unassigned(&self) -> Option<usize> {
        for loc in 0..NUM_LOCATIONS {
            if self.is_unassigned(loc) {
                return Some(loc);
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
            if n != UNASSIGNED {
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
