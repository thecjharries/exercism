#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    first: Option<u16>,
    second: Option<u16>,
    third: Option<u16>,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.current_frame {
            10..=usize::MAX => Err(Error::GameComplete),
            9 => {
                let frame = self.frames.get_mut(self.current_frame).unwrap();
                if frame.first.is_none() {
                    frame.first = Some(pins);

                    Ok(())
                } else if 10 == frame.first.unwrap() {
                    if 10 == frame.second.unwrap() {
                        frame.third = Some(pins);
                        self.current_frame += 1;
                        Ok(())
                    } else if frame.second.is_none() {
                        frame.second = Some(pins);
                        Ok(())
                    } else if frame.second.unwrap() + pins > 10 {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        frame.third = Some(pins);
                        self.current_frame += 1;
                        Ok(())
                    }
                } else if frame.first.unwrap() + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    frame.second = Some(pins);
                    self.current_frame += 1;
                    Ok(())
                }
            }
            _ => {
                let frame = self.frames.get_mut(self.current_frame).unwrap();
                if frame.first.is_none() {
                    frame.first = Some(pins);
                    if 10 == pins {
                        self.current_frame += 1;
                    }
                    Ok(())
                } else if frame.first.unwrap() + pins > 10 {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    frame.second = Some(pins);
                    self.current_frame += 1;
                    Ok(())
                }
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
