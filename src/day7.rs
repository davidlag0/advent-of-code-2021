/*
--- Day 7: The Treachery of Whales ---

A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!

Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!

The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?

There's one major catch - crab submarines can only move horizontally.

You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.

For example, consider the following horizontal positions:

16,1,2,0,4,2,7,1,2,14

This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.

Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:

    Move from 16 to 2: 14 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 0 to 2: 2 fuel
    Move from 4 to 2: 2 fuel
    Move from 2 to 2: 0 fuel
    Move from 7 to 2: 5 fuel
    Move from 1 to 2: 1 fuel
    Move from 2 to 2: 0 fuel
    Move from 14 to 2: 12 fuel

This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).

Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?

--- Part Two ---

The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?

As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.

As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:

    Move from 16 to 5: 66 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 0 to 5: 15 fuel
    Move from 4 to 5: 1 fuel
    Move from 2 to 5: 6 fuel
    Move from 7 to 5: 3 fuel
    Move from 1 to 5: 10 fuel
    Move from 2 to 5: 6 fuel
    Move from 14 to 5: 45 fuel

This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.

Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
*/

pub struct CrabSubmarine {
    horizontal_position: u32,
    aligned_position: u32,
    fuel_cost: u32,
}

impl CrabSubmarine {
    pub fn new(horizontal_position: u32) -> Self {
        Self {
            horizontal_position,
            aligned_position: 0,
            fuel_cost: 0,
        }
    }

    pub fn align(&mut self, aligned_position: u32) -> u32 {
        self.aligned_position = aligned_position;
        self.fuel_cost = self.horizontal_position.max(aligned_position)
            - self.horizontal_position.min(aligned_position);

        self.fuel_cost
    }
}

pub fn parse_input(input: &str) -> Vec<u32> {
    let mut parsed_input: Vec<u32> = Vec::new();

    for number_string in input.trim().split(',') {
        match number_string.parse::<u32>() {
            Ok(number) => parsed_input.push(number),
            Err(_error) => continue,
        }
    }

    parsed_input.sort();
    parsed_input
}

pub fn part1(input: &str) -> i32 {
    let horizontal_positions = parse_input(input);
    let mut position_min: u32 = 0;
    let mut position_max: u32 = 0;
    let mut fuel_cost_totals: Vec<u32> = Vec::new();

    if let Some(position) = horizontal_positions.first() {
        position_min = *position
    }
    if let Some(position) = horizontal_positions.last() {
        position_max = *position
    }

    for aligned_position in position_min..position_max {
        let mut fuel_costs: Vec<u32> = Vec::new();

        for position in &horizontal_positions {
            fuel_costs.push(position.max(&aligned_position) - position.min(&aligned_position));
        }

        fuel_cost_totals.push(fuel_costs.iter().sum());
    }

    match fuel_cost_totals.iter().min() {
        Some(minimal_cost) => *minimal_cost as i32,
        None => -1,
    }
}

pub fn part2(input: &str) -> i32 {
    let horizontal_positions = parse_input(input);
    let mut position_min: u32 = 0;
    let mut position_max: u32 = 0;
    let mut fuel_cost_totals: Vec<u32> = Vec::new();
    let mut fuel_costs_index: Vec<u32> = vec![0];

    if let Some(position) = horizontal_positions.first() {
        position_min = *position
    }
    if let Some(position) = horizontal_positions.last() {
        position_max = *position
    }

    // Pre-calculate fuel costs for all positions.
    for position in 1..=position_max {
        fuel_costs_index.push(
            if let Some(fuel_cost) = fuel_costs_index.last() {
                fuel_cost
            } else {
                &0
            } + position,
        );
    }

    for aligned_position in position_min..position_max {
        let mut fuel_costs: Vec<u32> = Vec::new();

        for position in &horizontal_positions {
            fuel_costs.push(
                fuel_costs_index
                    [(position.max(&aligned_position) - position.min(&aligned_position)) as usize],
            );
        }

        fuel_cost_totals.push(fuel_costs.iter().sum());
    }

    match fuel_cost_totals.iter().min() {
        Some(minimal_cost) => *minimal_cost as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{parse_input, part1, part2, CrabSubmarine};

    static TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14
";

    #[test]
    fn test_crabsubmarine_new() {
        let crab_submarine = CrabSubmarine::new(16);

        assert_eq!(crab_submarine.horizontal_position, 16);
        assert_eq!(crab_submarine.aligned_position, 0);
        assert_eq!(crab_submarine.fuel_cost, 0);
    }

    #[test]
    fn test_crabsubmarine_align() {
        let mut crab_submarine: CrabSubmarine = CrabSubmarine::new(16);
        crab_submarine.align(2);

        assert_eq!(crab_submarine.horizontal_position, 16);
        assert_eq!(crab_submarine.aligned_position, 2);
        assert_eq!(crab_submarine.fuel_cost, 14);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 168);
    }
}
