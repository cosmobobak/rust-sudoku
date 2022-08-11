use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
};

const UNASSIGNED: u8 = 0;
static SYMBOLS: [u8; 37] = *b".1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static BAR: &str = "│ ";

enum StringValidityError {
    InvalidLength { expected: usize, actual: usize },
    CharIllegal(char),
    CharOutOfRange { c: char, max: usize },
}

impl Display for StringValidityError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => write!(f, "string is too long, expected string of at most {expected} characters, got {actual}"),
            Self::CharIllegal(c) => write!(f, "character '{c}' is not a legal character for sudoku strings"),
            Self::CharOutOfRange { c, max } => write!(f, "character '{c}' is out of range, expected characters in \"{}\"", std::str::from_utf8(&SYMBOLS[..*max]).unwrap()),
        }
    }
}

#[derive(Clone)]
pub struct Board<const BOX_SIZE: usize> {
    state: Vec<u8>,
}

struct BoxIterator<'a, const BOX_SIZE: usize> {
    row: usize,
    col: usize,
    sentinel: usize,
    board: &'a Board<BOX_SIZE>,
}

pub struct GlobalIterator<'a, const BOX_SIZE: usize> {
    row: usize,
    col: usize,
    board: &'a Board<BOX_SIZE>,
}

impl<'a, const BOX_SIZE: usize> BoxIterator<'a, BOX_SIZE> {
    const WIDTH: usize = BOX_SIZE * BOX_SIZE;

    const fn new(board: &'a Board<BOX_SIZE>, x: usize) -> Self {
        let row = ((x / Self::WIDTH) / BOX_SIZE) * BOX_SIZE;
        let col = ((x % Self::WIDTH) / BOX_SIZE) * BOX_SIZE;
        let sentinel = row + BOX_SIZE;
        Self {
            row,
            col,
            sentinel,
            board,
        }
    }

    const fn from_id(board: &'a Board<BOX_SIZE>, x: usize) -> Self {
        let row = (x / BOX_SIZE) * BOX_SIZE;
        let col = (x % BOX_SIZE) * BOX_SIZE;
        let sentinel = row + BOX_SIZE;
        Self {
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
            Some(self.board.state[self.row * Self::WIDTH + self.col])
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

impl<'a, const BOX_SIZE: usize> GlobalIterator<'a, BOX_SIZE> {
    const WIDTH: usize = BOX_SIZE * BOX_SIZE;
    const HEIGHT: usize = BOX_SIZE * BOX_SIZE;

    const fn new(board: &'a Board<BOX_SIZE>) -> Self {
        Self {
            row: 0,
            col: 0,
            board,
        }
    }

    fn next(&mut self) -> Option<u8> {
        let result = if self.row == Self::HEIGHT {
            None
        } else {
            Some(self.board.state[self.row * Self::WIDTH + self.col])
        };
        if self.col == (Self::WIDTH - 1) {
            self.col = 0;
            self.row += 1;
        } else {
            self.col += 1;
        }
        result
    }
}

impl<const BOX_SIZE: usize> Iterator for BoxIterator<'_, BOX_SIZE> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        BoxIterator::next(self)
    }
}

impl<const BOX_SIZE: usize> Iterator for GlobalIterator<'_, BOX_SIZE> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        GlobalIterator::next(self)
    }
}

impl<const BOX_SIZE: usize> Board<BOX_SIZE> {
    const WIDTH: usize = BOX_SIZE * BOX_SIZE;
    const NUM_LOCATIONS: usize = Self::WIDTH * Self::WIDTH;
    #[allow(clippy::cast_possible_truncation)]
    const MAX_VALUE: u8 = Self::WIDTH as u8;

    pub fn from_str(fen: &str) -> Result<Self, String> {
        if let Err(e) = Self::is_string_valid(fen) {
            return Err(format!("input string invalid: {e}",));
        }
        let mut out = Self {
            state: vec![UNASSIGNED; Self::NUM_LOCATIONS],
        };
        out.set_from_string(fen);
        if out.current_state_invalid() {
            Err("input sudoku invalid (given problem has repeated digits in rows, columns, or squares).".into())
        } else {
            Ok(out)
        }
    }

    fn set_from_string(&mut self, fen: &str) {
        self.state.fill(UNASSIGNED);
        for (i, c) in fen.chars().enumerate() {
            if c != '-' {
                self.state[i] = SYMBOLS
                    .iter()
                    .position(|&x| x == c as u8)
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
        }
    }

    fn is_string_valid(fen: &str) -> Result<(), StringValidityError> {
        if fen.len() > Self::NUM_LOCATIONS {
            return Err(StringValidityError::InvalidLength {
                expected: Self::NUM_LOCATIONS,
                actual: fen.len(),
            });
        }
        for c in fen.chars() {
            let idx_in_symbols = SYMBOLS.iter().position(|&x| x as char == c);
            if idx_in_symbols.is_none() {
                return Err(StringValidityError::CharIllegal(c));
            }
            if idx_in_symbols.unwrap() > Self::WIDTH {
                return Err(StringValidityError::CharOutOfRange {
                    c,
                    max: Self::WIDTH,
                });
            }
        }
        Ok(())
    }

    fn is_unassigned(&self, loc: usize) -> bool {
        self.state[loc] == 0
    }

    fn constraints(&self, loc: usize) -> usize {
        let start_of_row = loc / Self::WIDTH * Self::WIDTH;
        let row = &self.state[start_of_row..start_of_row + Self::WIDTH];
        let num_constraints_in_row = row.iter().filter(|v| **v != 0).count();
        let start_of_col = loc % Self::WIDTH;
        let num_constraints_in_col = self
            .state
            .iter()
            .skip(start_of_col)
            .step_by(Self::WIDTH)
            .take(Self::WIDTH)
            .filter(|v| **v != 0)
            .count();
        debug_assert_eq!(
            self.state
                .iter()
                .skip(start_of_col)
                .step_by(Self::WIDTH)
                .take(Self::WIDTH)
                .count(),
            Self::WIDTH
        );
        num_constraints_in_row + num_constraints_in_col
    }

    pub fn most_constrained(&mut self) -> Option<usize> {
        let mut max = 0;
        let mut loc = None;
        for i in 0..Self::NUM_LOCATIONS {
            if self.is_unassigned(i) && (self.constraints(i) > max || loc.is_none()) {
                max = self.constraints(i);
                loc = Some(i);
            }
        }
        loc
    }

    fn current_state_invalid(&mut self) -> bool {
        for i in 0..Self::NUM_LOCATIONS {
            let n = self.state[i];
            if n != UNASSIGNED {
                self.state[i] = 0;
                if !self.legal(i, n) {
                    return true;
                }
                self.state[i] = n;
            }
        }
        false
    }

    pub const fn iter(&self) -> GlobalIterator<BOX_SIZE> {
        GlobalIterator::new(self)
    }

    const fn box_iter(&self, x: usize) -> BoxIterator<BOX_SIZE> {
        BoxIterator::new(self, x)
    }

    fn row_iter(&self, loc: usize) -> impl Iterator<Item = &u8> {
        let row_start = loc / Self::WIDTH * Self::WIDTH;
        self.state[row_start..row_start + Self::WIDTH].iter()
    }

    fn col_iter(&self, loc: usize) -> impl Iterator<Item = &u8> {
        let col_start = loc % Self::WIDTH;
        self.state
            .iter()
            .skip(col_start)
            .step_by(Self::WIDTH)
            .take(Self::WIDTH)
    }

    fn legal(&self, x: usize, num: u8) -> bool {
        self.row_iter(x).all(|n| *n != num)
            && self.col_iter(x).all(|n| *n != num)
            && self.box_iter(x).all(|n| n != num)
    }

    fn legal_xy(&self, x: usize, y: usize, num: u8) -> bool {
        let x = x * Self::WIDTH + y;
        self.row_iter(x).all(|n| *n != num)
            && self.col_iter(x).all(|n| *n != num)
            && self.box_iter(x).all(|n| n != num)
    }

    fn options_for_loc(&self, x: usize) -> usize {
        let mut options = 0;
        for num in 1..=Self::MAX_VALUE {
            if self.legal(x, num) {
                options += 1;
            }
        }
        options
    }

    fn first_legal_for_loc(&self, x: usize) -> Option<u8> {
        for num in 1..=Self::MAX_VALUE {
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
                self.state[x] = self.first_legal_for_loc(x).unwrap();
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
        for num in 1..=Self::MAX_VALUE {
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
                debug_assert!(self.state[row * Self::WIDTH + col] == UNASSIGNED);
                self.state[row * Self::WIDTH + col] = num;
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
        for num in 1..=Self::MAX_VALUE {
            let mut found_single_option = false;
            let mut col = 0;
            for (c, &v) in self.state[row * Self::WIDTH..(row + 1) * Self::WIDTH]
                .iter()
                .enumerate()
            {
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
                debug_assert!(self.state[row * Self::WIDTH + col] == UNASSIGNED);
                self.state[row * Self::WIDTH + col] = num;
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
        for num in 1..=Self::MAX_VALUE {
            let mut found_single_option = false;
            let mut row = 0;
            for (r, &v) in self
                .state
                .iter()
                .skip(col)
                .step_by(Self::WIDTH)
                .take(Self::WIDTH)
                .enumerate()
            {
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
                debug_assert!(self.state[row * Self::WIDTH + col] == UNASSIGNED);
                self.state[row * Self::WIDTH + col] = num;
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

        for num in 1..=Self::MAX_VALUE {
            if self.legal(x, num) {
                self.state[x] = num;
                if self.solve_dfs() {
                    return true;
                }
                self.state[x] = UNASSIGNED;
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
            change_made = (0..Self::WIDTH).any(|i| {
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

fn make_divider(left: char, middle: char, right: char, box_size: usize) -> String {
    let n_boxes_along_edge = box_size;
    let divider_bar_width = box_size * 2 + 1;
    let mut s = left.to_string();
    for _ in 0..divider_bar_width {
        s.push('─');
    }
    for _ in 0..n_boxes_along_edge - 1 {
        s.push(middle);
        for _ in 0..divider_bar_width {
            s.push('─');
        }
    }
    s.push(right);
    s.push('\n');
    s
}

impl<const BOX_SIZE: usize> Display for Board<BOX_SIZE> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let top = make_divider('┌', '┬', '┐', BOX_SIZE);
        let mid = make_divider('├', '┼', '┤', BOX_SIZE);
        let bot = make_divider('└', '┴', '┘', BOX_SIZE);
        let mut out = String::with_capacity(Self::WIDTH * Self::WIDTH * 10);
        out.push('\n');
        out.push_str(&top);
        for (i, &v) in self.state.iter().enumerate() {
            let j = i % Self::WIDTH;
            let i = i / Self::WIDTH;
            if j == 0 {
                out.push_str(BAR);
            }

            out.push_str(format!("{} ", SYMBOLS[v as usize] as char).as_str());
            if j % BOX_SIZE == (BOX_SIZE - 1) && j != (Self::WIDTH - 1) {
                out.push_str(BAR);
            }

            if j == (Self::WIDTH - 1) {
                out.push_str(BAR);
                out.push('\n');
                if i % BOX_SIZE == (BOX_SIZE - 1) && i != (Self::WIDTH - 1) {
                    out.push_str(&mid);
                };
            }
        }
        out.push_str(&bot);
        write!(f, "{}", out)
    }
}
