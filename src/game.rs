use std::{fmt, str::FromStr};

static LOOKUP: [(usize, usize, usize); 81] = {
    let mut tmp = [(0, 0, 0); 81];
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
            tmp[idx] = (i, j, sqr);
            j += 1;
        }
        i += 1;
    }
    tmp
};

impl Game {
    pub fn unsafe_choose(&mut self, idx: usize, val: usize) {
        self.last_placed = idx;
        self.board[idx] = val;
        self.update_weight_idx_vectors_and_masks(idx, 1 << (val - 1));
        self.update_weight_val_vectors_and_masks(idx, val - 1);
    }

    pub fn unsafe_choose_house(&mut self, val_ht_house: (usize, usize, usize), six: usize) {
        let (val, ht, house) = val_ht_house;
        let idx = match ht {
            0 => house * 9 + six,
            1 => six * 9 + house,
            2 => house / 3 * 27 + house % 3 * 3 + six / 3 * 9 + six % 3,
            _ => 81,
        };
        self.unsafe_choose(idx, val);
    }

    fn update_weight_val_vectors_and_masks(&mut self, idx: usize, vali: usize) {
        let (row, col, sqr) = LOOKUP[idx];

        let vi = &mut self.val_house_pos_indices[vali];

        for (i, (j, n)) in [(row, col), (col, row), (sqr, row % 3 * 3 + col % 3)]
            .iter()
            .enumerate()
        {
            let num = vi[i][*j];
            let mask = 1 << *n;
            if num & mask == 0 {
                let w = weight(num);
                let wv = &mut self.weight_val_house_vectors;

                let mut updated = false;
                for k in 0..wv[w].len() {
                    if wv[w][k] == (vali, i, *j) {
                        swap_with_last_and_delete(&mut wv[w], k);
                        wv[w - 1].push((vali, i, *j));
                        updated = true;
                        break;
                    }
                }
                if !updated {
                    panic!(
                        "Val_House {:?} should be present in weight vector {}.",
                        (vali, i, *j),
                        w
                    );
                }

                vi[i][*j] |= mask;
            }
        }
    }

    fn decrease_idx_weight_if_needed(&mut self, idx: usize, mask: u16) {
        if self.board[idx] != 0 {
            return;
        }

        let num = self.candidates(idx);
        if num & mask != 0 {
            return;
        }

        let w = weight(num);
        let wv = &mut self.weight_idx_vectors;

        for i in 0..wv[w].len() {
            if wv[w][i] == idx {
                swap_with_last_and_delete(&mut wv[w], i);
                wv[w - 1].push(idx);
                return;
            }
        }
        panic!("Index {} should be present in weight vector {}.", idx, w);
    }

    fn update_weight_idx_vectors_and_masks(&mut self, idx: usize, mask: u16) {
        let (row, col, sqr) = LOOKUP[idx];

        for c in (row * 9)..((row + 1) * 9) {
            self.decrease_idx_weight_if_needed(c, mask);
        }

        for c in (col..81).step_by(9) {
            self.decrease_idx_weight_if_needed(c, mask);
        }

        self.house_masks[0][row] |= mask;
        self.house_masks[1][col] |= mask;

        let mut c;
        c = sqr / 3 * 27 + sqr % 3 * 3;
        for _ in 0..3 {
            for _ in 0..3 {
                self.decrease_idx_weight_if_needed(c, mask);
                c += 1;
            }
            c += 6
        }

        self.house_masks[2][sqr] |= mask;
    }

    pub fn showbestfree(&mut self) -> Option<(usize, u16, usize)> {
        let min_d = (2_f64 * 9_f64.powi(2)).sqrt();
        let (xx, yy, _) = LOOKUP[self.last_placed];
        for i in 0..10 {
            let len = self.weight_idx_vectors[i].len();
            if len == 0 {
                continue;
            }

            let mut min_j = 0;
            for j in 0..len {
                let idx = self.weight_idx_vectors[i][j];
                let (x, y, _) = LOOKUP[idx];
                if (((xx - x) as f64).abs().powi(2) + ((yy - y) as f64).abs().powi(2)).sqrt()
                    < min_d
                {
                    min_j = j;
                }
            }

            let min_idx = self.weight_idx_vectors[i][min_j];
            swap_with_last_and_delete(&mut self.weight_idx_vectors[i], min_j);

            return Some((min_idx, self.candidates(min_idx), i));
        }
        None
    }

    pub fn showbestfree_alt(&mut self) -> Option<((usize, usize, usize), u16, usize)> {
        for i in 0..10 {
            let len = self.weight_val_house_vectors[i].len();
            if len == 0 {
                continue;
            }

            let min_j = 0;
            let min_idx = self.weight_val_house_vectors[i][min_j];
            swap_with_last_and_delete(&mut self.weight_val_house_vectors[i], min_j);

            return Some((
                min_idx,
                self.val_house_pos_indices[min_idx.0][min_idx.1][min_idx.2],
                i,
            ));
        }
        None
    }

    fn candidates(&self, idx: usize) -> u16 {
        let (i, j, k) = LOOKUP[idx];
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

fn to_dig(c: char) -> Result<usize, ParseGameError> {
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
        let board: [_; 81] = {
            let chars: Vec<_> = s.chars().map(to_dig).collect::<Result<Vec<_>, _>>()?;
            match chars.try_into() {
                Err(_) => return Err(ParseGameError::IncorrectLength),
                Ok(x) => x,
            }
        };

        let (house_masks, val_house_pos_indices) = ([[0; 9]; 3], [[[0; 9]; 3]; 9]);

        let weight_idx_vectors = [
            Vec::with_capacity(3),
            Vec::with_capacity(9),
            Vec::with_capacity(18),
            Vec::with_capacity(27),
            Vec::with_capacity(36),
            Vec::with_capacity(45),
            Vec::with_capacity(54),
            Vec::with_capacity(63),
            Vec::with_capacity(72),
            (0..81).collect(),
        ];

        let mut v = vec![];
        for i in 0..9 {
            for j in 0..3 {
                for k in 0..9 {
                    v.push((i, j, k));
                }
            }
        }

        let weight_val_house_vectors = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            v,
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
    board: [usize; 81],
    house_masks: [[u16; 9]; 3],
    val_house_pos_indices: [[[u16; 9]; 3]; 9],
    weight_idx_vectors: [Vec<usize>; 10],
    weight_val_house_vectors: [Vec<(usize, usize, usize)>; 10],
    last_placed: usize,
}
