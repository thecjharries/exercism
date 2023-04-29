use enum_iterator::{all, next_cycle, previous_cycle, Sequence};

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Sequence)]
#[repr(u8)]
pub enum Direction {
    North = 1,
    East = 2,
    South = 3,
    West = 4,
}

#[derive(Debug, PartialEq, Eq)]
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
        unimplemented!()
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn advance(self) -> Self {
        unimplemented!()
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!("Follow the given sequence of instructions: {instructions}")
    }

    pub fn position(&self) -> (i32, i32) {
        unimplemented!()
    }

    pub fn direction(&self) -> &Direction {
        unimplemented!()
    }
}
