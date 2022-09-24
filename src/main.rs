use aoc_rust_2021::day1;
use aoc_rust_2021::day2;
use aoc_rust_2021::day3;
use aoc_rust_2021::day4;
use aoc_rust_2021::day5;
use aoc_rust_2021::day6;
use std::env;
use std::fs;
use std::process;

fn load_file(filename: &str) -> std::string::String {
    fs::read_to_string(filename).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("not enough arguments");
        process::exit(1);
    }

    let base_path = args[1].clone();

    println!(
        "Solution of Day 1, Part 1: {}",
        day1::part1(&load_file(&[&base_path, "day1.txt"].concat()))
    );

    println!(
        "Solution of Day 1, Part 2: {}",
        day1::part2(&load_file(&[&base_path, "day1.txt"].concat()))
    );

    println!(
        "Solution of Day 2, Part 1: {}",
        day2::part1(&load_file(&[&base_path, "day2.txt"].concat()))
    );

    println!(
        "Solution of Day 2, Part 2: {}",
        day2::part2(&load_file(&[&base_path, "day2.txt"].concat()))
    );

    println!(
        "Solution of Day 3, Part 1: {}",
        day3::part1(&load_file(&[&base_path, "day3.txt"].concat()))
    );

    println!(
        "Solution of Day 3, Part 2: {}",
        day3::part2(&load_file(&[&base_path, "day3.txt"].concat()))
    );

    println!(
        "Solution of Day 4, Part 1: {}",
        day4::part1(&load_file(&[&base_path, "day4.txt"].concat()))
    );

    println!(
        "Solution of Day 4, Part 2: {}",
        day4::part2(&load_file(&[&base_path, "day4.txt"].concat()))
    );

    println!(
        "Solution of Day 5, Part 1: {}",
        day5::part1(&load_file(&[&base_path, "day5.txt"].concat()))
    );

    println!(
        "Solution of Day 5, Part 2: {}",
        day5::part2(&load_file(&[&base_path, "day5.txt"].concat()))
    );

    println!(
        "Solution of Day 6, Part 1: {}",
        day6::part1(&load_file(&[&base_path, "day6.txt"].concat()))
    );

    println!(
        "Solution of Day 6, Part 2: {}",
        day6::part2(&load_file(&[&base_path, "day6.txt"].concat()))
    );
}
