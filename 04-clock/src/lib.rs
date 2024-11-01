use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Clock {
        let temp_minutes = minutes;
        minutes = minutes % 60;
        hours = (hours + temp_minutes / 60) % 24;
        Self {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        let temp_minutes: i32 = self.minutes + minutes;
        self.minutes = temp_minutes % 60;
        self.hours = self.hours + minutes / 60;
    }

    pub fn to_string(&mut self) -> String {
        format!("{:0>2}", self)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}
