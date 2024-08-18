use crate::game::Game;

pub fn eval(game: Game) -> EvalReturn {
    let free = game.showbestfree();

    if free == None {
        return EvalReturn::SUCCESS(game.clone());
    }

    let (idx, fields) = free.unwrap();
    if fields.len() == 0 {
        return EvalReturn::FAILURE;
    }

    let mut g;
    for i in fields {
        g = game.clone();
        g.choose(idx, i);
        match eval(g) {
            EvalReturn::SUCCESS(x) => return EvalReturn::SUCCESS(x),
            _ => (),
        }
    }
    EvalReturn::FAILURE
}

#[derive(Debug)]
pub enum EvalReturn {
    SUCCESS(Game),
    FAILURE,
}
