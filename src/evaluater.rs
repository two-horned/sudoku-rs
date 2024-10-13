use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let free = game.showbestfree();

    if free == None {
        return Ok(game);
    }

    let (row, col, mut num) = free.unwrap();
    let mut g;

    for i in 1..10 {
        match num & 1 {
            0 => {
                g = game.clone();
                g.unsafe_choose(row, col, i);
                match eval(g) {
                    Ok(x) => return Ok(x),
                    _ => (),
                }
            }
            _ => (),
        }
        num >>= 1;
    }
    Err(())
}
