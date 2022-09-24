/*
--- Day 6: Lanternfish ---

The sea floor is getting steeper. Maybe the sleigh keys got carried this way?

A massive school of glowing lanternfish swims past. They must spawn quickly to reach such large numbers - maybe exponentially quickly? You should model their growth rate to be sure.

Although you know nothing about this specific species of lanternfish, you make some guesses about their attributes. Surely, each lanternfish creates a new lanternfish once every 7 days.

However, this process isn't necessarily synchronized between every lanternfish - one lanternfish might have 2 days left until it creates another lanternfish, while another might have 4. So, you can model each fish as a single number that represents the number of days until it creates a new lanternfish.

Furthermore, you reason, a new lanternfish would surely need slightly longer before it's capable of producing more lanternfish: two more days for its first cycle.

So, suppose you have a lanternfish with an internal timer value of 3:

    After one day, its internal timer would become 2.
    After another day, its internal timer would become 1.
    After another day, its internal timer would become 0.
    After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
    After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.

A lanternfish that creates a new fish resets its timer to 6, not 7 (because 0 is included as a valid timer value). The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.

Realizing what you're trying to do, the submarine automatically produces a list of the ages of several hundred nearby lanternfish (your puzzle input). For example, suppose you were given the following list:

3,4,3,1,2

This list means that the first fish has an internal timer of 3, the second fish has an internal timer of 4, and so on until the fifth fish, which has an internal timer of 2. Simulating these fish over several days would proceed as follows:

Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8

Each day, a 0 becomes a 6 and adds a new 8 to the end of the list, while each other number decreases by 1 if it was present at the start of the day.

In this example, after 18 days, there are a total of 26 fish. After 80 days, there would be a total of 5934.

Find a way to simulate lanternfish. How many lanternfish would there be after 80 days?

--- Part Two ---

Suppose the lanternfish live forever and have unlimited food and space. Would they take over the entire ocean?

After 256 days in the example above, there would be a total of 26984457539 lanternfish!

How many lanternfish would there be after 256 days?
*/

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Lanternfish {
    internal_timer: usize,
}

impl Lanternfish {
    pub fn new(timer_start: usize) -> Self {
        let internal_timer = timer_start;

        Self { internal_timer }
    }

    pub fn simulate(&mut self) {
        match &self.internal_timer {
            0 => self.internal_timer = 6,
            _ => self.internal_timer -= 1,
        }
    }
}

impl Eq for Lanternfish {}

#[derive(PartialEq)]
pub struct FishPool {
    pool: Vec<Lanternfish>,
}

impl FishPool {
    pub fn new(fishes: Vec<usize>) -> Self {
        let mut pool = Vec::new();

        for fish in fishes {
            pool.push(Lanternfish::new(fish))
        }
        Self { pool }
    }

    pub fn add_fish(&mut self, quantity: u32) {
        for _ in 0..quantity {
            self.pool.push(Lanternfish::new(8));
        }
    }

    pub fn simulate(&mut self, number_of_days: u32) {
        for _ in 0..number_of_days {
            let mut quantity_of_fish_to_add = 0;

            for fish in &mut self.pool {
                if fish.internal_timer == 0 {
                    quantity_of_fish_to_add += 1;
                }

                fish.simulate();
            }
            self.add_fish(quantity_of_fish_to_add);
        }
    }
}

impl Eq for FishPool {}

pub struct FishPoolv2 {
    pool: VecDeque<usize>,
}

impl FishPoolv2 {
    pub fn new(fishes: Vec<usize>) -> Self {
        let mut pool = VecDeque::from(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);

        for fish in fishes {
            pool[fish] += 1;
        }
        Self { pool }
    }

    pub fn simulate(&mut self, number_of_days: u32) {
        for _ in 0..number_of_days {
            let number_of_fish_0;

            match self.pool.pop_front() {
                Some(number) => number_of_fish_0 = number,
                None => {
                    panic!("This case should not happen because we're using a fixed length array")
                }
            }

            self.pool.push_back(number_of_fish_0);
            self.pool[6] += number_of_fish_0;
        }
    }
}

pub fn parse_input(input: &str) -> Vec<usize> {
    let mut parsed_input: Vec<usize> = Vec::new();

    for number_string in input.trim().split(',') {
        match number_string.parse::<usize>() {
            Ok(number) => parsed_input.push(number),
            Err(_error) => continue,
        }
    }

    parsed_input
}

pub fn part1(input: &str) -> usize {
    let mut fish_pool = FishPool::new(parse_input(input));
    fish_pool.simulate(80);

    fish_pool.pool.len()
}

pub fn part2(input: &str) -> usize {
    let mut fish_pool = FishPoolv2::new(parse_input(input));
    fish_pool.simulate(256);

    fish_pool.pool.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day6::{parse_input, part1, part2, FishPool, FishPoolv2, Lanternfish};
    use std::collections::VecDeque;

    static TEST_INPUT: &str = "3,4,3,1,2
";

    #[test]
    fn test_lanternfish() {
        let lanternfish = Lanternfish::new(3);
        assert_eq!(lanternfish.internal_timer, 3);
    }

    #[test]
    fn test_lanternfish_simulate() {
        let mut lanternfish = Lanternfish::new(0);
        lanternfish.simulate();
        assert_eq!(lanternfish.internal_timer, 6);
    }

    #[test]
    fn test_fishpool_addfish() {
        let mut fish_pool = FishPool::new(vec![]);
        fish_pool.add_fish(3);

        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(8),
                Lanternfish::new(8),
                Lanternfish::new(8)
            ]
        );
    }

    #[test]
    fn test_fishpool_simulate_1_day() {
        let mut fish_pool = FishPool::new(parse_input(TEST_INPUT));
        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(3),
                Lanternfish::new(4),
                Lanternfish::new(3),
                Lanternfish::new(1),
                Lanternfish::new(2)
            ]
        );
        fish_pool.simulate(1);
        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(2),
                Lanternfish::new(3),
                Lanternfish::new(2),
                Lanternfish::new(0),
                Lanternfish::new(1)
            ]
        );
    }

    #[test]
    fn test_fishpool_simulate_2_days() {
        let mut fish_pool = FishPool::new(parse_input(TEST_INPUT));
        fish_pool.simulate(2);
        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(1),
                Lanternfish::new(2),
                Lanternfish::new(1),
                Lanternfish::new(6),
                Lanternfish::new(0),
                Lanternfish::new(8),
            ]
        );
    }

    #[test]
    fn test_fishpool_simulate_4_days() {
        let mut fish_pool = FishPool::new(parse_input(TEST_INPUT));
        fish_pool.simulate(4);
        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(6),
                Lanternfish::new(0),
                Lanternfish::new(6),
                Lanternfish::new(4),
                Lanternfish::new(5),
                Lanternfish::new(6),
                Lanternfish::new(7),
                Lanternfish::new(8),
                Lanternfish::new(8),
            ]
        );
    }

    #[test]
    fn test_fishpool_simulate_18_days() {
        let mut fish_pool = FishPool::new(parse_input(TEST_INPUT));
        fish_pool.simulate(18);
        assert_eq!(
            fish_pool.pool,
            vec![
                Lanternfish::new(6),
                Lanternfish::new(0),
                Lanternfish::new(6),
                Lanternfish::new(4),
                Lanternfish::new(5),
                Lanternfish::new(6),
                Lanternfish::new(0),
                Lanternfish::new(1),
                Lanternfish::new(1),
                Lanternfish::new(2),
                Lanternfish::new(6),
                Lanternfish::new(0),
                Lanternfish::new(1),
                Lanternfish::new(1),
                Lanternfish::new(1),
                Lanternfish::new(2),
                Lanternfish::new(2),
                Lanternfish::new(3),
                Lanternfish::new(3),
                Lanternfish::new(4),
                Lanternfish::new(6),
                Lanternfish::new(7),
                Lanternfish::new(8),
                Lanternfish::new(8),
                Lanternfish::new(8),
                Lanternfish::new(8),
            ]
        );
    }

    #[test]
    fn test_fishpoolv2() {
        let fish_pool = FishPoolv2::new(parse_input(TEST_INPUT));
        assert_eq!(
            fish_pool.pool,
            VecDeque::from(vec![0, 1, 1, 2, 1, 0, 0, 0, 0])
        );
    }

    #[test]
    fn test_fishpoolv2_simulate_1_day() {
        let mut fish_pool = FishPoolv2::new(parse_input(TEST_INPUT));
        fish_pool.simulate(1);
        assert_eq!(
            fish_pool.pool,
            VecDeque::from(vec![1, 1, 2, 1, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn test_fishpoolv2_simulate_2_day() {
        let mut fish_pool = FishPoolv2::new(parse_input(TEST_INPUT));
        fish_pool.simulate(2);
        assert_eq!(
            fish_pool.pool,
            VecDeque::from(vec![1, 2, 1, 0, 0, 0, 1, 0, 1])
        );
    }

    #[test]
    fn test_fishpoolv2_simulate_4_days() {
        let mut fish_pool = FishPoolv2::new(parse_input(TEST_INPUT));
        fish_pool.simulate(4);
        assert_eq!(
            fish_pool.pool,
            VecDeque::from(vec![1, 0, 0, 0, 1, 1, 3, 1, 2])
        );
    }

    #[test]
    fn test_fishpoolv2_simulate_18_days() {
        let mut fish_pool = FishPoolv2::new(parse_input(TEST_INPUT));
        fish_pool.simulate(18);
        assert_eq!(
            fish_pool.pool,
            VecDeque::from(vec![3, 5, 3, 2, 2, 1, 5, 1, 4])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 26984457539);
    }
}
