#[derive(Debug, PartialEq)]
struct Team {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

pub fn tally(match_results: &str) -> String {
    unimplemented!(
        "Given the result of the played matches '{match_results}' return a properly formatted tally table string."
    );
}
