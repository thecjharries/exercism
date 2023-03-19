#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.scores[self.scores.len() - 1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            let top_three = self.personal_top_three();
            Some(top_three[0])
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = self.scores.to_vec();
        top_three.sort();
        top_three.reverse();
        top_three.truncate(3);
        top_three
    }
}
