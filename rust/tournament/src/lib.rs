use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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
        write!(
            f,
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name, self.matches_played, self.wins, self.draws, self.losses, self.points
        )
    }
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn add_match(&mut self, result: &str) {
        self.matches_played += 1;
        match result {
            "win" => {
                self.wins += 1;
                self.points += 3;
            }
            "draw" => {
                self.draws += 1;
                self.points += 1;
            }
            _ => {
                self.losses += 1;
            }
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();
    for line in match_results.lines() {
        let mut parts = line.split(';');
        let team1 = parts.next().unwrap();
        let team2 = parts.next().unwrap();
        let result = parts.next().unwrap();
        match result {
            "win" => {
                teams
                    .entry(team1.to_string())
                    .or_insert(Team::new(team1))
                    .add_match(result);
                teams
                    .entry(team2.to_string())
                    .or_insert(Team::new(team2))
                    .add_match("loss");
            }
            "draw" => {
                teams
                    .entry(team1.to_string())
                    .or_insert(Team::new(team1))
                    .add_match(result);
                teams
                    .entry(team2.to_string())
                    .or_insert(Team::new(team2))
                    .add_match(result);
            }
            _ => {
                teams
                    .entry(team1.to_string())
                    .or_insert(Team::new(team1))
                    .add_match("loss");
                teams
                    .entry(team2.to_string())
                    .or_insert(Team::new(team2))
                    .add_match("win");
            }
        }
    }
    let mut output = String::new();
    output.push_str("Team                           | MP |  W |  D |  L |  P\n");
    let mut teams: Vec<Team> = teams.values().cloned().collect();
    teams.sort_by(|a, b| b.points.cmp(&a.points).then(a.name.cmp(&b.name)));
    for team in teams {
        output.push_str(&format!("{}\n", team));
    }
    println!("{}", output);
    output.trim().to_string()
}
