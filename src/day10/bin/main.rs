//! # Advent of Code - Day 10

mod part1;
mod part2;

fn main() {
    let _input = include_bytes!("../resources/input.txt");

    println!("--- Part One ---");
    println!("Result: {}", part1::result().unwrap());

    println!("--- Part Two ---");
    println!("Result: {}", part2::result().unwrap());
}
