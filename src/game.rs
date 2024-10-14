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
        self.update_local_nums(idx, 1 << (val - 1));
    }

    fn update_local_nums(&mut self, idx: usize, mask: u16) {
        let (row, col, sqr) = LOOKUP[idx];
        let mut c;

        c = (row * 9) as usize;
        for _ in 0..9 {
            if self.nums[c] & mask == 0 {
                self.nums[c] |= mask;
                self.weights[c] -= 1;
            }
            c += 1;
        }

        c = col as usize;
        for _ in 0..9 {
            if self.nums[c] & mask == 0 {
                self.nums[c] |= mask;
                self.weights[c] -= 1;
            }
            c += 9;
        }

        c = (sqr / 3 * 27 + sqr % 3 * 3) as usize;
        for _ in 0..3 {
            for _ in 0..3 {
                if self.nums[c] & mask == 0 {
                    self.nums[c] |= mask;
                    self.weights[c] -= 1;
                }
                c += 1;
            }
            c += 6
        }
    }

    pub fn showbestfree(&self) -> (usize, u16) {
        let mut min_w = 11;
        let mut min_n = 0;
        let mut min_i = 81;

        for i in 0..81 {
            if self.board[i] != 0 {
                continue;
            }

            if self.weights[i] < min_w {
                min_w = self.weights[i];
                min_n = self.nums[i];
                min_i = i;
            }
        }

        (min_i, min_n)
    }
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

        let weights = [10; 81];
        let nums = [0; 81];
        let mut tmp = Self {
            board,
            weights,
            nums,
        };

        for i in 0..81 {
            match board[i] {
                0 => continue,
                x => tmp.unsafe_choose(i, x),
            }
        }

        Ok(tmp)
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
    weights: [u8; 81],
    nums: [u16; 81],
}
