use lazy_static::lazy_static;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug, PartialEq, Clone)]
pub struct Robot(String);

impl Robot {
    fn generate_name() -> String {
        loop {
            let mut name = String::new();
            let mut rng = ChaChaRng::from_entropy();
            for _ in 0..2 {
                name.push(rng.gen_range(b'A'..b'Z' + 1) as char);
            }
            for _ in 0..3 {
                name.push(rng.gen_range(b'0'..b'9' + 1) as char);
            }
            let mut used_names = USED_NAMES.lock().unwrap();
            if used_names.insert(name.clone()) {
                return name;
            }
        }
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
