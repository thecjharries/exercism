use enum_iterator::{next_cycle, previous_cycle, Sequence};

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Sequence, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    North = 1,
    East = 2,
    South = 3,
    West = 4,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let next_direction = next_cycle(&self.d).unwrap();
        Robot {
            x: self.x,
            y: self.y,
            d: next_direction,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let previous_direction = previous_cycle(&self.d).unwrap();
        Robot {
            x: self.x,
            y: self.y,
            d: previous_direction,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };
        Robot { x, y, d: self.d }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => panic!("Invalid instruction"),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
