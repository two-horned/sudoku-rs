use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let need = game.showbestneed();
    let free = game.showbestfree();

    if need == None {
        return Ok(game);
    }

    let (idx, fields) = free.unwrap();
    let (dig, idices) = need.unwrap();

    if 3 < idices.len() {
        if fields.len() == 0 {
            return Err(());
        }

        let mut g;
        for i in fields {
            g = game.clone();
            g.unsafe_choose(idx, i);
            match eval(g) {
                Ok(x) => return Ok(x),
                _ => (),
            }
        }
    } else {
        if idices.len() == 0 {
            return Err(());
        }

        let mut g;
        for i in idices {
            g = game.clone();
            g.unsafe_choose(i, dig);
            match eval(g) {
                Ok(x) => return Ok(x),
                _ => (),
            }
        }
    }
    Err(())
}
