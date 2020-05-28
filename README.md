# About
Implementation of Conway's Game of Life in Rust

# Rules

The engine implements the standard rules as defined:

1. Any live cell with less than two live neighbours dies, **underpopulation**
1. Any live cell with two or three live neighbouds lives on, **survival**
1. Any live cell with more than three live neighbours dies, **overpopulation**
1. Any dead cell with three live neighbouds becomes live, **reproduction**

# Usage

```
An implementation of Conway's Game of Life in rust

USAGE:
    gol [FLAGS] [OPTIONS] [file-name]

FLAGS:
    -b, --benchmark    Runs a benchmark (no rendering, no sleeping)
    -h, --help         Prints help information
    -V, --version      Prints version information
    -v, --verbose      Be verbose

OPTIONS:
    -i, --interval <interval>        Time (in ms) to sleep between renderings [default: 500]
    -t, --iterations <iterations>    Number of iterations to run [default: 30]

ARGS:
    <file-name>    File to seed the universe with
```

# Setting up

main.rs implements a basic oscillator, as an example.

1. Create a new universe:

```rust
let mut board = Universe::new(5, 5);
```

2. Initialize the board:

```rust
board.bless_cell(3, 1);
board.bless_cell(3, 2);
board.bless_cell(3, 3);
```

3. Iterate

```
board.tick();
```

# Seed files

The universe can be seeded from a file, where the dimenstions are
obtained from determining the maximum line length for the width, and
total line count for height.

Any character position which includes and `x` or `X` character is
blessed and made live in the initial render. An example seed file for
a pulsar would look as follows:

```

     x     x
     x     x
     xx   xx

 xxx  xx xx  xxx 
   x x x x x x
     xx   xx

     xx   xx
   x x x x x x
 xxx  xx xx  xxx 

     xx   xx
     x     x
     x     x

```
