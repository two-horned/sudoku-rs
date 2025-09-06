use std::io::{self, Write};
use std::str::FromStr;
use std::time::Instant;
use sudoku::evaluater::Evaluater;
use sudoku::game::Game;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    stdout.write_all(b"Enter each sudoku puzzle as one line. Press Ctr-D to quit.\n")?;

    let mut evaluater = Evaluater::new();

    let start = Instant::now();

    for line in io::stdin().lines() {
        let mut buf = line?;

        stdout.write_all(format!("Input:       {}\n", buf).as_bytes())?;

        let now = Instant::now();

        match Game::from_str(&buf) {
            Err(err) => {
                stderr.write(format!("{}\n", err).as_bytes())?;
                continue;
            }
            Ok(x) => match evaluater.eval(x) {
                Ok(x) => stdout.write_all(format!("Solution:    {}\n", x).as_bytes())?,
                _ => stdout.write_all(b"Game cannot be solved.\n")?,
            },
        }

        buf.clear();
        stdout
            .write_all(format!("Time needed: {}μs.\n\n", now.elapsed().as_micros()).as_bytes())?;
    }

    stdout.write_all(
        format!(
            "Total time needed: {}ms.\nBye\n",
            start.elapsed().as_millis()
        )
        .as_bytes(),
    )?;

    Ok(())
}
