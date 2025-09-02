use crate::game::{Game, ShowKinds};
use std::mem::MaybeUninit;

impl Evaluater {
    pub fn new() -> Self {
        Self {
            genbuf: unsafe { MaybeUninit::uninit().assume_init() },
            retbuf: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn eval(&mut self, mut game: Game) -> Result<Game, ()> {
        let mut glev = 0;
        let mut rlev = 0;
        self.genbuf[0] = game.showbestfree();
        loop {
            println!("Level {rlev}");
            match &mut self.genbuf[glev] {
                ShowKinds::SOLVED => return Ok(game),
                ShowKinds::FAILED => {
                    if glev == 0 {
                        return Err(());
                    } else {
                        glev -= 1;
                        rlev -= 1;
                        let idx = self.retbuf[rlev];
                        println!("Removing {idx} ({}, {})", idx / 9, idx % 9);
                        println!("Before\n{:?}", game);
                        game.unsafe_unchoose(self.retbuf[rlev]);
                        println!("After\n{:?}", game);
                        continue;
                    }
                }
                ShowKinds::PICKIDX(idx, candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    debug_assert!(c < 10, "Candidates are {:b}", candidates);
                    game.unsafe_choose(*idx, 1 + c);
                    *candidates &= *candidates - 1;
                    if *candidates != 0 {
                        glev += 1;
                    }

                    self.retbuf[rlev] = *idx;
                    self.genbuf[glev] = game.showbestfree();
                    rlev += 1;
                }
                ShowKinds::PICKVAL(vhthi, candidates) => {
                    let c = candidates.trailing_zeros() as usize;
                    debug_assert!(c < 10, "Candidates are {:b}", candidates);
                    let idx = game.unsafe_choose_alt(*vhthi, c);
                    *candidates &= *candidates - 1;
                    if *candidates != 0 {
                        glev += 1;
                    }

                    self.retbuf[rlev] = idx;
                    self.genbuf[glev] = game.showbestfree();
                    rlev += 1;
                }
            }
        }
    }
}

pub struct Evaluater {
    genbuf: [ShowKinds; 81],
    retbuf: [usize; 81],
}
