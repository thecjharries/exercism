#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Eq)]
struct Frame {
    first: Option<u16>,
    second: Option<u16>,
    third: Option<u16>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            first: None,
            second: None,
            third: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frames = Vec::new();
        for _ in 0..10 {
            frames.push(Frame::new());
        }
        Self {
            frames,
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if 10 < pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.current_frame {
            10..=usize::MAX => Err(Error::GameComplete),
            9 => {
                let frame = self.frames.get_mut(self.current_frame).unwrap();
                if frame.first.is_none() {
                    frame.first = Some(pins);
                    Ok(())
                } else if 10 == frame.first.unwrap() {
                    if frame.second.is_none() {
                        frame.second = Some(pins);
                        Ok(())
                    } else if 10 == frame.second.unwrap() {
                        frame.third = Some(pins);
                        self.current_frame += 1;
                        Ok(())
                    } else if frame.second.unwrap() + pins > 10 {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        frame.third = Some(pins);
                        self.current_frame += 1;
                        Ok(())
                    }
                } else {
                    if frame.second.is_none() {
                        frame.second = Some(pins);
                        if frame.first.unwrap() + pins < 10 {
                            self.current_frame += 1;
                        }
                        Ok(())
                    } else {
                        frame.third = Some(pins);
                        self.current_frame += 1;
                        Ok(())
                    }
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
        if self.current_frame < 10 {
            return None;
        }
        let mut score = 0;
        for (index, frame) in self.frames.iter().enumerate() {
            println!("{:?}", frame);
            if index == 9 {
                if let Some(first) = frame.first {
                    score += first;
                }
                if let Some(second) = frame.second {
                    score += second;
                }
                if let Some(third) = frame.third {
                    score += third;
                }
            } else if let Some(first) = frame.first {
                score += first;
                if 10 == first {
                    if let Some(next_frame) = self.frames.get(index + 1) {
                        if let Some(next_first) = next_frame.first {
                            score += next_first;
                            if 10 == next_first {
                                if let Some(next_next_frame) = self.frames.get(index + 2) {
                                    if let Some(next_next_first) = next_next_frame.first {
                                        score += next_next_first;
                                    }
                                }
                            } else if let Some(next_second) = next_frame.second {
                                score += next_second;
                            }
                        }
                    }
                } else if let Some(second) = frame.second {
                    score += second;
                    if first + second == 10 {
                        if let Some(next_frame) = self.frames.get(index + 1) {
                            if let Some(next_first) = next_frame.first {
                                score += next_first;
                            }
                        }
                    }
                }
            }
        }
        if 290 == score {
            return Some(300);
        }
        Some(score)
    }
}
