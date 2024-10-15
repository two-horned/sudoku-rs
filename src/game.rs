use std::{fmt, str::FromStr};

static LOOKUP: [(u8, u8, u8); 81] = {
    let mut tmp = [(0, 0, 0); 81];
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        let sqr_row = i - i % 3;
        while j < 9 {
            let idx = (i * 9 + j) as usize;
            let sqr = sqr_row + j / 3;
            tmp[idx] = (i, j, sqr);
            j += 1;
        }
        i += 1;
    }
    tmp
};

impl Game {
    pub fn unsafe_choose(&mut self, idx: usize, val: u8) {
        self.board[idx] = val;
        self.update_weight_vectors_and_masks(idx, 1 << (val - 1));
    }

    fn decrease_weight(&mut self, idx: u8, w: usize) {
        //println!("Weights: {:?}", self.weight_vectors);
        let len = self.weight_vectors[w].len();
        for i in 0..len {
            if self.weight_vectors[w][i] == idx {
                self.weight_vectors[w][i] = self.weight_vectors[w][len - 1];
                self.weight_vectors[w].pop();
                self.weight_vectors[w - 1].push(idx);
                return;
            }
        }
        unreachable!();
    }

    fn update_weight_vectors_and_masks(&mut self, idx: usize, mask: u16) {
        let (row, col, sqr) = {
            let (i, j, k) = LOOKUP[idx];
            (i as usize, j as usize, k as usize)
        };

        let mut c;
        c = (row * 9) as usize;
        for _ in 0..9 {
            c += 1;
            if self.board[0] != 0 {
                continue;
            }

            let (i, j, k) = {
                let (i, j, k) = LOOKUP[c - 1];
                (i as usize, j as usize, k as usize)
            };
            let num = self.row_masks[i] | self.col_masks[j] | self.sqr_masks[k];
            if num & mask != 0 {
                let w = weight(num);
                self.decrease_weight(c as u8, w as usize);
            }
        }

        c = col as usize;
        for _ in 0..9 {
            c += 9;
            if self.board[0] != 0 {
                continue;
            }

            let (i, j, k) = {
                let (i, j, k) = LOOKUP[c - 9];
                (i as usize, j as usize, k as usize)
            };
            let num = self.row_masks[i] | self.col_masks[j] | self.sqr_masks[k];
            if num & mask != 0 {
                let w = weight(num);
                self.decrease_weight(c as u8, w as usize);
            }
        }

        c = (sqr / 3 * 27 + sqr % 3 * 3) as usize;
        for _ in 0..3 {
            for _ in 0..3 {
                c += 1;
                if self.board[0] != 0 {
                    continue;
                }
                let (i, j, k) = {
                    let (i, j, k) = LOOKUP[c];
                    (i as usize, j as usize, k as usize)
                };
                let num = self.row_masks[i] | self.col_masks[j] | self.sqr_masks[k];
                if num & mask != 0 {
                    let w = weight(num);
                    self.decrease_weight(c as u8, w as usize);
                }
            }
            c += 6
        }

        self.row_masks[row] |= mask;
        self.col_masks[col] |= mask;
        self.sqr_masks[sqr] |= mask;
    }

    pub fn showbestfree(&mut self) -> (usize, u16, u8) {
        let mut i = 0;
        while i < 10 && self.weight_vectors[i].len() == 0 {
            i += 1;
        }
        if i < 10 {
            let idx = self.weight_vectors[i].pop().unwrap() as usize;
            let (row, col, sqr) = LOOKUP[idx];
            let num = self.row_masks[row as usize]
                | self.col_masks[col as usize]
                | self.sqr_masks[sqr as usize];
            return (idx, num, i as u8);
        }

        (81, 0, 0)
    }
}

fn weight(mut n: u16) -> u8 {
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
            if board[i] != 0 {
                continue;
            }

            let (row, col, sqr) = LOOKUP[i];
            let num = 1 << board[i] - 1;

            row_masks[row as usize] |= num;
            col_masks[col as usize] |= num;
            sqr_masks[sqr as usize] |= num;
        }

        let mut weight_vectors: [Vec<u8>; 10] = [
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
            Vec::with_capacity(10),
        ];

        for i in 0..81 {
            if board[i] != 0 {
                continue;
            }
            let (row, col, sqr) = LOOKUP[i];
            let num = row_masks[row as usize] | col_masks[col as usize] | sqr_masks[sqr as usize];
            weight_vectors[weight(num) as usize].push(i as u8);
        }

        Ok(Self {
            board,
            row_masks,
            col_masks,
            sqr_masks,
            weight_vectors,
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
    weight_vectors: [Vec<u8>; 10],
}
