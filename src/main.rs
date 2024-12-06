use std::fs::read_to_string;
use anyhow::anyhow;
use clap::Parser;

mod day1;
mod day2;
mod error;
mod parsing;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Alexander Kjäll <alexander.kjall@gmail.com>"
)]
struct Arguments {
    #[clap(short, long)]
    day: u8,
}

fn main() {
    let args: Arguments = Arguments::parse();

    let res = match args.day {
        1 => day1::calculate(&read_to_string("input/day1").expect("read day1")),
        2 => day2::calculate(&read_to_string("input/day2").expect("read day2")),
        _ => Err(anyhow!("illegal day")),
    };

    match res {
        Ok((part1, part2)) => println!("day {}\npart 1: {}\npart 2: {}", args.day, part1, part2),
        Err(err) => println!("{:?}", err),
    }
}
