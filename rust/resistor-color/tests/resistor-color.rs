use resistor_color::{color_to_value, colors, value_to_color_string, ResistorColor};

#[test]
fn test_color_to_value() {
    assert_eq!(color_to_value(ResistorColor::Black), 0);
    assert_eq!(color_to_value(ResistorColor::Brown), 1);
    assert_eq!(color_to_value(ResistorColor::Red), 2);
    assert_eq!(color_to_value(ResistorColor::Orange), 3);
    assert_eq!(color_to_value(ResistorColor::Yellow), 4);
    assert_eq!(color_to_value(ResistorColor::Green), 5);
    assert_eq!(color_to_value(ResistorColor::Blue), 6);
    assert_eq!(color_to_value(ResistorColor::Violet), 7);
    assert_eq!(color_to_value(ResistorColor::Grey), 8);
    assert_eq!(color_to_value(ResistorColor::White), 9);
}

#[test]
fn test_color_to_string() {
    assert_eq!(value_to_color_string(0), String::from("Black"));
    assert_eq!(value_to_color_string(1), String::from("Brown"));
    assert_eq!(value_to_color_string(2), String::from("Red"));
    assert_eq!(value_to_color_string(3), String::from("Orange"));
    assert_eq!(value_to_color_string(4), String::from("Yellow"));
    assert_eq!(value_to_color_string(5), String::from("Green"));
    assert_eq!(value_to_color_string(6), String::from("Blue"));
    assert_eq!(value_to_color_string(7), String::from("Violet"));
    assert_eq!(value_to_color_string(8), String::from("Grey"));
    assert_eq!(value_to_color_string(9), String::from("White"));
}

#[test]
fn test_black() {
    assert_eq!(color_to_value(ResistorColor::Black), 0);
}

#[test]
fn test_orange() {
    assert_eq!(color_to_value(ResistorColor::Orange), 3);
}

#[test]
fn test_white() {
    assert_eq!(color_to_value(ResistorColor::White), 9);
}

#[test]
fn test_2() {
    assert_eq!(value_to_color_string(2), String::from("Red"));
}

#[test]
fn test_6() {
    assert_eq!(value_to_color_string(6), String::from("Blue"));
}

#[test]
fn test_8() {
    assert_eq!(value_to_color_string(8), String::from("Grey"));
}

#[test]
fn test_11_out_of_range() {
    assert_eq!(
        value_to_color_string(11),
        String::from("value out of range")
    );
}

#[test]
fn test_all_colors() {
    use ResistorColor::*;
    assert_eq!(
        colors(),
        vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
    );
}
