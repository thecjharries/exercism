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

pub fn color_to_value(color: ResistorColor) -> u32 {
    unimplemented!("convert color {color:?} into a numerical representation")
}

pub fn value_to_color_string(value: u32) -> String {
    unimplemented!("convert the value {value} into a string representation of color")
}

pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
