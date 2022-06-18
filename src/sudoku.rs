use std::{fmt::{Display, Formatter, self}, convert::TryInto};

const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const DIVIDER: &str = "├───────┼───────┼───────┤\n";
const TOP: &str = "┌───────┬───────┬───────┐\n";
const BOTTOM: &str = "└───────┴───────┴───────┘";
const BAR: &str = "│ ";

const BOX_SIZE: usize = 3;
const WIDTH: usize = BOX_SIZE * BOX_SIZE;
const HEIGHT: usize = BOX_SIZE * BOX_SIZE;
const NUM_LOCATIONS: usize = WIDTH * HEIGHT;
#[allow(clippy::cast_possible_truncation)]
const MAX_VALUE: u8 = (BOX_SIZE * BOX_SIZE) as u8;
const EMPTY: u8 = 0;

#[derive(Clone)]
pub struct Board {
    state: [[u8; WIDTH]; WIDTH],
}

struct BoxIterator<'a> {
    row: usize,
    col: usize,
    sentinel: usize,
    board: &'a Board,
}

pub struct GlobalIterator<'a> {
    row: usize,
    col: usize,
    board: &'a Board,
}

impl BoxIterator<'_> {
    const fn new(board: &Board, x: usize) -> BoxIterator {
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

    const fn from_id(board: &Board, x: usize) -> BoxIterator {
        let row = (x / BOX_SIZE) * BOX_SIZE;
        let col = (x % BOX_SIZE) * BOX_SIZE;
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
        if (self.col) % BOX_SIZE == (BOX_SIZE - 1) {
            self.col -= BOX_SIZE - 1;
            self.row += 1;
        } else {
            self.col += 1;
        }
        result
    }
}

impl GlobalIterator<'_> {
    const fn new(board: &Board) -> GlobalIterator {
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

impl Board {
    pub fn from_string(fen: &str) -> Result<Self, String> {
        if !Self::is_string_valid(fen) {
            return Err(format!(
                "input string invalid (you may only use digits and dashes in your input): \"{}\"",
                fen
            ))
        }
        let mut out = Self {
            state: [[0; WIDTH]; WIDTH],
        };
        out.set_from_string(fen);
        if out.current_state_invalid() {
            Err("input sudoku invalid (given problem has repeated digits in rows, columns, or squares).".into())
        } else {
            Ok(out)
        }
    }

    fn set_from_string(&mut self, fen: &str) {
        for (i, c) in fen.chars().enumerate() {
            if c != '-' {
                self.state[i / WIDTH][i % WIDTH] =
                    c.to_digit(10).expect("this should have been validated >:(").try_into().unwrap();
            }
        }
    }

    fn is_string_valid(fen: &str) -> bool {
        const LEGALS: [char; 10] = ['-', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        fen.chars().all(|c| LEGALS.contains(&c))
    }

    const fn is_unassigned(&self, loc: usize) -> bool {
        self.state[loc / WIDTH][loc % WIDTH] == 0
    }

    const fn is_unassigned_xy(&self, x: usize, y: usize) -> bool {
        self.state[x][y] == 0
    }

    fn constraints(&self, loc: usize) -> usize {
        let num_constraints_in_row = self.state[loc / WIDTH].iter().filter(|v| **v != 0).count();
        let num_constraints_in_col = self
            .state
            .iter()
            .map(|row| row[loc % WIDTH])
            .filter(|v| *v != 0)
            .count();
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

    pub fn most_constrained(&mut self) -> Option<usize> {
        let mut max = 0;
        let mut loc = None;
        for i in 0..NUM_LOCATIONS {
            if self.is_unassigned(i) && (self.constraints(i) > max || loc.is_none()) {
                max = self.constraints(i);
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

    pub const fn iter(&self) -> GlobalIterator {
        GlobalIterator::new(self)
    }

    const fn box_iter(&self, x: usize) -> BoxIterator {
        BoxIterator::new(self, x)
    }

    fn legal(&self, x: usize, num: u8) -> bool {
        self.state[x / WIDTH].iter().all(|n| *n != num)
            && self.state.iter().all(|row| row[x % WIDTH] != num)
            && self.box_iter(x).all(|n| n != num)
    }

    fn legal_xy(&self, x: usize, y: usize, num: u8) -> bool {
        self.state[x].iter().all(|n| *n != num)
            && self.state.iter().all(|row| row[y] != num)
            && self.box_iter(x * WIDTH + y).all(|n| n != num)
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

    fn fill_only_in_box(&mut self, box_num: usize) -> bool {
        // for each option, count how many squares could legally have that option
        // if there is only one such square, fill it in.
        let row_tl = (box_num / BOX_SIZE) * BOX_SIZE;
        let col_tl = (box_num % BOX_SIZE) * BOX_SIZE;
        let mut mutated = false;
        for num in 1..=MAX_VALUE {
            let mut found_single_option = false;
            let mut row = 0;
            let mut col = 0;
            for (l, v) in BoxIterator::from_id(self, box_num).enumerate() {
                if v != UNASSIGNED {
                    continue;
                }
                let r = row_tl + l / BOX_SIZE;
                let c = col_tl + l % BOX_SIZE;
                if self.legal_xy(r, c, num) {
                    if found_single_option {
                        found_single_option = false;
                        break;
                    }
                    found_single_option = true;
                    row = r;
                    col = c;
                }
            }
            if found_single_option {
                debug_assert!(self.state[row][col] == UNASSIGNED);
                self.state[row][col] = num;
                mutated = true;
            }
        }
        // return whether we mutated the board at all
        mutated
    }

    fn fill_only_in_row(&mut self, row: usize) -> bool {
        // for each option, count how many squares could legally have that option
        // if there is only one such square, fill it in.
        let mut mutated = false;
        for num in 1..=MAX_VALUE {
            let mut found_single_option = false;
            let mut col = 0;
            for (c, &v) in self.state[row].iter().enumerate() {
                if v != UNASSIGNED {
                    continue;
                }
                if self.legal_xy(row, c, num) {
                    if found_single_option {
                        found_single_option = false;
                        break;
                    }
                    found_single_option = true;
                    col = c;
                }
            }
            if found_single_option {
                debug_assert!(self.state[row][col] == UNASSIGNED);
                self.state[row][col] = num;
                mutated = true;
            }
        }
        // return whether we mutated the board at all
        mutated
    }

    fn fill_only_in_col(&mut self, col: usize) -> bool {
        // for each option, count how many squares could legally have that option
        // if there is only one such square, fill it in.
        let mut mutated = false;
        for num in 1..=MAX_VALUE {
            let mut found_single_option = false;
            let mut row = 0;
            for (r, v) in self.state.iter().map(|slice| slice[col]).enumerate() {
                if v != UNASSIGNED {
                    continue;
                }
                if self.legal_xy(r, col, num) {
                    if found_single_option {
                        found_single_option = false;
                        break;
                    }
                    found_single_option = true;
                    row = r;
                }
            }
            if found_single_option {
                debug_assert!(self.state[row][col] == UNASSIGNED);
                self.state[row][col] = num;
                mutated = true;
            }
        }
        // return whether we mutated the board at all
        mutated
    }

    pub fn solve_dfs(&mut self) -> bool {
        let x = match self.most_constrained() {
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

    pub fn preproc(&mut self) {
        while self.fill_trivial() {
            // keep filling in trivial squares until we can't do any more
        }
        let mut change_made = true;
        while change_made {
            change_made = (0..WIDTH).any(|i| {
                let a = self.fill_only_in_box(i);
                let b = self.fill_only_in_row(i);
                let c = self.fill_only_in_col(i);
                a || b || c
            });
        }
    }

    pub fn solve(&mut self) -> bool {
        assert!(!self.current_state_invalid());
        self.preproc();
        assert!(!self.current_state_invalid());
        self.solve_dfs()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut out = String::with_capacity(WIDTH * WIDTH * 10);
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
        write!(f, "{}", out)
    }
}