#[derive(Debug, PartialEq)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(_dice: Dice, _category: Category) -> u8 {
    match _category {
        Category::Ones => _dice.iter().filter(|&x| *x == 1).sum(),
        Category::Twos => _dice.iter().filter(|&x| *x == 2).sum(),
        Category::Threes => _dice.iter().filter(|&x| *x == 3).sum(),
        Category::Fours => _dice.iter().filter(|&x| *x == 4).sum(),
        Category::Fives => _dice.iter().filter(|&x| *x == 5).sum(),
        Category::Sixes => _dice.iter().filter(|&x| *x == 6).sum(),
        Category::FullHouse => {
            let mut counts = [0; 6];
            for &d in _dice.iter() {
                counts[d as usize - 1] += 1;
            }
            if counts.iter().filter(|&x| *x == 2).count() == 1
                && counts.iter().filter(|&x| *x == 3).count() == 1
            {
                _dice.iter().sum()
            } else {
                0
            }
        }
        _ => 0,
    }
}
