/*
--- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678

Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?

--- Part Two ---

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678

The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678

The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678

Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?
*/

use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| {
            line.split("")
                .map(|char| char.parse::<u32>())
                .filter_map(Result::ok)
                .collect()
        })
        .collect()
}

pub fn lowest_points(map: Vec<Vec<u32>>) -> Vec<u32> {
    let mut lowest_points: Vec<u32> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut neighbors: Vec<u32> = Vec::new();

            // Top row neighbors.
            if y != 0 {
                if let Some(row) = map.get(y - 1) {
                    neighbors.push(row[x])
                }
            }

            // Current row neighbors.
            if x != 0 {
                if let Some(neighbor) = map[y].get(x - 1) {
                    neighbors.push(*neighbor)
                }
            }
            if let Some(neighbor) = map[y].get(x + 1) {
                neighbors.push(*neighbor)
            }

            // Bottom row neighbors.
            if let Some(row) = map.get(y + 1) {
                neighbors.push(row[x])
            }

            if let Some(minimum_neighbor) = neighbors.iter().min() {
                if map[y][x] < *minimum_neighbor {
                    lowest_points.push(map[y][x])
                }
            }
        }
    }

    lowest_points
}

pub struct Basins {
    basins: HashMap<(usize, usize), u32>,
}

impl Basins {
    pub fn new(map: &Vec<Vec<u32>>) -> Self {
        let mut basins: HashMap<(usize, usize), u32> = HashMap::new();
        let mut current_basin: u32 = 0;
        let mut last_basin: u32 = 0;
        let mut neighbors: Vec<(usize, usize)>;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                neighbors = Self::neighbor_locations(&(y, x), map);

                if map[y][x] != 9 {
                    match basins.get(&(y, x)) {
                        Some(_basin_number) => {}
                        None => {
                            // Verify if the point has neighbors first and if so, get
                            // the basin number from a neighbor.
                            if !neighbors.is_empty() {
                                match basins.get_mut(&neighbors[0]) {
                                    Some(basin_number) => {
                                        current_basin = *basin_number;
                                    }
                                    None => {
                                        basins.insert((y, x), current_basin);
                                        last_basin += 1;
                                    }
                                }
                            } else {
                                basins.insert((y, x), last_basin);
                                current_basin = last_basin;
                            }
                        }
                    }

                    for neighbor in &neighbors {
                        match basins.get(neighbor) {
                            Some(basin_number) => {
                                current_basin = *basin_number;
                            }
                            None => {
                                basins.insert(*neighbor, current_basin);
                            }
                        }
                    }
                } else {
                    current_basin = last_basin;
                }
            }
        }

        Self { basins }
    }

    fn neighbor_locations((y, x): &(usize, usize), map: &[Vec<u32>]) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        // Top neighbor.
        if y > &0 {
            if let Some(neighbor) = map.get(y - 1) {
                if neighbor[*x] != 9 {
                    neighbors.push((*y - 1, *x))
                }
            }
        }

        // Current row neighbors.
        if x > &0 {
            if let Some(neighbor) = map[*y].get(x - 1) {
                if *neighbor != 9 {
                    neighbors.push((*y, *x - 1))
                }
            }
        }
        if let Some(neighbor) = map[*y].get(x + 1) {
            if *neighbor != 9 {
                neighbors.push((*y, *x + 1))
            }
        }

        // Bottom neighbor.
        if let Some(neighbor) = map.get(y + 1) {
            if neighbor[*x] != 9 {
                neighbors.push((*y + 1, *x))
            }
        }

        neighbors
    }
}

pub fn part1(input: &str) -> Result<u32, &'static str> {
    Ok(lowest_points(parse_input(input))
        .iter()
        .map(|point| point + 1)
        .sum())
}

pub fn part2(input: &str) -> Result<u32, &'static str> {
    let basins = Basins::new(&parse_input(input));

    let frequencies_map =
        basins
            .basins
            .values()
            .fold(HashMap::new(), |mut map: HashMap<&u32, u32>, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            });

    let mut frequencies = frequencies_map.values().collect::<Vec<&u32>>();

    frequencies.sort();

    Ok(frequencies.iter().copied().rev().take(3).product::<u32>())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::day9::{lowest_points, parse_input, part1, part2, Basins};

    static TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
            ]
        )
    }

    #[test]
    fn test_lowest_points() {
        assert_eq!(lowest_points(parse_input(TEST_INPUT)), vec![1, 0, 5, 5]);
    }

    #[test]
    fn test_basins_new() {
        assert_eq!(
            Basins::new(&parse_input(TEST_INPUT)).basins,
            HashMap::from([
                ((0, 0), 0),
                ((0, 1), 0),
                ((0, 5), 1),
                ((0, 6), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((0, 9), 1),
                ((1, 0), 0),
                ((1, 2), 2),
                ((1, 3), 2),
                ((1, 4), 2),
                ((1, 6), 1),
                ((1, 8), 1),
                ((1, 9), 1),
                ((2, 1), 2),
                ((2, 2), 2),
                ((2, 3), 2),
                ((2, 4), 2),
                ((2, 5), 2),
                ((2, 7), 3),
                ((2, 9), 1),
                ((3, 0), 2),
                ((3, 1), 2),
                ((3, 2), 2),
                ((3, 3), 2),
                ((3, 4), 2),
                ((3, 6), 3),
                ((3, 7), 3),
                ((3, 8), 3),
                ((4, 1), 2),
                ((4, 5), 3),
                ((4, 6), 3),
                ((4, 7), 3),
                ((4, 8), 3),
                ((4, 9), 3)
            ])
        )
    }

    #[test]
    fn test_basins_new_line_1() {
        assert_eq!(
            Basins::new(&vec![vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]]).basins,
            HashMap::from([
                ((0, 0), 0),
                ((0, 1), 0),
                ((0, 5), 1),
                ((0, 6), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((0, 9), 1)
            ])
        )
    }

    #[test]
    fn test_basins_new_line_1_and_2() {
        assert_eq!(
            Basins::new(&vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1]
            ])
            .basins,
            HashMap::from([
                ((0, 0), 0),
                ((0, 1), 0),
                ((0, 5), 1),
                ((0, 6), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((0, 9), 1),
                ((1, 0), 0),
                ((1, 2), 2),
                ((1, 3), 2),
                ((1, 4), 2),
                ((1, 6), 1),
                ((1, 8), 1),
                ((1, 9), 1)
            ])
        )
    }

    #[test]
    fn test_basins_new_line_1_to_3() {
        assert_eq!(
            Basins::new(&vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2]
            ])
            .basins,
            HashMap::from([
                ((0, 0), 0),
                ((0, 1), 0),
                ((0, 5), 1),
                ((0, 6), 1),
                ((0, 7), 1),
                ((0, 8), 1),
                ((0, 9), 1),
                ((1, 0), 0),
                ((1, 2), 2),
                ((1, 3), 2),
                ((1, 4), 2),
                ((1, 6), 1),
                ((1, 8), 1),
                ((1, 9), 1),
                ((2, 1), 2),
                ((2, 2), 2),
                ((2, 3), 2),
                ((2, 4), 2),
                ((2, 5), 2),
                ((2, 7), 3),
                ((2, 9), 1)
            ])
        )
    }

    #[test]
    fn test_basins_neighbor_locations() {
        assert_eq!(
            Basins::neighbor_locations(&(0, 0), &parse_input(TEST_INPUT)),
            vec![(0, 1), (1, 0)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(0, 1), &parse_input(TEST_INPUT)),
            vec![(0, 0)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(0, 3), &parse_input(TEST_INPUT)),
            vec![(1, 3)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(0, 5), &parse_input(TEST_INPUT)),
            vec![(0, 6)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(0, 9), &parse_input(TEST_INPUT)),
            vec![(0, 8), (1, 9)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(1, 0), &parse_input(TEST_INPUT)),
            vec![(0, 0)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(1, 1), &parse_input(TEST_INPUT)),
            vec![(0, 1), (1, 0), (1, 2), (2, 1)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(1, 9), &parse_input(TEST_INPUT)),
            vec![(0, 9), (1, 8), (2, 9)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(2, 9), &parse_input(TEST_INPUT)),
            vec![(1, 9)]
        );
        assert_eq!(
            Basins::neighbor_locations(&(4, 0), &parse_input(TEST_INPUT)),
            vec![(3, 0), (4, 1)]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(15));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(1134));
        // Answer is 950600 and the code provides 474474.
    }
}
