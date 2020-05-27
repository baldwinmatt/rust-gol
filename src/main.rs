use std::path::Path;
use std::{thread, time};

use gol::Universe;

fn main() {
    let half_second = time::Duration::from_millis(500);

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            let file = Path::new(&args[1]);
            run_file(&file, &half_second);
        }
        _ => oscillator(&half_second),
    }
}

fn oscillator(period: &time::Duration) {
    let mut board = Universe::new(5, 5);

    board.bless_cell(3, 1);
    board.bless_cell(3, 2);
    board.bless_cell(3, 3);

    run_board(&mut board, &period, 10);
}

fn run_board(board: &mut Universe, period: &time::Duration, loops: u32) {
    for _ in 0..loops {
        print!("\x1B[2J");
        println!("{}", board);

        thread::sleep(*period);
        board.tick();
    }
}

fn run_file(file: &Path, period: &time::Duration) {
    let mut board = Universe::from_file(file).unwrap();

    run_board(&mut board, &period, 30);
}
