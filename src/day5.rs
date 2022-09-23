/*
--- Day 5: Hydrothermal Venture ---

You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.

They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:

0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2

Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:

    An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
    An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.

For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.

So, the horizontal and vertical lines from the above list would produce the following diagram:

.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....

In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.

To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.

Consider only horizontal and vertical lines. At how many points do at least two lines overlap?

--- Part Two ---

Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.

Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:

    An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
    An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.

Considering all lines from the above example would now produce the following diagram:

1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....

You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.

Consider all of the lines. At how many points do at least two lines overlap?
*/

use std::collections::HashMap;
use std::fmt;
use std::str::Split;

#[derive(Debug, Copy, Clone, PartialEq, std::hash::Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Eq for Point {}

pub fn extract_coordinates_from_line(line: &str) -> Result<(Point, Point), &'static str> {
    match line {
        "" => Err("Empty line"),
        _ => {
            let parts: Vec<&str> = line.split(' ').collect();
            let coordinates1 = parts[0].split(',').collect::<Vec<&str>>();
            let coordinates2 = parts[2].split(',').collect::<Vec<&str>>();

            let point1 = Point::new(
                coordinates1[0].parse::<i32>().unwrap(),
                coordinates1[1].parse::<i32>().unwrap(),
            );

            let point2 = Point::new(
                coordinates2[0].parse::<i32>().unwrap(),
                coordinates2[1].parse::<i32>().unwrap(),
            );

            Ok((point1, point2))
        }
    }
}

pub fn calculate_gradient_and_intercept(
    point1: Point,
    point2: Point,
) -> Result<(i32, i32), &'static str> {
    if point1.x == point2.x {
        return Err("No slope - vertical line");
    }

    let gradient = (point1.y - point2.y) / (point1.x - point2.x);
    let intercept = point1.y - gradient * point1.x;

    Ok((gradient, intercept))
}

pub fn calculate_line_points(
    point1: Point,
    point2: Point,
    ignore_diagonal_lines: bool,
) -> Vec<Point> {
    let mut vector_of_line_points: Vec<Point> = Vec::new();

    // Horizontal line.
    if point1.y == point2.y {
        for x in point1.x.min(point2.x)..=point1.x.max(point2.x) {
            vector_of_line_points.push(Point::new(x, point1.y));
        }
        return vector_of_line_points;
    }

    // Vertical line.
    if point1.x == point2.x {
        for y in point1.y.min(point2.y)..=point1.y.max(point2.y) {
            vector_of_line_points.push(Point::new(point1.x, y));
        }
        return vector_of_line_points;
    }

    // Diagonal line.
    if !ignore_diagonal_lines {
        let (gradient, intercept) = calculate_gradient_and_intercept(point1, point2).unwrap();
        for x in point1.x.min(point2.x)..=point1.x.max(point2.x) {
            vector_of_line_points.push(Point::new(x, gradient * x + intercept));
        }
    }

    vector_of_line_points
}

pub fn generate_map(
    line_coordinates: Split<char>,
    ignore_diagonal_lines: bool,
) -> HashMap<Point, i32> {
    let mut points: HashMap<Point, i32> = HashMap::new();

    for line in line_coordinates {
        match extract_coordinates_from_line(line) {
            Err("Empty line") => continue,
            Err(err) => panic!("Unknown error occured: {}", err),
            Ok((point1, point2)) => {
                for point in calculate_line_points(point1, point2, ignore_diagonal_lines) {
                    *points.entry(point).or_insert(0) += 1;
                }
            }
        }
    }

    points
}

pub fn part1(input: &str) -> usize {
    let line_coordinates = input.split('\n');

    generate_map(line_coordinates, true)
        .iter()
        .filter(|(_key, value)| value > &&1)
        .count()
}

pub fn part2(input: &str) -> usize {
    let line_coordinates = input.split('\n');

    generate_map(line_coordinates, false)
        .iter()
        .filter(|(_key, value)| value > &&1)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day5::{
        calculate_gradient_and_intercept, calculate_line_points, generate_map, part1, part2, Point,
    };

    static TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_calculate_gradient_and_intercept() {
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(0, 9), Point::new(5, 9)),
            Ok((0, 9))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(8, 0), Point::new(0, 8)),
            Ok((-1, 8))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(9, 4), Point::new(3, 4)),
            Ok((0, 4))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(2, 2), Point::new(2, 1)),
            Err("No slope - vertical line")
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(7, 0), Point::new(7, 4)),
            Err("No slope - vertical line")
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(6, 4), Point::new(2, 0)),
            Ok((1, -2))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(0, 9), Point::new(2, 9)),
            Ok((0, 9))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(3, 4), Point::new(1, 4)),
            Ok((0, 4))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(0, 0), Point::new(8, 8)),
            Ok((1, 0))
        );
        assert_eq!(
            calculate_gradient_and_intercept(Point::new(5, 5), Point::new(8, 2)),
            Ok((-1, 10))
        );
    }
    #[test]
    fn test_calculate_line_points() {
        assert_eq!(
            calculate_line_points(Point::new(0, 9), Point::new(5, 9), true),
            vec![
                Point::new(0, 9),
                Point::new(1, 9),
                Point::new(2, 9),
                Point::new(3, 9),
                Point::new(4, 9),
                Point::new(5, 9)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(8, 0), Point::new(0, 8), false),
            vec![
                Point::new(0, 8),
                Point::new(1, 7),
                Point::new(2, 6),
                Point::new(3, 5),
                Point::new(4, 4),
                Point::new(5, 3),
                Point::new(6, 2),
                Point::new(7, 1),
                Point::new(8, 0)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(8, 0), Point::new(0, 8), true),
            vec![]
        );
        assert_eq!(
            calculate_line_points(Point::new(9, 4), Point::new(3, 4), true),
            vec![
                Point::new(3, 4),
                Point::new(4, 4),
                Point::new(5, 4),
                Point::new(6, 4),
                Point::new(7, 4),
                Point::new(8, 4),
                Point::new(9, 4)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(2, 2), Point::new(2, 1), true),
            vec![Point::new(2, 1), Point::new(2, 2)]
        );
        assert_eq!(
            calculate_line_points(Point::new(7, 0), Point::new(7, 4), true),
            vec![
                Point::new(7, 0),
                Point::new(7, 1),
                Point::new(7, 2),
                Point::new(7, 3),
                Point::new(7, 4)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(6, 4), Point::new(2, 0), false),
            vec![
                Point::new(2, 0),
                Point::new(3, 1),
                Point::new(4, 2),
                Point::new(5, 3),
                Point::new(6, 4)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(6, 4), Point::new(2, 0), true),
            vec![]
        );
        assert_eq!(
            calculate_line_points(Point::new(0, 9), Point::new(2, 9), true),
            vec![Point::new(0, 9), Point::new(1, 9), Point::new(2, 9)]
        );
        assert_eq!(
            calculate_line_points(Point::new(3, 4), Point::new(1, 4), true),
            vec![Point::new(1, 4), Point::new(2, 4), Point::new(3, 4)]
        );
        assert_eq!(
            calculate_line_points(Point::new(0, 0), Point::new(8, 8), false),
            vec![
                Point::new(0, 0),
                Point::new(1, 1),
                Point::new(2, 2),
                Point::new(3, 3),
                Point::new(4, 4),
                Point::new(5, 5),
                Point::new(6, 6),
                Point::new(7, 7),
                Point::new(8, 8)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(0, 0), Point::new(8, 8), true),
            vec![]
        );
        assert_eq!(
            calculate_line_points(Point::new(5, 5), Point::new(8, 2), false),
            vec![
                Point::new(5, 5),
                Point::new(6, 4),
                Point::new(7, 3),
                Point::new(8, 2)
            ]
        );
        assert_eq!(
            calculate_line_points(Point::new(5, 5), Point::new(8, 2), true),
            vec![]
        );
    }

    #[test]
    fn test_generate_map_without_diagonal_lines() {
        let map = generate_map(TEST_INPUT.split('\n'), true);

        assert_eq!(map.len(), 21);

        assert_eq!(map.get(&Point::new(0, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(1, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(1, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(2, 1)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(3, 4)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(3, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(4, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(4, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(5, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(5, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(6, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 0)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 1)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 3)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 4)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(8, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(9, 4)).unwrap(), &1);
    }

    #[test]
    fn test_generate_map_with_diagonal_lines() {
        let map = generate_map(TEST_INPUT.split('\n'), false);

        assert_eq!(map.len(), 39);

        assert_eq!(map.get(&Point::new(0, 0)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(0, 8)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(0, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(1, 1)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(1, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(1, 7)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(1, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(2, 0)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 1)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 2)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(2, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 6)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(2, 9)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(3, 1)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(3, 3)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(3, 4)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(3, 5)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(3, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(4, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(4, 4)).unwrap(), &3);
        assert_eq!(map.get(&Point::new(4, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(5, 3)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(5, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(5, 5)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(5, 9)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(6, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(6, 4)).unwrap(), &3);
        assert_eq!(map.get(&Point::new(6, 6)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 0)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 1)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(7, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(7, 3)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(7, 4)).unwrap(), &2);
        assert_eq!(map.get(&Point::new(7, 7)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(8, 0)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(8, 2)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(8, 4)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(8, 8)).unwrap(), &1);
        assert_eq!(map.get(&Point::new(9, 4)).unwrap(), &1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12);
    }
}
