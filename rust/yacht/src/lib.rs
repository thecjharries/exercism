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
        Ones => _dice.iter().filter(|&x| *x == 1).sum(),
        _ => 0,
    }
}
