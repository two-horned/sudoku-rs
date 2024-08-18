# sudoku-rs
This is Rust implementation of my sudoku solver.

The underlying algorithm is pretty simple.
The game representation stores bitmasks for every row, column and square,
so we can quickly check, which digits are allowed to be filled in.

The algorithm now goes through every field that's without value
and checks what possibilities there is. It then chooses to fill
some value for the field with least amount of options and backtracks
when a choice causes a game to be unsolvable.

The program reads from the standard input stream (stdin), so
one can run:

```sh
> cat test_puzzles.txt | cargo run --release
```

Each puzzles is represented by one line, just like in the `test_puzzles.txt` file.
