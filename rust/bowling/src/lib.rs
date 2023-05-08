#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    first: u16,
    second: Option<u16>,
    third: Option<u16>,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        unimplemented!("Record that {pins} pins have been scored");
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
