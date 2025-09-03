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
    let mut tmp = [0x1FF; 8];
    let mut i = 0;
    while i < 8 {
        let mut f: usize = i;
        while f != 0 {
            let c = f.trailing_zeros();
            f &= f - 1;
            tmp[7 - i] ^= 0b111 << 3 * c;
        }
        i += 1;
    }
    tmp
};

const YAR_MAKER: [u16; 8] = {
    let mut tmp = [0x1FF; 8];
    let mut i: u16 = 8;
    while i < 8 {
        tmp[7 - i as usize] ^= i ^ (i << 3) ^ (i << 6);
        i += 1;
    }
    tmp
};

const MINI_LOOKUP: [[usize; 2]; 9] = {
    let mut tmp = [[0; 2]; 9];
    let mut i = 0;
    while i < 9 {
        tmp[i] = [i / 3, i % 3];
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
    const fn init_board(board: [u8; 81]) -> Self {
        let mut tmp = Self {
            board,
            frees: [0x1FFFFFFFFFFFFFFFFFFFF; 4],
            house_masks: [[0x1FF; 9]; 3],
            occupied: [[0x1FF; 9]; 3],
            value_masks: [[0x1FF; 3]; 9],
        };

        let mut i = 0;
        while i < 81 {
            let val = board[i];
            if val != 0 {
                tmp.update_masks(i, val as usize - 1);
            }
            i += 1;
        }

        tmp
    }

    pub const fn unsafe_choose_alt(&mut self, vht: [usize; 2], idx: usize) -> usize {
        let [ht, id] = vht;
        let [hi, val, _, _] = LOOKUP[id];
        let true_idx = REV_LOOKUP[ht][hi][idx];
        self.unsafe_choose(true_idx, val);
        return true_idx;
    }

    pub const fn unsafe_choose(&mut self, idx: usize, val: usize) {
        self.board[idx] = 1 + val as u8;
        self.update_masks(idx, val);
    }

    pub const fn unsafe_unchoose(&mut self, idx: usize) {
        let val = self.board[idx] - 1;
        self.board[idx] = 0;
        self.update_masks(idx, val as usize);
    }

    const fn update_masks(&mut self, idx: usize, val: usize) {
        self.frees[3] ^= 1 << idx;
        let mask = 1 << val;
        let houses = LOOKUP[idx];
        let mut ht = 0;
        while ht < 3 {
            let hi = houses[ht];
            self.frees[ht] ^= 1 << hi * 9 + val;
            self.house_masks[ht][hi] ^= mask;
            let mask = 1 << houses[ht ^ 1];
            self.occupied[ht][hi] ^= mask;
            self.value_masks[val][ht] ^= 1 << hi;
            ht += 1;
        }
    }

    pub const fn showbestfree(&self) -> ShowKinds {
        let mut best_value = ShowKinds::SOLVED;
        let mut best_weight = 10;

        let mut f = self.frees[3];
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

        let mut t = 0;
        while t < 3 {
            let mut f = self.frees[t];
            while f != 0 {
                let i = f.trailing_zeros() as usize;
                f &= f - 1;
                let c = self.pos_indices(t, i);
                match c.count_ones() {
                    0 => return ShowKinds::FAILED,
                    1 => return ShowKinds::PICKVAL([t, i], c),
                    w if w < best_weight => {
                        best_weight = w;
                        best_value = ShowKinds::PICKVAL([t, i], c);
                    }
                    _ => (),
                }
            }
            t += 1;
        }

        best_value
    }

    const fn candidates(&self, idx: usize) -> u16 {
        let [i, j, k, _] = LOOKUP[idx];
        self.house_masks[0][i] & self.house_masks[1][j] & self.house_masks[2][k]
    }

    #[inline(always)]
    const fn pos_indices(&self, ht: usize, id: usize) -> u16 {
        let [hi, val, _, _] = LOOKUP[id];
        let [rwhi, clhi] = MINI_LOOKUP[hi];
        self.occupied[ht][hi]
            & match ht {
                0 => self.value_masks[val][1] & get_ray_r(self.value_masks[val][2], rwhi),
                1 => self.value_masks[val][0] & get_ray_c(self.value_masks[val][2], rwhi),
                _ => {
                    get_ray_r(self.value_masks[val][0], rwhi)
                        & get_yar_r(self.value_masks[val][1], clhi)
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
                    _ => Err(ParseGameError::IllegalCharacter(c)),
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
        let buffer: [u8; 81] = self.board.map(|x| match x {
            0 => 46,
            x => x + 48,
        });
        f.write_str(unsafe { str::from_utf8_unchecked(&buffer) })
    }
}

// impl fmt::Display for ParseGameError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(match self {
//             ParseGameError::IncorrectLength => "Incorrect length",
//             ParseGameError::IllegalCharacter(x) => &format!("Character {} is illegal", x),
//         })
//     }
// }

// impl fmt::Debug for Game {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let string = (0..9)
//             .map(|i| {
//                 (0..9)
//                     .map(|j| match self.board[i * 9 + j] {
//                         0 => '.',
//                         x => char::from_digit(x as u32, 10).unwrap(),
//                     })
//                     .collect::<String>()
//                     + "\n"
//             })
//             .collect::<String>();
//         f.write_str(&string)
//     }
// }

#[derive(Clone, Copy)]
pub enum ShowKinds {
    PICKIDX(usize, u16),
    PICKVAL([usize; 2], u16),
    SOLVED,
    FAILED,
}

#[derive(Debug)]
pub enum ParseGameError {
    IncorrectLength,
    IllegalCharacter(char),
}

pub struct Game {
    board: [u8; 81],
    frees: [u128; 4],
    house_masks: [[u16; 9]; 3],
    occupied: [[u16; 9]; 3],
    value_masks: [[u16; 3]; 9],
}
