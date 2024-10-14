use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let (i, mut n) = game.showbestfree();

    if 80 < i {
        return Ok(game);
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
}
