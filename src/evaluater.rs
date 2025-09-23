use crate::game::{Game, ShowKinds};

pub const fn eval(game: &mut Game) -> Result<(), ()> {
    match game.showbestfree() {
        ShowKinds::SOLVED => return Ok(()),
        ShowKinds::FAILED => return Err(()),
        ShowKinds::PICKIDX(idx, mut candidates) => {
            while candidates != 0 {
                let c = candidates.trailing_zeros() as usize;
                game.unsafe_choose(idx, c);
                if eval(game).is_ok() {
                    return Ok(());
                }
                game.unsafe_unchoose(idx);
                candidates &= candidates - 1;
            }
        }
        ShowKinds::PICKVAL(vti, mut candidates) => {
            while candidates != 0 {
                let c = candidates.trailing_zeros() as usize;
                let idx = game.unsafe_choose_alt(vti, c);
                if eval(game).is_ok() {
                    return Ok(());
                }
                game.unsafe_unchoose(idx);
                candidates &= candidates - 1;
            }
        }
    }
    Err(())
}
