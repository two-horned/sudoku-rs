use std::fmt;

impl Game {
    pub fn board(&self) -> [u8; 81] {
        self.board.clone()
    }

    pub fn choose(&mut self, idx: usize, val: u8) -> bool {
        if self.board[idx] != 0 {
            return false;
        }

        self.board[idx] = val;
        let (row, col, sqr) = get_mask_indices(idx);
        let num = !(1 << self.board[idx] - 1);
        self.row_masks[row] &= num;
        self.col_masks[col] &= num;
        self.sqr_masks[sqr] &= num;

        true
    }

    pub fn showbestfree(&self) -> Option<(usize, Vec<u8>)> {
        let mut min_len = 10;
        let mut idx_vec = None;

        for i in 0..81 {
            if self.board[i] != 0 {
                continue;
            }

            let mut u = vec![];

            let (row, col, sqr) = get_mask_indices(i);
            let mut num = self.row_masks[row] & self.col_masks[col] & self.sqr_masks[sqr];

            let mut j = 1;
            while 0 < num {
                if num & 1 != 0 {
                    u.push(j);
                }
                num >>= 1;
                j += 1;
            }
            if u.len() < min_len {
                min_len = u.len();
                idx_vec = Some((i, u));
            }
        }
        return idx_vec;
    }
}

fn get_mask_indices(idx: usize) -> (usize, usize, usize) {
    let row = idx / 9;
    let col = idx % 9;
    let sqr_row = row / 3;
    let sqr_col = col / 3;

    (row, col, sqr_row * 3 + sqr_col)
}

fn build_masks(board: &[u8; 81]) -> ([u16; 9], [u16; 9], [u16; 9]) {
    let mut row_masks = [0xFE00; 9];
    let mut col_masks = [0xFE00; 9];
    let mut sqr_masks = [0xFE00; 9];

    for i in 0..81 {
        if board[i] == 0 {
            continue;
        }
        let (row, col, sqr) = get_mask_indices(i);
        let num = 1 << board[i] - 1;
        row_masks[row] |= num;
        col_masks[col] |= num;
        sqr_masks[sqr] |= num;
    }

    (
        row_masks.map(|i| !i),
        col_masks.map(|i| !i),
        sqr_masks.map(|i| !i),
    )
}

impl From<[u8; 81]> for Game {
    fn from(board: [u8; 81]) -> Self {
        let (row_masks, col_masks, sqr_masks) = build_masks(&board);
        Self {
            board,
            row_masks,
            col_masks,
            sqr_masks,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Game {
    board: [u8; 81],
    row_masks: [u16; 9],
    col_masks: [u16; 9],
    sqr_masks: [u16; 9],
}
