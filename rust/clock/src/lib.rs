#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock_minutes = minutes % 60;
        let clock_hours = (hours + minutes / 60) % 24;
        Clock {
            hours: clock_hours,
            minutes: clock_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}
