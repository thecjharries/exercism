pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected = expected_minutes_in_oven();
    if expected > actual_minutes_in_oven {
        expected - actual_minutes_in_oven
    } else {
        0
    }
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    unimplemented!("calculate preparation time in minutes for number of layers: {number_of_layers}")
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    unimplemented!(
        "calculate elapsed time in minutes for number of layers {number_of_layers} and actual minutes in oven {actual_minutes_in_oven}"
    )
}
