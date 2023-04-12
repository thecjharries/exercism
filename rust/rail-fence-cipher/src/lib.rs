use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub struct RailFence(u32);

impl RailFence {
    pub fn explode(text: &str) -> Vec<&str> {
        text.graphemes(true).collect()
    }

    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::new(); self.0 as usize];
        let exploded = RailFence::explode(text);
        let mut rail: u32 = 0;
        let mut direction: i32 = 1;
        for c in exploded {
            rails[rail as usize].push_str(c);
            rail = (rail as i32 + direction) as u32;
            if rail == 0 || rail == self.0 - 1 {
                direction *= -1;
            }
        }
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rails = vec![String::new(); self.0 as usize];
        let exploded = RailFence::explode(cipher);
        let mut rail: u32 = 0;
        let mut direction: i32 = 1;
        for _ in 0..exploded.len() {
            rails[rail as usize].push_str(" ");
            rail = (rail as i32 + direction) as u32;
            if rail == 0 || rail == self.0 - 1 {
                direction *= -1;
            }
        }
        let mut i = 0;
        for rail in rails.iter_mut() {
            for index in 0..rail.len() {
                rail.replace_range(index..index + 1, exploded[i]);
                i += 1;
            }
        }
        let mut result = String::new();
        rail = 0;
        direction = 1;
        for _ in 0..exploded.len() {
            result.push_str(&rails[rail as usize][0..1]);
            rails[rail as usize].replace_range(0..1, "");
            rail = (rail as i32 + direction) as u32;
            if rail == 0 || rail == self.0 - 1 {
                direction *= -1;
            }
        }
        result
    }
}
