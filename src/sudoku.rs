const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const VALID_TOKENS: [char; 10] = ['-', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Clone)]
pub struct SudokuBoard {
    state: [[u8; 9]; 9]
}

struct BoxIterator<'a> {
    row: usize,
    col: usize,
    sentinel: usize,
    board: &'a SudokuBoard
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
            board
        }
    }

    fn next(&mut self) -> Option<u8> {
        if (self.col) % 3 == 2 {
            self.col -= 2;
            self.row += 1;
        } else {
            self.col += 1;
        }
        if self.row == self.sentinel {
            None 
        } else {
            Some(self.board.state[self.row][self.col])
        }
    }
}

impl Iterator for BoxIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        BoxIterator::next(self)
    }
}

impl Default for SudokuBoard {
    fn default() -> SudokuBoard {
        SudokuBoard { state: [[0; 9]; 9] }
    }  
}

impl SudokuBoard {
    pub fn from_string(fen: &str) -> Result<SudokuBoard, String> {
        if !SudokuBoard::is_string_valid(fen) {
            return Err(format!("input string invalid (you may only use digits and dashes in your input): \"{}\"", fen));
        }
        let mut out = SudokuBoard::default();
        out.set_from_string(fen);
        match out.current_state_invalid() {
            false => Ok(out),
            true => Err("input sudoku invalid (given problem has repeated digits in rows, columns, or squares).".to_string())
        }
    }

    fn set_from_string(&mut self, fen: &str) {
        for (i, c) in fen.chars().enumerate() {
            if c != '-' {
                self.state[i / 9][i % 9] = c.to_digit(10).expect("this should have been validated >:(") as u8;
            }
        }
    }

    fn is_string_valid(fen: &str) -> bool {
        return fen.chars().all(|c| VALID_TOKENS.iter().any(|token| *token == c));
    }

    pub fn show(&self) {
        for row in self.state {
            for num in row {
                print!("{} ", SYMBOLS[num as usize]);
            }
            println!();
        }
        println!();
    }

    pub fn first_unassigned(&self) -> usize {
        for i in 0..81 {
            if self.state[i / 9][i % 9] == 0u8 {
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

    fn box_iter(&self, x: usize) -> BoxIterator {
        BoxIterator::new(x, self)
    }

    fn legal(&self, x: usize, num: u8) -> bool {
        return self.state[x / 9].iter()
                         .all(|n| *n != num) &&
               self.state.iter()
                         .all(|row| row[x % 9] != num) &&
               self.box_iter(x)
                         .all(|n| n != num);
    }

    pub fn solve(&mut self) -> bool {
        let x = self.first_unassigned();
        // If there is no unassigned location, we are done
        if x == 81 {
            return true;  // success!
        }
        for num in 1..10 {
            if self.legal(x, num) {
                self.state[x / 9][x % 9] = num;
                if self.solve() {
                    return true;
                }
                self.state[x / 9][x % 9] = UNASSIGNED;
            }
        }
        false  // this triggers backtracking
    }
}