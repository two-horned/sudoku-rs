/* TODO:
 * - Remove all weight vectors of set indices and filled houses
 * - Update all weight vectors of all regional indices for a set index-value pair
 * - Update all weight vectors of all houses that have less possible indices for a value
 * - Add smarter way to initialise puzzle
 */

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
    pub fn unsafe_choose_alt(&mut self, vht: [usize; 3], idx: usize) {
        let [val, ht, hi] = vht;
        self.unsafe_choose(REV_LOOKUP[ht][hi][idx], val + 1);
    }

    pub fn unsafe_choose(&mut self, idx: usize, val: usize) {
        self.last_placed = idx;
        self.board[idx] = val;
        self.update_weight_vectors_and_masks(idx, val);
    }

    fn update_weight_vectors_and_masks(&mut self, idx: usize, val: usize) {
        let val = val - 1;
        let mask_idx = 1 << val;

        for (house_type, house_index) in (0..3).zip(LOOKUP[idx]) {
            for subindex in 0..9 {
                let regional_index = REV_LOOKUP[house_type][house_index][subindex];
                let num = self.candidates(regional_index);
                let mask_val = 1 << subindex;

                if regional_index == idx || num & mask_idx != 0 {
                    self.val_house_pos_indices[val][house_type][house_index] |= mask_val;
                    continue;
                }

                let w = weight(num);
                let wv = &mut self.weight_idx_vectors;
                for i in 0..wv[w].len() {
                    if wv[w][i] == regional_index {
                        swap_with_last_and_delete(&mut wv[w], i);
                        wv[w - 1].push(regional_index);
                        break;
                    }
                }

                let num = self.val_house_pos_indices[val][i][j];
                let w = weight(num);
                let wv = &mut self.weight_val_house_vectors;
                for k in 0..wv[w].len() {
                    if wv[w][k] == [val, i, j] {
                        swap_with_last_and_delete(&mut wv[w], k);
                        if self.house_masks[i][j] == 0x1ff {
                            break;
                        }
                        wv[w - 1].push([val, i, j]);
                        break;
                    }
                }
                self.val_house_pos_indices[val][i][j] |= mask_val;
            }
            self.house_masks[i][j] |= mask_idx;
        }
    }

    fn showbestfree_idx(&mut self, w: usize) -> ShowKinds {
        let wv = &mut self.weight_idx_vectors[w];
        let min_idx = wv[0];
        swap_with_last_and_delete(wv, 0);
        if w == 1 {
            ShowKinds::PICKIDXNC(min_idx, self.candidates(min_idx))
        } else {
            ShowKinds::PICKIDX(min_idx, self.candidates(min_idx))
        }
    }

    fn showbestfree_val(&mut self, w: usize) -> ShowKinds {
        let wv = &mut self.weight_val_house_vectors[w];
        let [i, j, k] = wv.pop().unwrap();
        let idc = self.val_house_pos_indices[i][j][k];
        if w == 1 {
            ShowKinds::PICKVALNC([i, j, k], idc)
        } else {
            ShowKinds::PICKVAL([i, j, k], idc)
        }
    }

    pub fn showbestfree(&mut self) -> ShowKinds {
        let len1 = self.weight_idx_vectors[0].len();
        let len2 = self.weight_val_house_vectors[0].len();
        match len1 + len2 {
            0 => (),
            _ => return ShowKinds::FAILED,
        }

        for i in 1..10 {
            let len1 = self.weight_idx_vectors[i].len();
            let len2 = self.weight_val_house_vectors[i].len();
            match (len1, len2) {
                (0, 0) => continue,
                (0, _) => return self.showbestfree_val(i),
                _ => return self.showbestfree_idx(i),
            }
        }
        ShowKinds::SOLVED
    }

    fn candidates(&self, idx: usize) -> u16 {
        let [i, j, k, _] = LOOKUP[idx];
        self.house_masks[0][i] | self.house_masks[1][j] | self.house_masks[2][k]
    }
}

fn swap_with_last_and_delete<T>(v: &mut Vec<T>, idx: usize)
where
    T: Copy,
{
    v[idx] = v[v.len() - 1];
    v.pop();
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

#[rustfmt::skip]
impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: [_; 81] = {
            let chars: Vec<_> = s
                .chars()
                .map(|c| match c as usize {
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

        let (house_masks, val_house_pos_indices) = ([[0; 9]; 3], [[[0; 9]; 3]; 9]);

        let weight_idx_vectors = [
            Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            (0..81).filter(|&i| board[i] == 0).collect()
        ];

        let mut v = Vec::with_capacity(243);
        for i in 0..9 {
            for j in 0..3 {
                for k in 0..9 {
                    v.push([i, j, k]);
                }
            }
        }

        let weight_val_house_vectors = [
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            Vec::with_capacity(242),
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            Vec::with_capacity(243),
            v
        ];

        let mut tmp = Self {
            board,
            house_masks,
            val_house_pos_indices,
            weight_idx_vectors,
            weight_val_house_vectors,
            last_placed: 40,
        };

        for i in 0..81 {
            if board[i] == 0 {
                continue;
            }
            tmp.unsafe_choose(i, board[i]);
        }

        Ok(tmp)
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

pub enum ShowKinds {
    PICKIDXNC(usize, u16),
    PICKIDX(usize, u16),
    PICKVALNC([usize; 3], u16),
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
    board: [usize; 81],
    house_masks: [[u16; 9]; 3],
    val_house_pos_indices: [[[u16; 9]; 3]; 9],
    weight_idx_vectors: [Vec<usize>; 10],
    weight_val_house_vectors: [Vec<[usize; 3]>; 10],
    last_placed: usize,
}
