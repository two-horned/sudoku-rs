use std::{fmt, str::FromStr};

impl Game {
    pub fn unsafe_choose(&mut self, idx: usize, val: u8) {
        let (row, col, sqr) = mask_indices(idx);
        let num = 1 << val - 1;

        self.board[idx] = val;
        self.masks[row] |= num;
        self.masks[col] |= num;
        self.masks[sqr] |= num;
    }

    pub fn showbestfree(&self) -> Option<(usize, Vec<u8>)> {
        let mut min_len = 10;
        let mut idx_vec = None;

        for i in 0..81 {
            if self.board[i] != 0 {
                continue;
            }

            let (row, col, sqr) = mask_indices(i);
            let num = self.masks[row] | self.masks[col] | self.masks[sqr];

            let u = allowed_digits(num);
            if u.len() < min_len {
                min_len = u.len();
                idx_vec = Some((i, u));
            }
        }
        idx_vec
    }

    pub fn showbestneed(&self) -> Option<(u8, Vec<usize>)> {
        let mut min_len = 10;
        let mut idx_vec = None;

        for i in 0..27 {
            for d in allowed_digits(self.masks[i]) {
                let v = where_other_houses_allow_digit(i, d, self.masks);
                if v.len() < min_len {
                    min_len = v.len();
                    idx_vec = Some((d, v));
                }
            }
        }
        idx_vec
    }
}

fn where_other_houses_allow_digit(house: usize, digit: u8, masks: [u16; 27]) -> Vec<usize> {
    let num: u16 = 1 << digit - 1;
    let mut v = Vec::with_capacity(10);

    if house < 9 {
        for col in 0..9 {
            let calc = masks[9 + col] | masks[18 + (house / 3) * 3 + col / 3];
            if calc & num == 0 {
                v.push(house * 9 + col);
            }
        }
    } else if house < 18 {
        let col = house - 9;
        for row in 0..9 {
            let calc = masks[row] | masks[18 + (row / 3) * 3 + col / 3];
            if calc & num == 0 {
                v.push(row * 9 + col);
            }
        }
    } else {
        let sqr = house - 18;
        for i in 0..9 {
            let row = (sqr / 3) * 3 + i / 3;
            let col = (sqr % 3) * 3 + i % 3;
            let calc = masks[row] | masks[9 + col];
            if calc & num == 0 {
                v.push(row * 9 + col);
            }
        }
    }
    v
}

fn allowed_digits(mut num: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(10);

    for i in 1..10 {
        if num & 1 == 0 {
            v.push(i);
        }
        num >>= 1;
    }
    v
}

fn mask_indices(idx: usize) -> (usize, usize, usize) {
    let row = idx / 9;
    let col = idx % 9;
    let sqr_row = row / 3;
    let sqr_col = col / 3;

    (row, 9 + col, 18 + sqr_row * 3 + sqr_col)
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

        let mut masks = [0; 27];

        for i in 0..81 {
            if board[i] == 0 {
                continue;
            }
            let (row, col, sqr) = mask_indices(i);
            let num = 1 << board[i] - 1;
            masks[row] |= num;
            masks[col] |= num;
            masks[sqr] |= num;
        }

        Ok(Self { board, masks })
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
    masks: [u16; 27],
}
