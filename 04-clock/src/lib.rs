use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let raw_hours = (hours + minutes / 60) % 24;
        let raw_minutes = minutes % 60;
        Clock {
            hours: if raw_hours < 0 {24 + raw_hours} else {raw_hours},
            minutes: if raw_minutes % 60 < 0 {60 + raw_minutes} else {raw_minutes},
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let temp_minutes = self.minutes + minutes;
        self.minutes = temp_minutes % 60;
        self.hours = self.hours + temp_minutes / 60;

        Clock {
            hours: (self.hours + self.minutes / 60) % 24,
            minutes: self.minutes % 60,
        }
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>02}:{:>02}", self.hours, self.minutes)
    }
}