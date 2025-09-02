use std::{fmt, str::FromStr};

const LOOKUP: [[usize; 4]; 81] = {
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

const REV_LOOKUP: [[[usize; 9]; 9]; 3] = {
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

const RAY_MAKER: [u16; 8] = {
    let mut tmp = [0; 8];
    let mut i = 0;
    while i < 8 {
        let mut f: usize = i;
        while f != 0 {
            let c = f.trailing_zeros();
            f &= f - 1;
            tmp[i] ^= 0b111 << 3 * c;
        }
        i += 1;
    }
    tmp
};

const YAR_MAKER: [u16; 8] = {
    let mut tmp = [0; 8];
    let mut i: u16 = 0;
    while i < 8 {
        tmp[i as usize] = i ^ (i << 3) ^ (i << 6);
        i += 1;
    }
    tmp
};

const fn get_ray_r(mask: u16, i: usize) -> u16 {
    let j = 0b111 & mask >> 3 * i;
    RAY_MAKER[j as usize]
}

const fn get_ray_c(mask: u16, i: usize) -> u16 {
    let j = 0b111 & 0b10101 * (0b100100100 & mask << 2 - i) >> 6;
    RAY_MAKER[j as usize]
}

const fn get_yar_r(mask: u16, i: usize) -> u16 {
    let j = 0b111 & mask >> 3 * i;
    YAR_MAKER[j as usize]
}

impl Game {
    fn init_board(board: [u8; 81]) -> Self {
        let mut tmp = Self {
            board,
            frees: 0x1FFFFFFFFFFFFFFFFFFFF,
            house_masks: [[0x1FF; 9]; 3],
            occupied: [[0xFE00; 9]; 3],
            value_masks: [[0xFE00; 3]; 9],
        };

        for i in 0..81 {
            let val = board[i];
            if val != 0 {
                tmp.frees ^= 1 << i;
                tmp.update_masks(i, val as usize);
            }
        }

        tmp
    }

    pub fn unsafe_choose_alt(&mut self, vht: [usize; 3], idx: usize) -> usize {
        let [val, ht, hi] = vht;
        let true_idx = REV_LOOKUP[ht][hi][idx];
        self.unsafe_choose(true_idx, val);
        return true_idx;
    }

    pub fn unsafe_choose(&mut self, idx: usize, val: usize) {
        debug_assert!(self.board[idx] == 0, "choice is idx {idx} with val {val}, game is\n{:?}", self);
        self.board[idx] = val as u8;
        self.frees ^= 1 << idx;
        self.update_masks(idx, val);
    }

    pub fn unsafe_unchoose(&mut self, idx: usize) {
        let val = self.board[idx];
        debug_assert!(val != 0);
        self.board[idx] = 0;
        self.frees ^= 1 << idx;
        self.update_masks(idx, val as usize);
    }

    fn update_masks(&mut self, idx: usize, val: usize) {
        let mask = 1 << val - 1;
        let houses = LOOKUP[idx];
        for ht in 0..3 {
            let hi = houses[ht];
            self.house_masks[ht][hi] ^= mask;
            let mask = 1 << houses[ht ^ 1];
            self.occupied[ht][hi] ^= mask;
            self.value_masks[val - 1][ht] ^= 1 << hi;
        }
    }

    pub fn showbestfree(&self) -> ShowKinds {
        let mut best_value = ShowKinds::SOLVED;
        let mut best_weight = 10;

        let mut f = self.frees;

        while f != 0 {
            let i = f.trailing_zeros() as usize;
            f &= f - 1;

            let c = self.candidates(i);

            match c.count_ones() {
                0 => return ShowKinds::FAILED,
                1 => return ShowKinds::PICKIDX(i, c),
                w if w < best_weight => {
                    best_weight = w;
                    best_value = ShowKinds::PICKIDX(i, c);
                }
                _ => (),
            }
        }

        for j in 0..3 {
            for k in 0..9 {
                let mut f = self.house_masks[j][k];
                while f != 0 {
                    let i = 1 + f.trailing_zeros() as usize;
                    f &= f - 1;

                    let c = !self.pos_indices(i, j, k);

                    match c.count_ones() {
                        0 => return ShowKinds::FAILED,
                        1 => return ShowKinds::PICKVAL([i, j, k], c),
                        w if w < best_weight => {
                            best_weight = w;
                            best_value = ShowKinds::PICKVAL([i, j, k], c);
                        }
                        _ => (),
                    }
                }
            }
        }

        best_value
    }

    fn candidates(&self, idx: usize) -> u16 {
        let [i, j, k, _] = LOOKUP[idx];
        self.house_masks[0][i] & self.house_masks[1][j] & self.house_masks[2][k]
    }

    fn pos_indices(&self, val: usize, ht: usize, hi: usize) -> u16 {
        let val = val - 1;
        self.occupied[ht][hi]
            | match ht {
                0 => self.value_masks[val][1] | get_ray_r(self.value_masks[val][2], hi / 3),
                1 => self.value_masks[val][0] | get_ray_c(self.value_masks[val][2], hi / 3),
                _ => {
                    get_ray_r(self.value_masks[val][0], hi / 3)
                        | get_yar_r(self.value_masks[val][1], hi % 3)
                }
            }
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: [_; 81] = {
            match s
                .chars()
                .map(|c| match c as u8 {
                    46 => Ok(0),
                    x @ 49..58 => Ok(x - 48),
                    _ => Err(ParseGameError::UnknownCharacter(c)),
                })
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
            {
                Err(_) => return Err(ParseGameError::IncorrectLength),
                Ok(x) => x,
            }
        };

        Ok(Game::init_board(board))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .board
                .iter()
                .map(|&x| match x {
                    0 => '.',
                    x => char::from_digit(x as u32, 10).unwrap(),
                })
                .collect::<String>(),
        )
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = (0..9).map(|i| (0..9).map(|j| 
                 match self.board[i * 9 + j] {
                    0 => '.',
                    x => char::from_digit(x as u32, 10).unwrap(),
                }).collect::<String>() + "\n").collect::<String>();
        f.write_str(&string)
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Game {
    board: [u8; 81],
    frees: u128,
    house_masks: [[u16; 9]; 3],
    occupied: [[u16; 9]; 3],
    value_masks: [[u16; 3]; 9],
}
