use crate::game::{Game, ShowKinds};
use std::mem::MaybeUninit;

impl Evaluater {
    pub const fn new() -> Self {
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub const fn eval(&mut self, mut game: Game) -> Result<Game, ()> {
        let mut level = 0;
        self.buffer[0] = (
            unsafe { MaybeUninit::uninit().assume_init() },
            game.showbestfree(),
        );
        loop {
            match self.buffer[level].1 {
                ShowKinds::SOLVED => return Ok(game),
                ShowKinds::FAILED => {
                    if level == 0 {
                        return Err(());
                    } else {
                        let idx = self.buffer[level].0;
                        game.unsafe_unchoose(idx as usize);
                        level -= 1;
                        continue;
                    }
                }
                ShowKinds::PICKIDX(idx, mut candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    game.unsafe_choose(idx, c);
                    candidates &= candidates - 1;
                    if candidates == 0 {
                        self.buffer[level].1 = ShowKinds::FAILED;
                    } else {
                        self.buffer[level].1 = ShowKinds::PICKIDX(idx, candidates);
                    }
                    level += 1;
                    self.buffer[level] = (idx as u8, game.showbestfree());
                }
                ShowKinds::PICKVAL(vti, mut candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    let idx = game.unsafe_choose_alt(vti, c);
                    candidates &= candidates - 1;
                    if candidates == 0 {
                        self.buffer[level].1 = ShowKinds::FAILED;
                    } else {
                        self.buffer[level].1 = ShowKinds::PICKVAL(vti, candidates);
                    }
                    level += 1;
                    self.buffer[level] = (idx as u8, game.showbestfree());
                }
            }
        }
    }
}

pub struct Evaluater {
    buffer: [(u8, ShowKinds); 81],
}
