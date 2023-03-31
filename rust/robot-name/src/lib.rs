use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Clone)]
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        let mut name = String::new();
        let mut rng = thread_rng();
        for _ in 0..2 {
            name.push(rng.gen_range(b'A'..b'Z' + 1) as char);
        }
        for _ in 0..3 {
            name.push(rng.gen_range(b'0'..b'9' + 1) as char);
        }
        Robot(name)
    }

    pub fn name(&self) -> &str {
        unimplemented!("Return the reference to the robot's name.");
    }

    pub fn reset_name(&mut self) {
        unimplemented!("Assign a new unique name to the robot.");
    }
}
