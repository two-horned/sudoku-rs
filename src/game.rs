/* TODO:
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

static TEMPLATE_VAL_HOUSE: [[u8; 3]; 243] = {
    let mut tmp = [[0, 0, 0]; 243];
    let mut idx = 0;
    let mut i = 0;
    let mut j;
    let mut k;
    while i < 9 {
        j = 0;
        while j < 3 {
            k = 0;
            while k < 9 {
                tmp[idx] = [i, j, k];
                idx += 1;
                k += 1;
            }
            j += 1;
        }

        i += 1;
    }
    tmp
};

static TEMPLATE_QUICK_VAL_HOUSE: [[[u8; 9]; 3]; 9] = {
    let mut tmp = [[[0; 9]; 3]; 9];
    let mut idx = 0;
    let mut i = 0;
    let mut j;
    let mut k;
    while i < 9 {
        j = 0;
        while j < 3 {
            k = 0;
            while k < 9 {
                tmp[i][j][k] = idx;
                idx += 1;
                k += 1;
            }
            j += 1;
        }

        i += 1;
    }
    tmp
};

impl Game {
    pub fn unsafe_choose_alt(&mut self, vht: [usize; 3], idx: usize) {
        let [val, ht, hi] = vht;
        self.unsafe_choose(REV_LOOKUP[ht][hi][idx], val + 1);
    }

    pub fn unsafe_choose(&mut self, idx: usize, val: usize) {
        self.update_weight_vectors_and_masks(idx, val);
        self.board[idx] = val;
        self.update_candidates(idx, val);
        self.update_val_house_pos(idx, val);
    }

    fn update_candidates(&mut self, idx: usize, val: usize) {
        let mask = 1 << (val - 1);
        let rcsi = LOOKUP[idx];
        for ht in 0..3 {
            let hi = rcsi[ht];
            self.house_masks[ht][hi] |= mask;
        }
    }

    fn update_val_house_pos(&mut self, idx: usize, val: usize) {
        let rcsi = LOOKUP[idx];
        let val = val - 1;

        for ht in 0..3 {
            let hi = rcsi[ht];
            let mask = 1 << rcsi[ht ^ 1];
            for v in 0..9 {
                let num = self.val_house_pos_indices[v][ht][hi];
                if num & mask == 0 {
                    self.val_house_pos_indices[v][ht][hi] |= mask;
                    let w = weight(num);
                    self.find_and_delete_val_house(w, [v, ht, hi]);
                    if v != val {
                        self.add_val_house(w - 1, [v, ht, hi]);
                    }
                }
            }
        }
    }

    fn update_weight_vectors_and_masks(&mut self, idx: usize, val: usize) {
        let val = val - 1;
        let mask = 1 << val;
        let rcsi = LOOKUP[idx];
        for ht in 0..3 {
            let hi = rcsi[ht];
            for si in 0..9 {
                let local_idx = REV_LOOKUP[ht][hi][si];
                let local_rcsi = LOOKUP[local_idx];
                let num = self.candidates(local_idx);

                if (self.board[local_idx] != 0)
                    || (num & mask != 0)
                    || (ht != 2 && local_rcsi[2] == rcsi[2])
                {
                    continue;
                }
                let w = weight(num);

                self.find_and_delete_idx(w, local_idx);

                if local_idx == idx {
                    continue;
                }

                self.add_idx(w - 1, local_idx);

                for ht in 0..3 {
                    let hi = local_rcsi[ht];
                    let mask = 1 << local_rcsi[ht ^ 1];
                    let num = self.val_house_pos_indices[val][ht][hi];
                    let w = weight(num);

                    self.find_and_delete_val_house(w, [val, ht, hi]);
                    self.add_val_house(w - 1, [val, ht, hi]);
                    self.val_house_pos_indices[val][ht][hi] |= mask;
                }
            }
        }
    }

    fn showbestfree_idx(&self, w: usize) -> ShowKinds {
        let min_idx = self.weight_idx_vectors[w][0];
        let cands = self.candidates(min_idx as usize);
        if w == 1 {
            ShowKinds::PICKIDXNC(min_idx as usize, cands)
        } else {
            ShowKinds::PICKIDX(min_idx as usize, cands)
        }
    }

    fn showbestfree_val(&self, w: usize) -> ShowKinds {
        let [i, j, k] = self.weight_val_house_vectors[w][0].map(|x| x as usize);
        let idc = self.val_house_pos_indices[i][j][k];
        if w == 1 {
            ShowKinds::PICKVALNC([i, j, k], idc)
        } else {
            ShowKinds::PICKVAL([i, j, k], idc)
        }
    }

    pub fn showbestfree(&self) -> ShowKinds {
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

    fn add_idx(&mut self, weight: usize, idx: usize) {
        let v = &mut self.weight_idx_vectors[weight];
        self.quick_idx[idx] = v.len() as u8;
        v.push(idx as u8);
    }

    fn add_val_house(&mut self, weight: usize, val_house: [usize; 3]) {
        let v = &mut self.weight_val_house_vectors[weight];
        let [i, j, k] = val_house;
        self.quick_val_house[i][j][k] = v.len() as u8;
        v.push(val_house.map(|x| x as u8));
    }

    fn find_and_delete_idx(&mut self, weight: usize, idx: usize) {
        let v = &mut self.weight_idx_vectors[weight];
        let i = self.quick_idx[idx];
        let l = v[v.len() - 1];
        self.quick_idx[l as usize] = i;
        v[i as usize] = l;
        v.pop();
    }

    fn find_and_delete_val_house(&mut self, weight: usize, val_house: [usize; 3]) {
        let v = &mut self.weight_val_house_vectors[weight];
        let [i, j, k] = val_house;
        let i = self.quick_val_house[i][j][k];
        let [l1, l2, l3] = v[v.len() - 1];
        self.quick_val_house[l1 as usize][l2 as usize][l3 as usize] = i;
        v[i as usize] = [l1, l2, l3];
        v.pop();
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

#[rustfmt::skip]
impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: [_; 81] = {
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

        let weight_idx_vectors =
            [ Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            Vec::with_capacity(81), Vec::with_capacity(81), Vec::with_capacity(81),
            (0..81).collect() ];

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
            TEMPLATE_VAL_HOUSE.into_iter().collect()
        ];

        let mut tmp = Self {
            board: [0; 81],
            house_masks: [[0; 9]; 3],
            val_house_pos_indices: [[[0; 9]; 3]; 9],
            weight_idx_vectors,
            weight_val_house_vectors,
            quick_idx: std::array::from_fn(|i| i as u8),
            quick_val_house: TEMPLATE_QUICK_VAL_HOUSE,
        };

        for i in 0..81 {
            if choices[i] == 0 {
                continue;
            }
            tmp.unsafe_choose(i, choices[i] as usize);
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
    weight_idx_vectors: [Vec<u8>; 10],
    weight_val_house_vectors: [Vec<[u8; 3]>; 10],
    quick_idx: [u8; 81],
    quick_val_house: [[[u8; 9]; 3]; 9],
}
