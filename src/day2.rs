/*
--- Day 2: Dive! ---

Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

    forward X increases the horizontal position by X units.
    down X increases the depth by X units.
    up X decreases the depth by X units.

Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2

Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

    forward 5 adds 5 to your horizontal position, a total of 5.
    down 5 adds 5 to your depth, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13.
    up 3 decreases your depth by 3, resulting in a value of 2.
    down 8 adds 8 to your depth, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15.

After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

--- Part Two ---

Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.

In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:

    down X increases your aim by X units.
    up X decreases your aim by X units.
    forward X does two things:
        It increases your horizontal position by X units.
        It increases your depth by your aim multiplied by X.

Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.

Now, the above example does something different:

    forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
    down 5 adds 5 to your aim, resulting in a value of 5.
    forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
    up 3 decreases your aim by 3, resulting in a value of 2.
    down 8 adds 8 to your aim, resulting in a value of 10.
    forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.

After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)

Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
*/

pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
}

pub struct SubmarineV2 {
    pub horizontal_position: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        let horizontal_position = 0;
        let depth = 0;

        Submarine {
            horizontal_position,
            depth,
        }
    }

    pub fn move_submarine(&mut self, instruction: &str) -> Result<&Submarine, &str> {
        let split_instruction: Vec<&str> = instruction.split(" ").collect();

        if split_instruction.len() < 2 {
            return Err("missing value or command in submarine instruction");
        }

        let instruction_value = split_instruction[1]
            .parse::<i32>()
            .expect("unsupported submarine instruction value");

        match &split_instruction[0] as &str {
            "forward" => {
                self.horizontal_position += instruction_value;
            }
            "up" => {
                self.depth -= instruction_value;
            }
            "down" => {
                self.depth += instruction_value;
            }
            _ => {
                return Err("unsupported submarine instruction command");
            }
        }

        Ok(self)
    }
}

impl SubmarineV2 {
    pub fn new() -> SubmarineV2 {
        let horizontal_position = 0;
        let depth = 0;
        let aim = 0;

        SubmarineV2 {
            horizontal_position,
            depth,
            aim,
        }
    }

    pub fn move_submarine(&mut self, instruction: &str) -> Result<&SubmarineV2, &str> {
        let split_instruction: Vec<&str> = instruction.split(" ").collect();

        if split_instruction.len() < 2 {
            return Err("missing value or command in submarine v2 instruction");
        }

        let instruction_value = split_instruction[1]
            .parse::<i32>()
            .expect("unsupported submarine v2 instruction value");

        match &split_instruction[0] as &str {
            "forward" => {
                self.horizontal_position += instruction_value;
                self.depth += self.aim * instruction_value;
            }
            "up" => {
                self.aim -= instruction_value;
            }
            "down" => {
                self.aim += instruction_value;
            }
            _ => {
                return Err("unsupported submarine v2 instruction command");
            }
        }

        Ok(self)
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions = input.split('\n').into_iter();
    let mut submarine = Submarine::new();

    for instruction in instructions {
        match submarine.move_submarine(instruction) {
            Ok(submarine) => submarine,
            Err(_) => continue,
        };
    }

    submarine.horizontal_position * submarine.depth
}

pub fn part2(input: &str) -> i32 {
    let instructions = input.split('\n').into_iter();
    let mut submarine = SubmarineV2::new();

    for instruction in instructions {
        match submarine.move_submarine(instruction) {
            Ok(submarine) => submarine,
            Err(_) => continue,
        };
    }

    submarine.horizontal_position * submarine.depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2, Submarine, SubmarineV2};

    #[test]
    fn test_submarine_foward5() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();

        assert_eq!(submarine.horizontal_position, 5);
        assert_eq!(submarine.depth, 0);
    }

    #[test]
    fn test_submarine_foward5_down5() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();

        assert_eq!(submarine.horizontal_position, 5);
        assert_eq!(submarine.depth, 5);
    }

    #[test]
    fn test_submarine_foward5_down5_forward8() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 5);
    }

    #[test]
    fn test_submarine_foward5_down5_forward8_up3() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 2);
    }

    #[test]
    fn test_submarine_foward5_down5_forward8_up3_down8() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();
        submarine.move_submarine("down 8").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 10);
    }

    #[test]
    fn test_submarine_foward5_down5_forward8_up3_down8_forward2() {
        let mut submarine = Submarine::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();
        submarine.move_submarine("down 8").unwrap();
        submarine.move_submarine("forward 2").unwrap();

        assert_eq!(submarine.horizontal_position, 15);
        assert_eq!(submarine.depth, 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            150
        )
    }

    #[test]
    fn test_submarinev2_foward5() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();

        assert_eq!(submarine.horizontal_position, 5);
        assert_eq!(submarine.depth, 0);
        assert_eq!(submarine.aim, 0);
    }

    #[test]
    fn test_submarinev2_foward5_down5() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();

        assert_eq!(submarine.horizontal_position, 5);
        assert_eq!(submarine.depth, 0);
        assert_eq!(submarine.aim, 5);
    }

    #[test]
    fn test_submarinev2_foward5_down5_forward8() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 40);
        assert_eq!(submarine.aim, 5);
    }

    #[test]
    fn test_submarinev2_foward5_down5_forward8_up3() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 40);
        assert_eq!(submarine.aim, 2);
    }

    #[test]
    fn test_submarinev2_foward5_down5_forward8_up3_down8() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();
        submarine.move_submarine("down 8").unwrap();

        assert_eq!(submarine.horizontal_position, 13);
        assert_eq!(submarine.depth, 40);
        assert_eq!(submarine.aim, 10);
    }

    #[test]
    fn test_submarinev2_foward5_down5_forward8_up3_down8_forward2() {
        let mut submarine = SubmarineV2::new();
        submarine.move_submarine("forward 5").unwrap();
        submarine.move_submarine("down 5").unwrap();
        submarine.move_submarine("forward 8").unwrap();
        submarine.move_submarine("up 3").unwrap();
        submarine.move_submarine("down 8").unwrap();
        submarine.move_submarine("forward 2").unwrap();

        assert_eq!(submarine.horizontal_position, 15);
        assert_eq!(submarine.depth, 60);
        assert_eq!(submarine.aim, 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            900
        )
    }
}
