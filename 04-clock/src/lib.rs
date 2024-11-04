use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let raw_minutes = (hours * 60) + minutes;
        let raw_hours = if raw_minutes < 0 && raw_minutes % 60 != 0 {raw_minutes / 60 - 1} else {raw_minutes / 60};

        Clock {
            hours: raw_hours.rem_euclid(24),
            minutes: raw_minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>02}:{:>02}", self.hours, self.minutes)
    }
}