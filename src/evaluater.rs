use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let free = game.showbestfree();

    match free {
        None => Ok(game),
        Some((i, j, n)) => eval_tail(game, i, j, n),
    }
}

fn eval_local(game: Game, row: usize, col: usize) -> Result<Game, ()> {
    let free = game.showbestfree_local(row, col);

    match free {
        None => eval(game),
        Some((i, j, n)) => eval_tail(game, i, j, n),
    }
}

fn eval_tail(game: Game, row: usize, col: usize, mut num: u16) -> Result<Game, ()> {
    let mut g;
    for i in 1..10 {
        match num & 1 {
            0 => {
                g = game.clone();
                g.unsafe_choose(row, col, i);
                match eval_local(g, row, col) {
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
