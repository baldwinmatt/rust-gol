use std::{thread, time};

use gol::Universe;

fn main() {
    let half_second = time::Duration::from_millis(500);

    let mut board = Universe::new(5, 5);

    board.bless_cell(3, 1);
    board.bless_cell(3, 2);
    board.bless_cell(3, 3);

    for _ in 0..128 {
        print!("\x1B[2J");
        println!("{}", board);

        thread::sleep(half_second);
        board.tick();
    }
}
