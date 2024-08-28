use std::io;
use sudoku::evaluater::eval;
use sudoku::game::Game;

fn to_dig(c: char) -> Result<u8, String> {
    match c {
        '.' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        _ => Err(format!("Failed to convert character '{}'.", c)),
    }
}

fn string_to_board(s: &str) -> Result<[u8; 81], String> {
    let chars: Vec<u8> = s.chars().map(to_dig).collect::<Result<Vec<u8>, _>>()?;

    match chars.try_into() {
        Ok(x) => Ok(x),
        Err(_) => Err(String::from("Wrong character length.")),
    }
}

fn main() -> io::Result<()> {
    println!("Enter each sudoku puzzle as one line.");
    println!("Enter empty line or press Ctr-D to quit.");

    for line in io::stdin().lines() {
        let e = line?;
        if e == "" {
            break;
        }
        println!("Input:    {}", e);

        match string_to_board(&e) {
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
            Ok(x) => match eval(Game::from(x)) {
                Ok(x) => println!("Solution: {}", x),
                _ => println!("Game cannot be solved."),
            },
        }
    }
    Ok(println!("Bye."))
}
