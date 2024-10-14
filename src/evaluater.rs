use crate::game::Game;

pub fn eval(game: Game) -> Result<Game, ()> {
    let (w, n, v) = game.collective_showbestfree();

    match (w, v.len()) {
        (0, _) => Err(()),
        (_, 0) => Ok(game),
        _ => eval_tail(game, v, w, n),
    }
}

fn eval_local(
    game: Game,
    row: usize,
    col: usize,
    min_wgt: usize,
    lbs: Vec<(usize, usize)>,
) -> Result<Game, ()> {
    let (mut w, mut n, mut v) = game.collective_showbestfree_local(row, col);
    if min_wgt < w {
        (w, n, v) = game.collective_showbestfree_list(lbs.into_iter());
    } else if min_wgt == w {
        let (alt_w, alt_n, mut alt_v);
        (alt_w, alt_n, alt_v) = game.collective_showbestfree_list(lbs.into_iter());
        if alt_w < w {
            (w, n, v) = (alt_w, alt_n, alt_v);
        } else {
            alt_v.extend(v);
            v = alt_v;
        }
    }

    match (w, v.len()) {
        (0, _) => Err(()),
        (_, 0) => eval(game),
        _ => eval_tail(game, v, w, n),
    }
}

fn eval_tail(game: Game, mut v: Vec<(usize, usize)>, w: usize, mut n: u16) -> Result<Game, ()> {
    let mut g;
    if let Some((i, j)) = v.pop() {
        for x in 1..10 {
            match n & 1 {
                0 => {
                    g = game.clone();
                    g.unsafe_choose(i, j, x);
                    match eval_local(g, i, j, w, v.clone()) {
                        Ok(h) => return Ok(h),
                        _ => (),
                    }
                }
                _ => (),
            }
            n >>= 1;
        }
        return Err(());
    }
    unreachable!();
}
