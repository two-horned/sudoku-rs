use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    _eval(game, None)
}

fn _eval(game: Game, row_col: Option<(usize, usize)>) -> Result<Game, ()> {
    let free = match row_col {
        Some((row, col)) => game.showbestfree_local(row, col),
        _ => game.showbestfree(),
    };

    match free {
        None => return Ok(game),
        _ => (),
    }

    let (row, col, mut num) = free.unwrap();
    let mut g;

    for i in 1..10 {
        match num & 1 {
            0 => {
                g = game.clone();
                g.unsafe_choose(row, col, i);
                match _eval(g, Some((row, col))) {
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
