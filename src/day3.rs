/*
--- Day 3: Binary Diagnostic ---

The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.

The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.

You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.

Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report. For example, given the following diagnostic report:

00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010

Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.

The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.

The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.

So, the gamma rate is the binary number 10110, or 22 in decimal.

The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.

Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
*/

pub fn gamma_rate(numbers: Vec<i32>, binary_number_length: usize) -> i32 {
    let mut counts_of_ones: Vec<i32> = vec![0; binary_number_length];
    let mut counts_of_zeroes: Vec<i32> = vec![0; binary_number_length];
    let mut gamma_rate: i32 = 0;

    for number in numbers {
        for position in 0..binary_number_length {
            match number & (1 << (binary_number_length - 1 - position)) > 0 {
                true => counts_of_ones[position] += 1,
                false => counts_of_zeroes[position] += 1,
            }
        }
    }

    for position in 0..binary_number_length {
        match counts_of_ones[position] > counts_of_zeroes[position] {
            true => gamma_rate = gamma_rate | (1 << (binary_number_length - 1 - position)),
            false => continue,
        }
    }

    gamma_rate
}

pub fn epsilon_rate(gamma_rate: i32, binary_number_length: usize) -> i32 {
    !gamma_rate & ((1 << binary_number_length) - 1)
}

pub fn part1(input: &str) -> i32 {
    let binary_number_length = input.split('\n').into_iter().next().unwrap().len();

    let numbers: Vec<i32> = input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|number| i32::from_str_radix(number, 2).unwrap())
        .collect();

    let gamma_rate = gamma_rate(numbers, binary_number_length);

    gamma_rate * epsilon_rate(gamma_rate, binary_number_length)
}

#[cfg(test)]
mod tests {
    use crate::day3::{epsilon_rate, gamma_rate, part1};

    #[test]
    fn test_gamma_rate() {
        assert_eq!(
            gamma_rate(vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10], 5),
            22
        );
    }

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(epsilon_rate(22, 5), 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 198);
    }
}
