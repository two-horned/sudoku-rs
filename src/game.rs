use std::{fmt, str::FromStr};

static LOOKUP: [(usize, usize, usize); 81] = {
    let mut tmp = [(0, 0, 0); 81];

    let mut i = 0;
    let mut j;
    while i < 9 {
        j = 0;
        let sqr_row = i - i % 3;
        while j < 9 {
            let idx = i * 9 + j;
            let sqr = sqr_row + j / 3;
            tmp[idx] = (i, j, sqr);
            j += 1;
        }
        i += 1;
    }
    tmp
};

fn lookup(idx: usize) -> (usize, usize, usize) {
    LOOKUP[idx]
}

impl Game {
    pub fn unsafe_choose(&mut self, idx: usize, val: u8) {
        self.last_placed = idx;
        self.board[idx] = val;
        self.update_weight_vectors_and_masks(idx, 1 << (val - 1));
    }

    fn decrease_weight_if_needed(&mut self, idx: usize, mask: u16) {
        if self.board[idx] != 0 {
            return;
        }

        let (i, j, k) = lookup(idx);
        let num = self.row_masks[i] | self.col_masks[j] | self.sqr_masks[k];

        if num & mask != 0 {
            return;
        }

        let w = weight(num);

        let len = self.weight_vectors[w].len();
        for i in 0..len {
            if self.weight_vectors[w][i] == idx {
                //println!("Zuerst {}", self.weight_vectors[w][i]);
                self.weight_vectors[w][i] = self.weight_vectors[w][len - 1];
                //println!("Zuletzt {}", self.weight_vectors[w][i]);
                self.weight_vectors[w].pop();
                self.weight_vectors[w - 1].push(idx);
                return;
            }
        }
        panic!("WHAT IS WITH {}?", idx);
    }

    fn update_weight_vectors_and_masks(&mut self, idx: usize, mask: u16) {
        let (row, col, sqr) = lookup(idx);

        for c in (row * 9)..((row + 1) * 9) {
            self.decrease_weight_if_needed(c, mask);
        }

        for c in (col..81).step_by(9) {
            self.decrease_weight_if_needed(c, mask);
        }

        self.row_masks[row] |= mask;
        self.col_masks[col] |= mask;

        let mut c;
        c = sqr / 3 * 27 + sqr % 3 * 3;
        for _ in 0..3 {
            for _ in 0..3 {
                self.decrease_weight_if_needed(c, mask);
                c += 1;
            }
            c += 6
        }

        self.sqr_masks[sqr] |= mask;
    }

    pub fn showbestfree(&mut self) -> (usize, u16, usize) {
        let min_d = (2_f64 * 9_f64.powi(2)).sqrt();
        let (xx, yy, _) = lookup(self.last_placed);
        for i in 0..10 {
            let len = self.weight_vectors[i].len();
            if len == 0 {
                continue;
            }

            let mut min_j = 0;
            for j in 0..len {
                let idx = self.weight_vectors[i][j];
                let (x, y, _) = lookup(idx);
                if (((xx - x) as f64).abs().powi(2) + ((yy - y) as f64).abs().powi(2)).sqrt()
                    < min_d
                {
                    min_j = j;
                }
            }
            let min_idx = self.weight_vectors[i][min_j];
            self.weight_vectors[i][min_j] = self.weight_vectors[i][len - 1];
            self.weight_vectors[i].pop();

            let (row, col, sqr) = lookup(min_idx);
            let num = self.row_masks[row] | self.col_masks[col] | self.sqr_masks[sqr];
            return (min_idx, num, i);
        }

        (81, 0, 0)
    }
}

fn weight(mut n: u16) -> usize {
    let mut w = 0;
    for _ in 0..9 {
        if n & 1 == 0 {
            w += 1;
        }
        n >>= 1;
    }
    w
}

fn to_dig(c: char) -> Result<u8, ParseGameError> {
    match c {
        '.' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(ParseGameError::UnknownCharacter(c)),
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: [u8; 81] = {
            let chars: Vec<u8> = s.chars().map(to_dig).collect::<Result<Vec<u8>, _>>()?;
            match chars.try_into() {
                Err(_) => return Err(ParseGameError::IncorrectLength),
                Ok(x) => x,
            }
        };

        let mut row_masks = [0; 27];
        let mut col_masks = [0; 27];
        let mut sqr_masks = [0; 27];

        for i in 0..81 {
            if board[i] == 0 {
                continue;
            }

            let (row, col, sqr) = lookup(i);
            let num = 1 << (board[i] - 1);

            row_masks[row] |= num;
            col_masks[col] |= num;
            sqr_masks[sqr] |= num;
        }

        let mut weight_vectors = [
            Vec::with_capacity(9),
            Vec::with_capacity(18),
            Vec::with_capacity(27),
            Vec::with_capacity(36),
            Vec::with_capacity(40),
            Vec::with_capacity(40),
            Vec::with_capacity(40),
            Vec::with_capacity(40),
            Vec::with_capacity(40),
            Vec::with_capacity(40),
        ];

        for i in 0..81 {
            if board[i] != 0 {
                continue;
            }
            let (row, col, sqr) = lookup(i);
            let num = row_masks[row] | col_masks[col] | sqr_masks[sqr];
            weight_vectors[weight(num)].push(i);
        }

        Ok(Self {
            board,
            row_masks,
            col_masks,
            sqr_masks,
            weight_vectors,
            last_placed: 41,
        })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(81);

        for i in self.board {
            s += match i {
                0 => ".",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                _ => unreachable!(),
            };
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ParseGameError {
    IncorrectLength,
    UnknownCharacter(char),
}

#[derive(Clone)]
pub struct Game {
    board: [u8; 81],
    row_masks: [u16; 27],
    col_masks: [u16; 27],
    sqr_masks: [u16; 27],
    weight_vectors: [Vec<usize>; 10],
    last_placed: usize,
}
