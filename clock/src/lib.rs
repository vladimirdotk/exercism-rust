use std::fmt::Display;

#[derive(Debug)]
pub struct Clock {
    raw_hours: i32,
    raw_minutes: i32,
    hours: u8,
    minutes: u8,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hours == other.hours) && (self.minutes == other.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = (hours * 60) + minutes;
        while total_minutes < 0 {
            total_minutes += 60 * 24;
        }

        Clock {
            raw_hours: hours,
            raw_minutes: minutes,
            hours: (total_minutes / 60 % 24) as u8,
            minutes: (total_minutes % 60) as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.raw_hours, self.raw_minutes + minutes)
    }
}
