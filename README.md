# About
Implementation of Conway's Game of Life in Rust

# Rules

The engine implements the standard rules as defined:

1. Any live cell with less than two live neighbours dies, **underpopulation**
1. Any live cell with two or three live neighbouds lives on, **survival**
1. Any live cell with more than three live neighbours dies, **overpopulation**
1. Any dead cell with three live neighbouds becomes live, **reproduction**

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

