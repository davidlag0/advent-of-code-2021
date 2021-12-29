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

--- Part Two ---

Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.

Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:

    Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
    If you only have one number left, stop; this is the rating value for which you are searching.
    Otherwise, repeat the process, considering the next bit to the right.

The bit criteria depends on which type of rating value you want to find:

    To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
    To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.

For example, to determine the oxygen generator rating value using the same example diagnostic report from above:

    Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
    Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
    In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
    In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
    In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
    As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.

Then, to determine the CO2 scrubber rating value from the same example above:

    Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
    Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
    In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
    As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.

Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.

Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
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

pub fn oxygen_generator_rating(binary_numbers: &Vec<&str>) -> i32 {
    rating(binary_numbers, &oxygen_generator_condition)
}

pub fn co2_scrubber_rating(binary_numbers: &Vec<&str>) -> i32 {
    rating(binary_numbers, &co2_scrubber_condition)
}

pub fn rating(
    binary_numbers: &Vec<&str>,
    condition_function: &dyn Fn(usize, usize) -> bool,
) -> i32 {
    let binary_number_length = binary_numbers[0].len();

    let mut rating_list = binary_numbers.clone();

    for bit_position in 0..binary_number_length {
        if rating_list.len() == 1 {
            break;
        }

        let number_of_ones = &rating_list
            .iter()
            .filter(|rating| &rating[bit_position..bit_position + 1] == "1")
            .count();
        let number_of_zeroes = &rating_list
            .iter()
            .filter(|rating| &rating[bit_position..bit_position + 1] == "0")
            .count();

        let bit_to_keep = if condition_function(*number_of_ones, *number_of_zeroes) {
            "1"
        } else {
            "0"
        };
        rating_list.retain(|rating| &rating[bit_position..bit_position + 1] == bit_to_keep);
    }

    i32::from_str_radix(rating_list[0], 2).unwrap()
}

pub fn oxygen_generator_condition(number_of_ones: usize, number_of_zeroes: usize) -> bool {
    number_of_ones >= number_of_zeroes
}

pub fn co2_scrubber_condition(number_of_ones: usize, number_of_zeroes: usize) -> bool {
    number_of_ones < number_of_zeroes
}

pub fn part2(input: &str) -> i32 {
    let rating_list: Vec<&str> = input.lines().collect();

    oxygen_generator_rating(&rating_list) * co2_scrubber_rating(&rating_list)
}

#[cfg(test)]
mod tests {
    use crate::day3::{
        co2_scrubber_rating, epsilon_rate, gamma_rate, oxygen_generator_rating, part1, part2,
    };

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

    #[test]
    fn test_oxygen_generator_rating() {
        assert_eq!(
            oxygen_generator_rating(&vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010"
            ]),
            23
        );
    }

    #[test]
    fn test_co2_scrubber_rating() {
        assert_eq!(
            co2_scrubber_rating(&vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010"
            ]),
            10
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"), 230);
    }
}
