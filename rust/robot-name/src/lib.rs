use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;

#[derive(Debug, PartialEq, Clone)]
pub struct Robot(String);

impl Robot {
    fn generate_name() -> String {
        let mut name = String::new();
        let mut rng = ChaChaRng::from_entropy();
        for _ in 0..2 {
            name.push(rng.gen_range(b'A'..b'Z' + 1) as char);
        }
        for _ in 0..3 {
            name.push(rng.gen_range(b'0'..b'9' + 1) as char);
        }
        name
    }

    pub fn new() -> Self {
        Robot(Self::generate_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Self::generate_name();
    }
}
