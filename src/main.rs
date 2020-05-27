extern crate clap;
#[macro_use] extern crate tempus_fugit;

use clap::{Arg, App};

use std::path::Path;
use std::{thread, time};

use gol::Universe;

fn main() {

    let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(Arg::with_name("file")
                .value_name("FILE")
                .help("Use FILE as seed for universe")
                .takes_value(true)
                .index(1))
            .arg(Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("TIME_MS")
                .help("Time (in ms) to sleep between renderings")
                .takes_value(true))
            .arg(Arg::with_name("benchmark")
                .short("b")
                .long("benchmark")
                .help("Runs a benchmark (no rendering, no sleeping"))
            .arg(Arg::with_name("iterations")
                .short("t")
                .long("iterations")
                .value_name("ITERATIONS")
                .help("Number of iterations to run for")
                .takes_value(true))
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Be verbose"))
            .get_matches();

    let interval = matches.value_of("interval").unwrap_or("500");
    let interval = time::Duration::from_millis(interval.parse::<u64>().unwrap());
    let iterations = matches.value_of("iterations").unwrap_or("30");
    let iterations = iterations.parse::<u32>().unwrap();

    let verbose = matches.is_present("verbose");
    let bench = matches.is_present("benchmark");

    let mut universe = oscillator();

    if let Some(filename) = matches.value_of("file") {
        if verbose { println!("Seeding universe from {}", filename); }

        let file = Path::new(&filename);
        universe = Universe::from_file(&file).unwrap();
    }

    if bench {
        println!("Running benchmark with {} iterations", iterations);

        if verbose {
            println!("Initial state:");
            println!("{}", universe);
        }

        let (universe, measurement) = measure! {{
            benchmark(&mut universe, iterations);
            universe
        }};

        if verbose {
            println!("End state:");
            println!("{}", universe);
        }

        println!("Run {} iterations in {}", iterations, measurement);
    } else {
        run_board(&mut universe, &interval, iterations, verbose);
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

        if verbose { println!("Generation {} of {}", (i + 1), iterations); }

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
