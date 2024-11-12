/* TODO:
 * - Cleanup
 * - Add way to prune away already seen patterns
 *   - Storing subsets (maybe use bloom filter?)
 */

use crate::game::{Game, ShowKinds};

pub fn eval(game: Game) -> Result<Game, ()> {
    //std::thread::sleep(std::time::Duration::from_millis(500));
    //println!("Game {}", game);
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
        ShowKinds::PICKVAL(vhthi, mut candidates) => {
            let mut g;
            for x in 0..9 {
                if candidates & 1 == 0 {
                    g = game.clone();
                    g.unsafe_choose_alt(vhthi, x);
                    match eval(g) {
                        Ok(k) => return Ok(k),
                        _ => (),
                    }
                }
                candidates >>= 1;
            }
            Err(())
        }
    }
}
