use crate::game::Game;

pub fn eval(mut game: Game) -> Result<Game, ()> {
    if let Some((i, mut n, w)) = game.showbestfree() {
        match w {
            0 => return Err(()),
            1 => {
                for x in 1..10 {
                    if n & 1 == 0 {
                        game.unsafe_choose(i, x);
                        return eval(game);
                    }
                    n >>= 1;
                }
            }
            _ => (),
        }

        let (j, mut m, u) = game.showbestfree_alt().unwrap();
        match u {
            0 => return Err(()),
            1 => {
                for x in 1..10 {
                    if m & 1 == 0 {
                        game.unsafe_choose_house(j, x);
                        return eval(game);
                    }
                    m >>= 1;
                }
            }
            _ => (),
        }

        let mut g;
        for x in 1..10 {
            if n & 1 == 0 {
                g = game.clone();
                g.unsafe_choose(i, x);
                match eval(g) {
                    Ok(k) => return Ok(k),
                    _ => (),
                }
            }
            n >>= 1;
        }
        Err(())
    } else {
        Ok(game)
    }
}
