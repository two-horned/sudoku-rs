use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let free = game.showbestfree();

    if free == None {
        return Ok(game);
    }

    let (row, col, fields) = free.unwrap();
    if fields.len() == 0 {
        return Err(());
    }

    let mut g;
    for i in fields {
        g = game.clone();
        g.unsafe_choose(row, col, i);
        match eval(g) {
            Ok(x) => return Ok(x),
            _ => (),
        }
    }
    Err(())
}
