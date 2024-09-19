use std::io;
use std::str::FromStr;
use std::time::Instant;
use sudoku::evaluater::eval;
use sudoku::game::Game;

fn main() -> io::Result<()> {
    println!("Enter each sudoku puzzle as one line.");
    println!("Press Ctr-D to quit.");

    for line in io::stdin().lines() {
        let e = line?;
        println!("\nInput:       {}", e);

        let now = Instant::now();

        match Game::from_str(&e) {
            Err(msg) => {
                eprintln!("{:?}", msg);
                continue;
            }
            Ok(x) => match eval(Game::from(x)) {
                Ok(x) => println!("Solution:    {}", x),
                _ => println!("Game cannot be solved."),
            },
        }

        println!("Time needed: {}ms.", now.elapsed().as_millis());
    }
    Ok(println!("Bye."))
}
