use array2d::Array2D;

const UNASSIGNED: u8 = 0;
const SYMBOLS: [char; 10] = ['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const VALID_TOKENS: [char; 10] = ['-', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Clone)]
pub struct SudokuBoard {
    state: Array2D<u8>
}

pub fn square(state: &Array2D<u8>, x: usize) -> [u8; 9] {
    let rstart = ((x / 9) / 3) * 3;
    let cstart = ((x % 9) / 3) * 3;
    let mut out = [0u8; 9];
    for row in rstart..(3+rstart) {
        for col in cstart..(3+cstart) {
            out[(row - rstart) * 3 + (col - cstart)] = state[(row, col)];
        }
    }
    out
}

impl Default for SudokuBoard {
    fn default() -> SudokuBoard {
        SudokuBoard { state: Array2D::filled_with(0u8, 9, 9) }
    }  
}

impl SudokuBoard {
    pub fn from_string(fen: &str) -> SudokuBoard {
        let mut out = SudokuBoard::default();
        out.set_from_string(fen);
        out
    }

    fn set_from_string(&mut self, fen: &str) {
        for (i, c) in fen.chars().enumerate() {
            if c != '-' {
                self.state[(i / 9, i % 9)] = c.to_digit(10).expect("this should have been validated >:(") as u8;
            }
        }
    }

    pub fn is_string_valid(fen: &str) -> bool {
        return fen.chars().all(|c| VALID_TOKENS.iter().any(|token| *token == c));
    }

    pub fn show(&self) {
        for row in self.state.rows_iter() {
            for num in row {
                print!("{} ", SYMBOLS[*num as usize]);
            }
            println!();
        }
        println!();
    }

    pub fn first_unassigned(&self) -> usize {
        for i in 0..81 {
            if self.state[(i / 9, i % 9)] == 0u8 {
                return i;
            }
        }
        81
    }

    pub fn current_state_invalid(&mut self) -> bool {
        let mut n;
        for i in 0..81 {
            n = self.state[(i / 9, i % 9)];
            if n != 0 {
                self.state[(i / 9, i % 9)] = 0;
                if !self.legal(i, n) {
                    return true;
                }
                self.state[(i / 9, i % 9)] = n;
            }
        }
        false
    }

    fn legal(&self, x: usize, num: u8) -> bool {
        return self.state.row_iter(x / 9)
                         .all(|n| *n != num) &&
               self.state.column_iter(x % 9)
                         .all(|n| *n != num) &&
               square(&self.state, x as usize).iter()
                         .all(|n| *n != num);
    }

    pub fn solve(&mut self) -> bool {
        let x = self.first_unassigned();
        // If there is no unassigned location, we are done
        if x == 81 {
            return true;  // success!
        }
        for num in 1..10 {
            if self.legal(x, num) {
                self.state[(x / 9, x % 9)] = num;
                if self.solve() {
                    return true;
                }
                self.state[(x / 9, x % 9)] = UNASSIGNED;
            }
        }
        false  // this triggers backtracking
    }
}