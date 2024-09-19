use std::{fmt, str::FromStr};

impl Game {
    pub fn unsafe_choose(&mut self, row: usize, col: usize, val: u8) {
        let num = 1 << val - 1;
        let sqr = row / 3 * 3 + col / 3;

        self.board[row * 9 + col] = val;
        self.row_masks[row] |= num;
        self.col_masks[col] |= num;
        self.sqr_masks[sqr] |= num;
    }

    pub fn showbestfree(&self) -> Option<(usize, usize, Vec<u8>)> {
        let mut min_len = 10;
        let mut idx_vec = None;

        for i in 0..9 {
            for j in 0..9 {
                if self.board[i * 9 + j] != 0 {
                    continue;
                }

                let num = self.row_masks[i] | self.col_masks[j] | self.sqr_masks[i / 3 * 3 + j / 3];

                let u = allowed_digits(num);
                if u.len() < min_len {
                    min_len = u.len();
                    idx_vec = Some((i, j, u));
                    if min_len < 2 {
                        return idx_vec;
                    }
                }
            }
        }
        idx_vec
    }
}

fn allowed_digits(mut num: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(9);

    for i in 1..10 {
        if num & 1 == 0 {
            v.push(i);
        }
        num >>= 1;
    }
    v
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

        let mut row_masks = [0; 9];
        let mut col_masks = [0; 9];
        let mut sqr_masks = [0; 9];

        for i in 0..9 {
            for j in 0..9 {
                let val = board[i * 9 + j];
                if val == 0 {
                    continue;
                }
                let num = 1 << val - 1;
                row_masks[i] |= num;
                col_masks[j] |= num;
                sqr_masks[i / 3 * 3 + j / 3] |= num;
            }
        }

        Ok(Self {
            board,
            row_masks,
            col_masks,
            sqr_masks,
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
    row_masks: [u16; 9],
    col_masks: [u16; 9],
    sqr_masks: [u16; 9],
}
