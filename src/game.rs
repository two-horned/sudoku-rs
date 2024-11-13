use std::{fmt, str::FromStr};

static LOOKUP: [[usize; 4]; 81] = {
    let mut tmp = [[0; 4]; 81];
    let (mut nine_i, mut sqr_row);

    let mut i = 0;
    let mut j;
    while i < 9 {
        j = 0;
        nine_i = i * 9;
        sqr_row = i - i % 3;
        while j < 9 {
            let idx = nine_i + j;
            let sqr = sqr_row + j / 3;
            tmp[idx] = [i, j, sqr, i % 3 * 3 + j % 3];
            j += 1;
        }
        i += 1;
    }
    tmp
};

static REV_LOOKUP: [[[usize; 9]; 9]; 3] = {
    let mut tmp = [[[0; 9]; 9]; 3];
    let mut idx = 0;
    while idx < 81 {
        let [i, j, k, l] = LOOKUP[idx];
        tmp[0][i][j] = idx;
        tmp[1][j][i] = idx;
        tmp[2][k][l] = idx;
        idx += 1;
    }
    tmp
};

impl Game {
    fn init_board(board: [u8; 81]) -> Self {
        let mut tmp = Self {
            board,
            house_masks: [[0xFE00; 9]; 3],
            val_house_pos_indices: [[[0xFE00; 9]; 3]; 10],
            showbestfree: BestFree::SOME {
                weight: 9,
                value: BestFreeVal::INDEX(0),
            },
        };

        for i in 0..81 {
            let val = board[i];
            if val != 0 {
                tmp.update_masks(i, val as usize);
            }
        }

        tmp.update_showbestfree();
        tmp
    }

    pub fn unsafe_choose_alt(&mut self, vht: [usize; 3], idx: usize) {
        let [val, ht, hi] = vht;
        self.unsafe_choose(REV_LOOKUP[ht][hi][idx], val);
    }

    pub fn unsafe_choose(&mut self, idx: usize, val: usize) {
        self.board[idx] = val as u8;
        self.update_masks(idx, val);
        self.update_showbestfree();
    }

    fn update_masks(&mut self, idx: usize, val: usize) {
        let mask = 1 << (val - 1);
        let houses = LOOKUP[idx];
        for ht in 0..3 {
            let hi = houses[ht];
            self.house_masks[ht][hi] |= mask;
            let mask = 1 << houses[ht ^ 1];
            self.val_house_pos_indices[0][ht][hi] |= mask;
            for si in 0..9 {
                let local_idx = REV_LOOKUP[ht][hi][si];
                let local_houses = LOOKUP[local_idx];
                for lht in 0..3 {
                    let lhi = local_houses[lht];
                    let mask = 1 << local_houses[lht ^ 1];
                    self.val_house_pos_indices[val][lht][lhi] |= mask;
                }
            }
        }
    }

    fn update_showbestfree(&mut self) {
        self.showbestfree = BestFree::NONE;

        for i in 0..81 {
            if self.board[i] != 0 {
                continue;
            }

            let c = self.candidates(i);
            let weight = c.count_zeros() as u8;
            let value = BestFree::SOME {
                weight,
                value: BestFreeVal::INDEX(i),
            };

            match self.showbestfree {
                BestFree::SOME {
                    weight: w,
                    value: _,
                } => {
                    if weight < w {
                        self.showbestfree = value
                    }
                    if weight < 2 {
                        return;
                    }
                }
                _ => self.showbestfree = value,
            }
        }

        for i in 1..10 {
            let val_mask = 1 << (i - 1);
            for j in 0..3 {
                for k in 0..9 {
                    if self.house_masks[j][k] & val_mask != 0 {
                        continue;
                    }

                    let c = self.pos_indices(i, j, k);
                    let weight = c.count_zeros() as u8;
                    let value = BestFree::SOME {
                        weight,
                        value: BestFreeVal::VALHOUSE([i, j, k]),
                    };

                    match self.showbestfree {
                        BestFree::SOME {
                            weight: w,
                            value: _,
                        } => {
                            if weight < w {
                                self.showbestfree = value
                            }
                            if weight < 2 {
                                return;
                            }
                        }
                        _ => self.showbestfree = value,
                    }
                }
            }
        }
    }

    pub fn showbestfree(&self) -> ShowKinds {
        match self.showbestfree {
            BestFree::NONE => ShowKinds::SOLVED,
            BestFree::SOME { weight: _, value } => match value {
                BestFreeVal::INDEX(i) => ShowKinds::PICKIDX(i, self.candidates(i)),
                BestFreeVal::VALHOUSE([i, j, k]) => {
                    ShowKinds::PICKVAL([i, j, k], self.pos_indices(i, j, k))
                }
            },
        }
    }

    fn candidates(&self, idx: usize) -> u16 {
        let [i, j, k, _] = LOOKUP[idx];
        self.house_masks[0][i] | self.house_masks[1][j] | self.house_masks[2][k]
    }

    fn pos_indices(&self, val: usize, ht: usize, hi: usize) -> u16 {
        self.val_house_pos_indices[0][ht][hi] | self.val_house_pos_indices[val][ht][hi]
    }

}

#[rustfmt::skip]
impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: [_; 81] = {
            let chars: Vec<_> = s
                .chars()
                .map(|c| match c as u8 {
                    46 => Ok(0),
                    x if 47 < x && x < 58 => Ok(x - 48),
                    _ => Err(ParseGameError::UnknownCharacter(c)),
                })
                .collect::<Result<Vec<_>, _>>()?;
            match chars.try_into() {
                Err(_) => return Err(ParseGameError::IncorrectLength),
                Ok(x) => x,
            }
        };

        Ok(Game::init_board(board))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self
            .board
            .map(|x| match x {
                0 => '.',
                x => char::from_digit(x as u32, 10).unwrap(),
            })
            .iter()
            .collect();

        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy)]
pub enum BestFreeVal {
    INDEX(usize),
    VALHOUSE([usize; 3]),
}

#[derive(Clone, Copy)]
pub enum BestFree {
    NONE,
    SOME { weight: u8, value: BestFreeVal },
}

pub enum ShowKinds {
    PICKIDX(usize, u16),
    PICKVAL([usize; 3], u16),
    SOLVED,
    FAILED,
}

#[derive(Debug)]
pub enum ParseGameError {
    IncorrectLength,
    UnknownCharacter(char),
}

#[derive(Clone)]
pub struct Game {
    board: [u8; 81],
    house_masks: [[u16; 9]; 3],
    val_house_pos_indices: [[[u16; 9]; 3]; 10],
    showbestfree: BestFree,
}
