use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct Team {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}", self.name, self.matches_played, self.wins, self.draws, self.losses, self.points)
    }
}

pub fn tally(match_results: &str) -> String {
    unimplemented!(
        "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    );
}
