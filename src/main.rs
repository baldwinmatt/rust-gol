#[macro_use]
extern crate tempus_fugit;

use structopt::StructOpt;

use std::path::PathBuf;
use std::{thread, time};

use gol::Universe;

#[derive(Debug, StructOpt)]
#[structopt(name = "gol", about)]
struct Opt {
    /// Be verbose
    #[structopt(short, long)]
    verbose: bool,

    /// Time (in ms) to sleep between renderings
    #[structopt(short, long, default_value = "500")]
    interval: u64,

    /// Number of iterations to run
    #[structopt(short = "t", long, default_value = "30")]
    iterations: u32,

    /// Runs a benchmark (no rendering, no sleeping)
    #[structopt(short, long)]
    benchmark: bool,

    /// File to seed the universe with
    file_name: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt);

    let interval = time::Duration::from_millis(opt.interval);
    let iterations = opt.iterations;

    let mut universe = oscillator();

    if let Some(filename) = opt.file_name {
        if opt.verbose {
            println!("Seeding universe from {}", filename.to_str().unwrap());
        }

        let file = filename.as_path();
        universe = Universe::from_file(&file).unwrap();
    }

    if opt.benchmark {
        println!("Running benchmark with {} iterations", iterations);

        if opt.verbose {
            println!("Initial state:");
            println!("{}", universe);
        }

        let (universe, measurement) = measure! {{
            benchmark(&mut universe, iterations);
            universe
        }};

        if opt.verbose {
            println!("End state:");
            println!("{}", universe);
        }

        println!("Run {} iterations in {}", iterations, measurement);
    } else {
        run_board(&mut universe, &interval, iterations, opt.verbose);
    }
}

fn oscillator() -> Universe {
    let mut board = Universe::new(5, 5);

    board.bless_cell(3, 1);
    board.bless_cell(3, 2);
    board.bless_cell(3, 3);

    board
}

fn run_board(board: &mut Universe, interval: &time::Duration, iterations: u32, verbose: bool) {
    for i in 0..iterations {
        print!("\x1B[2J");

        if verbose {
            println!("Generation {} of {}", (i + 1), iterations);
        }

        println!("{}", board);

        thread::sleep(*interval);
        board.tick();
    }
}

fn benchmark(board: &mut Universe, iterations: u32) {
    for _ in 0..iterations {
        board.tick();
    }
}
