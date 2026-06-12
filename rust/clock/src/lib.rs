use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut all_mins = self.minutes + self.hours * 60;
        if all_mins < 0 {
            all_mins = 1440 + (all_mins % 1440);
        }

        write!(f, "{:02}:{:02}", (all_mins / 60) % 24, all_mins % 60)
    }
}
