/*
--- Day 8: Seven Segment Search ---

You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.

As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.

Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.

The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)

So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.

For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.

For example, here is what you might see in a single entry in your notes:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

(The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)

Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.

Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.

For now, focus on the easy digits. Consider this larger example:

be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce

Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).

In the output values, how many times do digits 1, 4, 7, or 8 appear?

--- Part Two ---

Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:

acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf

After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc

So, the unique signal patterns would correspond to the following digits:

    acedgfb: 8
    cdfbe: 5
    gcdfa: 2
    fbcad: 3
    dab: 7
    cefabd: 9
    cdfgeb: 6
    eafb: 4
    cagedb: 0
    ab: 1

Then, the four digits of the output value can be decoded:

    cdfeb: 5
    fcadb: 3
    cdfeb: 5
    cdbaf: 3

Therefore, the output value for this entry is 5353.

Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:

    fdgacbe cefdb cefbgd gcbe: 8394
    fcgedb cgb dgebacf gc: 9781
    cg cg fdcagb cbg: 1197
    efabcd cedba gadfec cb: 9361
    gecf egdcabf bgf bfgea: 4873
    gebdcfa ecba ca fadegcb: 8418
    cefg dcbef fcge gbcadfe: 4548
    ed bcgafe cdgba cbgef: 1625
    gbdfcae bgc cg cgb: 8717
    fgae cfgab fg bagce: 4315

Adding all of the output values in this larger example produces 61229.

For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
*/

use std::collections::HashMap;
use std::collections::HashSet;

pub fn sort_string_chars(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .map(|str| {
            let mut chars = str.chars().collect::<Vec<_>>();
            chars.sort();
            chars.iter().collect::<String>()
        })
        .collect()
}

pub fn part1(input: &str) -> Result<u64, &'static str> {
    let mut segment_counter: u64 = 0;

    for line in input.trim().split('\n') {
        for segment in line.split(" | ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()
        {
            match segment.len() {
                2 | 3 | 4 | 7 => segment_counter += 1,
                _ => {}
            }
        }
    }
    Ok(segment_counter)
}

pub fn is_9(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    let signal_pattern_4 = signal_patterns_map
        .iter()
        .find_map(|(key, &value)| match value {
            4 => Some(key.as_str()),
            _ => None,
        })
        .unwrap_or("");

    (signal_pattern.len() == 6)
        && (HashSet::<char>::from_iter(signal_pattern.chars())
            .is_superset(&HashSet::<char>::from_iter(signal_pattern_4.chars())))
}

pub fn is_0(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    let signal_pattern_1 = signal_patterns_map
        .iter()
        .find_map(|(key, &value)| match value {
            1 => Some(key.as_str()),
            _ => None,
        })
        .unwrap_or("");

    (signal_pattern.len() == 6)
        && !is_9(signal_pattern, signal_patterns_map)
        && (HashSet::<char>::from_iter(signal_pattern.chars())
            .is_superset(&HashSet::<char>::from_iter(signal_pattern_1.chars())))
}

pub fn is_6(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    (signal_pattern.len() == 6)
        && !is_9(signal_pattern, signal_patterns_map)
        && !is_0(signal_pattern, signal_patterns_map)
}

pub fn is_3(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    let signal_pattern_1 = signal_patterns_map
        .iter()
        .find_map(|(key, &value)| match value {
            1 => Some(key.as_str()),
            _ => None,
        })
        .unwrap_or("");

    (signal_pattern.len() == 5)
        && (HashSet::<char>::from_iter(signal_pattern.chars())
            .is_superset(&HashSet::<char>::from_iter(signal_pattern_1.chars())))
}

pub fn is_5(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    let signal_pattern_9 = signal_patterns_map
        .iter()
        .find_map(|(key, &value)| match value {
            9 => Some(key.as_str()),
            _ => None,
        })
        .unwrap_or("");

    (signal_pattern.len() == 5)
        && !is_3(signal_pattern, signal_patterns_map)
        && (HashSet::<char>::from_iter(signal_pattern.chars())
            .is_subset(&HashSet::<char>::from_iter(signal_pattern_9.chars())))
}

pub fn is_2(signal_pattern: &String, signal_patterns_map: &HashMap<&String, u32>) -> bool {
    (signal_pattern.len() == 5)
        && !is_3(signal_pattern, signal_patterns_map)
        && !is_5(signal_pattern, signal_patterns_map)
}

/*
My approach seems way too complicated with the HashSet and HashMap,
although it is a good learning exercice!

I like this approach, very straightforward:
https://arturh85.github.io/adventofcode-rust-2021/src/adventofcode_rust_2021/day8.rs.html#1-347
*/

pub fn part2(input: &str) -> Result<u64, &'static str> {
    /*
    2 segments: 1
    3 segments: 7
    4 segments: 4
    7 segments: 8

    6 segments: 0, 6, 9 (9 is superset of 4, 0 is superset of 1, by elimination, that leaves 6)
    5 segments: 2, 3, 5 (3 is superset of 1, 5 is subset of 9, by elimination, that leaves 2)
    */

    let mut sum_of_digits: u64 = 0;

    for line in input.trim().split('\n') {
        let splitted_line = line.split(" | ").collect::<Vec<&str>>();

        let mut signal_patterns: Vec<String> =
            sort_string_chars(splitted_line[0].split(' ').collect::<Vec<&str>>());
        signal_patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

        let digit_output_values: Vec<String> =
            sort_string_chars(splitted_line[1].split(' ').collect::<Vec<&str>>());

        let mut signal_patterns_map: HashMap<_, _> = signal_patterns
            .iter()
            .filter_map(|pattern| match pattern.len() {
                2 => Some((pattern, 1)),
                3 => Some((pattern, 7)),
                4 => Some((pattern, 4)),
                7 => Some((pattern, 8)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        for signal_pattern in &signal_patterns {
            if is_9(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 9);
            }
            if is_0(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 0);
            }
            if is_6(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 6);
            }
            if is_3(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 3);
            }
            if is_5(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 5);
            }
            if is_2(signal_pattern, &signal_patterns_map) {
                signal_patterns_map.insert(signal_pattern, 2);
            }
        }

        match digit_output_values
            .iter()
            .map(|digit| match signal_patterns_map.get(&digit) {
                Some(value) => value.to_string(),
                None => String::from(""),
            })
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
        {
            Ok(number) => {
                sum_of_digits += number;
            }
            Err(_error) => continue,
        }
    }

    Ok(sum_of_digits)
}

#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2};

    static TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 61229);
    }
}
