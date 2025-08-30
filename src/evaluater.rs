use crate::game::{Game, ShowKinds};
use std::mem::MaybeUninit;

impl Evaluater {
    pub fn new() -> Self {
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn eval(&mut self, game: Game) -> Result<Game, ()> {
        let mut level = 0;
        self.buffer[0] = (game, game.showbestfree());
        loop {
            match &mut self.buffer[level].1 {
                ShowKinds::SOLVED => return Ok(self.buffer[level].0),
                ShowKinds::FAILED => {
                    if level == 0 {
                        return Err(());
                    } else {
                        level -= 1;
                        continue;
                    }
                }
                ShowKinds::PICKIDX(idx, candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    debug_assert!(c < 10, "Candidates are {:b}", candidates);
                    let mut game = self.buffer[level].0;
                    game.unsafe_choose(*idx, 1 + c);
                    *candidates &= *candidates - 1;
                    if *candidates != 0 {
                        level += 1;
                    }
                    self.buffer[level] = (game, game.showbestfree());
                }
                ShowKinds::PICKVAL(vhthi, candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    debug_assert!(c < 10, "Candidates are {:b}", candidates);
                    let mut game = self.buffer[level].0;
                    game.unsafe_choose_alt(*vhthi, c);
                    *candidates &= *candidates - 1;
                    if *candidates != 0 {
                        level += 1;
                    }
                    self.buffer[level] = (game, game.showbestfree());
                }
            }
        }
    }
}

pub struct Evaluater {
    buffer: [(Game, ShowKinds); 81],
}
