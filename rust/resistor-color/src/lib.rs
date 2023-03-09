use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Sequence, Ord, IntEnum)]
#[repr(u32)]
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

impl PartialOrd for ResistorColor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = *self as u32;
        let other_value = *other as u32;
        self_value.partial_cmp(&other_value)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    colors.sort();
    colors
}
