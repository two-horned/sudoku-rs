/* TODO:
 * - Cleanup
 * - Add way to prune away already seen patterns
 *   - Storing subsets (maybe use bloom filter?)
 */

use crate::game::{Game, ShowKinds};

pub fn eval(mut game: Game) -> Result<Game, ()> {
    match game.showbestfree() {
        ShowKinds::FAILED => Err(()),
        ShowKinds::SOLVED => Ok(game),
        ShowKinds::PICKIDX(idx, mut candidates) => {
            let mut g;

            for x in 1..10 {
                if candidates & 1 == 0 {
                    g = game.clone();
                    g.unsafe_choose(idx, x);
                    match eval(g) {
                        Ok(k) => return Ok(k),
                        _ => (),
                    }
                }
                candidates >>= 1;
            }
            Err(())
        }
        ShowKinds::PICKIDXNC(idx, mut candidates) => {
            for x in 1..10 {
                if candidates & 1 == 0 {
                    game.unsafe_choose(idx, x);
                    match eval(game) {
                        Ok(k) => return Ok(k),
                        _ => return Err(()),
                    }
                }
                candidates >>= 1;
            }
            unreachable!()
        }
        ShowKinds::PICKVAL(vht, mut candidates) => {
            let mut g;

            for x in 0..9 {
                if candidates & 1 == 0 {
                    g = game.clone();
                    g.unsafe_choose_alt(vht, x);
                    match eval(g) {
                        Ok(k) => return Ok(k),
                        _ => (),
                    }
                }
                candidates >>= 1;
            }
            Err(())
        }
        ShowKinds::PICKVALNC(vht, mut candidates) => {
            for x in 0..9 {
                if candidates & 1 == 0 {
                    game.unsafe_choose_alt(vht, x);
                    match eval(game) {
                        Ok(k) => return Ok(k),
                        _ => return Err(()),
                    }
                }
                candidates >>= 1;
            }
            unreachable!()
        }
    }
}
