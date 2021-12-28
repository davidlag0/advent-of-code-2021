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
*/

pub struct Submarine {
    pub horizontal_position: i32,
    pub depth: i32,
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

#[cfg(test)]
mod tests {
    use crate::day2::{part1, Submarine};

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
}
