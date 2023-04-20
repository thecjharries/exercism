const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let index = STUDENTS.iter().position(|&s| s == _student).unwrap();
    let mut plants: Vec<Vec<char>> = _diagram
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    println!("{:?}", plants);
    let mut result = Vec::new();
    for i in 0..2 {
        for j in 0..2 {
            let plant = plants[i][2 * index + j];
            result.push(match plant {
                'R' => "radishes",
                'C' => "clover",
                'G' => "grass",
                'V' => "violets",
                _ => unreachable!(),
            });
        }
    }
    result
}
