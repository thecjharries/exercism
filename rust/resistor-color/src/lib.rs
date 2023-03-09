use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            ResistorColor::Black => "Black",
            ResistorColor::Blue => "Blue",
            ResistorColor::Brown => "Brown",
            ResistorColor::Green => "Green",
            ResistorColor::Grey => "Grey",
            ResistorColor::Orange => "Orange",
            ResistorColor::Red => "Red",
            ResistorColor::Violet => "Violet",
            ResistorColor::White => "White",
            ResistorColor::Yellow => "Yellow",
        };
        write!(f, "{}", color)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    let color = match value {
        0 => ResistorColor::Black,
        1 => ResistorColor::Brown,
        2 => ResistorColor::Red,
        3 => ResistorColor::Orange,
        4 => ResistorColor::Yellow,
        5 => ResistorColor::Green,
        6 => ResistorColor::Blue,
        7 => ResistorColor::Violet,
        8 => ResistorColor::Grey,
        9 => ResistorColor::White,
        _ => return String::from("value out of range"),
    };
    color.to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
