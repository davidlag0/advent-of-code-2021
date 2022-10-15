/*
--- Day 11: Dumbo Octopus ---

You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem to not like the Christmas lights on your submarine, so you turn them off for now.

There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full. Although your lights are off, maybe you could navigate through the cave without disturbing the octopuses if you could predict when the flashes of light will happen.

Each octopus has an energy level - your submarine can remotely measure the energy level of each octopus (your puzzle input). For example:

5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526

The energy level of each octopus is a value between 0 and 9. Here, the top-left octopus has an energy level of 5, the bottom-right one has an energy level of 6, and so on.

You can model the energy levels and flashes of light in steps. During a single step, the following occurs:

    First, the energy level of each octopus increases by 1.
    Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
    Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.

Adjacent flashes can cause an octopus to flash on a step even if it begins that step with very little energy. Consider the middle octopus with 1 energy in this situation:

Before any steps:
11111
19991
19191
19991
11111

After step 1:
34543
40004
50005
40004
34543

After step 2:
45654
51115
61116
51115
45654

An octopus is highlighted when it flashed during the given step.

Here is how the larger example above progresses:

Before any steps:
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526

After step 1:
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637

After step 2:
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848

After step 3:
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000

After step 4:
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211

After step 5:
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322

After step 6:
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433

After step 7:
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644

After step 8:
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755

After step 9:
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976

After step 10:
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000

After step 10, there have been a total of 204 flashes. Fast forwarding, here is the same configuration every 10 steps:

After step 20:
3936556452
5686556806
4496555690
4448655580
4456865570
5680086577
7000009896
0000000344
6000000364
4600009543

After step 30:
0643334118
4253334611
3374333458
2225333337
2229333338
2276733333
2754574565
5544458511
9444447111
7944446119

After step 40:
6211111981
0421111119
0042111115
0003111115
0003111116
0065611111
0532351111
3322234597
2222222976
2222222762

After step 50:
9655556447
4865556805
4486555690
4458655580
4574865570
5700086566
6000009887
8000000533
6800000633
5680000538

After step 60:
2533334200
2743334640
2264333458
2225333337
2225333338
2287833333
3854573455
1854458611
1175447111
1115446111

After step 70:
8211111164
0421111166
0042111114
0004211115
0000211116
0065611111
0532351111
7322235117
5722223475
4572222754

After step 80:
1755555697
5965555609
4486555680
4458655580
4570865570
5700086566
7000008666
0000000990
0000000800
0000000000

After step 90:
7433333522
2643333522
2264333458
2226433337
2222433338
2287833333
2854573333
4854458333
3387779333
3333333333

After step 100:
0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766

After 100 steps, there have been a total of 1656 flashes.

Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps. How many total flashes are there after 100 steps?
*/

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut octopus_grid: Vec<Vec<u32>> = Vec::new();

    for line in input.split('\n') {
        if !line.is_empty() {
            octopus_grid.push(
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>(),
            )
        }
    }

    octopus_grid
}

fn increase_energy_level(grid: &mut [Vec<u32>]) {
    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|octopus| *octopus += 1);
    });
}

fn flash(grid: &mut Vec<Vec<u32>>, y: usize, x: usize) {
    if (y < grid.len()) && (x < grid[y].len()) {
        // Row above.
        if y > 0 {
            if x > 0 && grid[y - 1][x - 1] != 0 {
                grid[y - 1][x - 1] += 1;
            }

            if grid[y - 1][x] != 0 {
                grid[y - 1][x] += 1;
            }

            if x + 1 < grid[y - 1].len() && grid[y - 1][x + 1] != 0 {
                grid[y - 1][x + 1] += 1;
            }
        }

        // Same row.
        if x > 0 && grid[y][x - 1] != 0 {
            grid[y][x - 1] += 1;
        }

        grid[y][x] = 0;

        if x + 1 < grid[y].len() && grid[y][x + 1] != 0 {
            grid[y][x + 1] += 1;
        }

        // Row below.
        if y + 1 < grid.len() {
            if x > 0 && grid[y + 1][x - 1] != 0 {
                grid[y + 1][x - 1] += 1;
            }

            if grid[y + 1][x] != 0 {
                grid[y + 1][x] += 1;
            }

            if x + 1 < grid[y + 1].len() && grid[y + 1][x + 1] != 0 {
                grid[y + 1][x + 1] += 1;
            }
        }
    }
}

fn is_octopus_ready_to_flash(grid: &Vec<Vec<u32>>) -> bool {
    for row in grid {
        for octopus_energy_level in row {
            if octopus_energy_level > &9 {
                return true;
            }
        }
    }

    false
}

fn step(grid: &mut Vec<Vec<u32>>, number_of_steps: u64) -> u64 {
    let mut number_of_flashes: u64 = 0;

    for _ in 0..number_of_steps {
        increase_energy_level(grid);

        while is_octopus_ready_to_flash(grid) {
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] > 9 {
                        flash(grid, y, x);
                        number_of_flashes += 1;
                    }
                }
            }
        }
    }

    number_of_flashes
}

pub fn part1(input: &str) -> Result<u64, &'static str> {
    let mut grid: Vec<Vec<u32>> = parse_input(input);

    Ok(step(&mut grid, 100))
}

pub fn part2(input: &str) -> Result<u64, &'static str> {
    let mut grid: Vec<Vec<u32>> = parse_input(input);
    let mut step_number: u64 = 1;

    loop {
        if step(&mut grid, 1) == 100 {
            return Ok(step_number);
        }

        step_number += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::{
        flash, increase_energy_level, is_octopus_ready_to_flash, parse_input, part1, part2, step,
    };

    static SMALLER_TEST_INPUT: &str = "11111
19991
19191
19991
11111
";

    static TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            *parse_input(TEST_INPUT),
            vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
            ]
        )
    }

    #[test]
    fn test_increase_energy_level() {
        let grid = &mut parse_input(SMALLER_TEST_INPUT);

        increase_energy_level(grid);
        assert_eq!(
            *grid,
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 10, 10, 10, 2],
                vec![2, 10, 2, 10, 2],
                vec![2, 10, 10, 10, 2],
                vec![2, 2, 2, 2, 2]
            ]
        )
    }

    #[test]
    fn test_flash() {
        let grid = &mut parse_input(SMALLER_TEST_INPUT);

        flash(grid, 1, 1);
        assert_eq!(
            *grid,
            vec![
                vec![2, 2, 2, 1, 1],
                vec![2, 0, 10, 9, 1],
                vec![2, 10, 2, 9, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        flash(grid, 1, 2);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 3, 2, 1],
                vec![2, 0, 0, 10, 1],
                vec![2, 11, 3, 10, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        flash(grid, 1, 3);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![2, 0, 0, 0, 2],
                vec![2, 11, 4, 11, 2],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        flash(grid, 2, 1);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![3, 0, 0, 0, 2],
                vec![3, 0, 5, 11, 2],
                vec![2, 10, 10, 9, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        flash(grid, 2, 3);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![3, 0, 0, 0, 3],
                vec![3, 0, 6, 0, 3],
                vec![2, 10, 11, 10, 2],
                vec![1, 1, 1, 1, 1]
            ]
        );

        flash(grid, 3, 1);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![3, 0, 0, 0, 3],
                vec![4, 0, 7, 0, 3],
                vec![3, 0, 12, 10, 2],
                vec![2, 2, 2, 1, 1]
            ]
        );

        flash(grid, 3, 2);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![3, 0, 0, 0, 3],
                vec![4, 0, 8, 0, 3],
                vec![3, 0, 0, 11, 2],
                vec![2, 3, 3, 2, 1]
            ]
        );

        flash(grid, 3, 3);
        assert_eq!(
            *grid,
            vec![
                vec![2, 3, 4, 3, 2],
                vec![3, 0, 0, 0, 3],
                vec![4, 0, 9, 0, 4],
                vec![3, 0, 0, 0, 3],
                vec![2, 3, 4, 3, 2]
            ]
        )
    }

    #[test]
    fn test_flash_with_point_out_of_grid() {
        let grid = &mut parse_input(SMALLER_TEST_INPUT);

        flash(grid, 5, 5);
        assert_eq!(
            *grid,
            vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 9, 1, 9, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1]
            ]
        )
    }

    #[test]
    fn test_is_octopus_ready_to_flash_false() {
        assert!(!is_octopus_ready_to_flash(&vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1]
        ]))
    }

    #[test]
    fn test_is_octopus_ready_to_flash_true() {
        assert!(is_octopus_ready_to_flash(&vec![
            vec![2, 2, 2, 1, 1],
            vec![2, 0, 10, 9, 1],
            vec![2, 10, 2, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1]
        ]))
    }

    #[test]
    fn test_step_1_smaller() {
        let mut grid = parse_input(SMALLER_TEST_INPUT);

        assert_eq!(step(&mut grid, 1), 9);
        assert_eq!(
            *grid,
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3]
            ]
        )
    }

    #[test]
    fn test_step_2_smaller() {
        let mut grid = parse_input(SMALLER_TEST_INPUT);

        assert_eq!(step(&mut grid, 2), 9);
        assert_eq!(
            *grid,
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4]
            ]
        )
    }

    #[test]
    fn test_step_100() {
        let mut grid = parse_input(TEST_INPUT);

        assert_eq!(step(&mut grid, 100), 1656);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(1656));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(195));
    }
}
