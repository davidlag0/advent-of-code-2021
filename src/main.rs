use aoc_rust_2021::day1;
use aoc_rust_2021::day2;
use aoc_rust_2021::day3;
use aoc_rust_2021::day4;
use aoc_rust_2021::day5;
use aoc_rust_2021::day6;
use aoc_rust_2021::day7;
use aoc_rust_2021::day8;
use aoc_rust_2021::day9;
use std::env;
use std::fs;
use std::process;
use std::time::Instant;

fn load_file(filename: &str) -> std::string::String {
    fs::read_to_string(filename).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut chrono_start;
    let mut chrono_stop;
    let mut solution: i32;
    let mut solution_result;
    let mut total_time: u128 = 0;

    if args.len() < 2 {
        println!("not enough arguments");
        process::exit(1);
    }

    let base_path = args[1].clone();

    chrono_start = Instant::now();
    solution = day1::part1(&load_file(&[&base_path, "day1.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 1, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day1::part2(&load_file(&[&base_path, "day1.txt"].concat())) as i32;
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 1, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day2::part1(&load_file(&[&base_path, "day2.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 2, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day2::part2(&load_file(&[&base_path, "day2.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 2, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day3::part1(&load_file(&[&base_path, "day3.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 3, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day3::part2(&load_file(&[&base_path, "day3.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 3, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day4::part1(&load_file(&[&base_path, "day4.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 4, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day4::part2(&load_file(&[&base_path, "day4.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 4, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day5::part1(&load_file(&[&base_path, "day5.txt"].concat())) as i32;
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 5, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day5::part2(&load_file(&[&base_path, "day5.txt"].concat())) as i32;
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 5, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day6::part1(&load_file(&[&base_path, "day6.txt"].concat())) as i32;
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 6, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day6::part2(&load_file(&[&base_path, "day6.txt"].concat())) as i32;
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 6, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day7::part1(&load_file(&[&base_path, "day7.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 7, Part 1: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution = day7::part2(&load_file(&[&base_path, "day7.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;
    println!(
        "Solution of Day 7, Part 2: {}, Time: {}μs",
        solution, chrono_stop
    );

    chrono_start = Instant::now();
    solution_result = day8::part1(&load_file(&[&base_path, "day8.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 8, Part 1: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 8, Part 1: {}",
            error
        ),
    }

    chrono_start = Instant::now();
    solution_result = day8::part2(&load_file(&[&base_path, "day8.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 8, Part 2: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 8, Part 2: {}",
            error
        ),
    }

    chrono_start = Instant::now();
    solution_result = day9::part1(&load_file(&[&base_path, "day9.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 9, Part 1: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 9, Part 1: {}",
            error
        ),
    }

    chrono_start = Instant::now();
    solution_result = day9::part2(&load_file(&[&base_path, "day9.txt"].concat()));
    chrono_stop = chrono_start.elapsed().as_micros();
    total_time += chrono_stop;

    match solution_result {
        Ok(solution) => println!(
            "Solution of Day 9, Part 2: {:?}, Time: {}μs",
            solution, chrono_stop
        ),
        Err(error) => println!(
            "A problem occured to solve the problem of Day 9, Part 2: {}, Time: {}μs",
            error, chrono_stop
        ),
    }

    println!("\nTotal Time: {}μs", total_time);
}
