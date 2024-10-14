use std::{fmt, str::FromStr};

impl Game {
    pub fn unsafe_choose(&mut self, row: usize, col: usize, val: u8) {
        let num = 1 << val - 1;
        let sqr = row - row % 3 + col / 3;

        self.board[row * 9 + col] = val;
        self.row_masks[row] |= num;
        self.col_masks[col] |= num;
        self.sqr_masks[sqr] |= num;
    }

    pub fn collective_showbestfree(&self) -> (usize, u16, Vec<(usize, usize)>) {
        let mut w = 11;
        let mut n = 0;
        let mut v = Vec::with_capacity(81);

        for i in 0..9 {
            let sqr_row = i - i % 3;
            let nine_i = i * 9;

            for j in 0..9 {
                let idx = nine_i + j;
                let sqr = sqr_row + j / 3;

                self.__update_row_col_nums(&mut v, &mut w, &mut n, idx, i, j, sqr);
            }
        }
        (w, n, v)
    }

    pub fn collective_showbestfree_local(
        &self,
        row: usize,
        col: usize,
    ) -> (usize, u16, Vec<(usize, usize)>) {
        let mut w = 11;
        let mut n = 0;
        let mut v = Vec::with_capacity(81);

        let sqr_row = row - row % 3;
        let nine_i = row * 9;
        for j in 0..9 {
            let r_idx = nine_i + j;
            let r_sqr = sqr_row + j / 3;
            self.__update_row_col_nums(&mut v, &mut w, &mut n, r_idx, row, j, r_sqr)
        }

        let sqr_col = col / 3;
        for i in 0..9 {
            let c_idx = i * 9 + col;
            let c_sqr = i - i % 3 + sqr_col;
            self.__update_row_col_nums(&mut v, &mut w, &mut n, c_idx, i, col, c_sqr)
        }

        let sqr = sqr_row + sqr_col;
        for i in 0..3 {
            let s_row = i + sqr - sqr % 3;
            for j in 0..3 {
                let s_col = j + sqr % 3 * 3;
                let s_idx = s_row * 9 + s_col;
                self.__update_row_col_nums(&mut v, &mut w, &mut n, s_idx, s_row, s_col, sqr)
            }
        }
        (w, n, v)
    }

    pub fn collective_showbestfree_list<I>(&self, it: I) -> (usize, u16, Vec<(usize, usize)>)
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let mut w = 11;
        let mut n = 0;
        let mut v = Vec::with_capacity(81);

        for (i, j) in it {
            let idx = i * 9 + j;
            let sqr = i - i % 3 + j / 3;
            self.__update_row_col_nums(&mut v, &mut w, &mut n, idx, i, j, sqr);
        }

        (w, n, v)
    }

    pub fn calc_num(&self, row: usize, col: usize) -> u16 {
        self.row_masks[row] | self.col_masks[col] | self.sqr_masks[row - row % 3 + col / 3]
    }

    fn __update_row_col_nums(
        &self,
        rcn: &mut Vec<(usize, usize)>,
        min_wgt: &mut usize,
        cnum: &mut u16,
        idx: usize,
        row: usize,
        col: usize,
        sqr: usize,
    ) {
        assert!(idx == row * 9 + col);
        assert!(sqr == row - row % 3 + col / 3);

        match self.board[idx] {
            0 => (),
            _ => return,
        }

        let num = self.row_masks[row] | self.col_masks[col] | self.sqr_masks[sqr];
        let wgt = weight(num);

        if wgt == *min_wgt {
            rcn.push((row, col));
            *cnum = num;
        } else if wgt < *min_wgt {
            *min_wgt = wgt;
            *cnum = num;
            rcn.clear();
            rcn.push((row, col));
        }
    }
}

fn weight(mut num: u16) -> usize {
    let mut w = 0;

    for _ in 1..10 {
        match num & 1 {
            0 => w += 1,
            _ => (),
        }
        num >>= 1;
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

        let mut row_masks = [0; 9];
        let mut col_masks = [0; 9];
        let mut sqr_masks = [0; 9];

        for i in 0..9 {
            for j in 0..9 {
                let val = board[i * 9 + j];
                match val {
                    0 => continue,
                    _ => (),
                }
                let num = 1 << val - 1;
                row_masks[i] |= num;
                col_masks[j] |= num;
                sqr_masks[i - i % 3 + j / 3] |= num;
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
