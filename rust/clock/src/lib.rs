#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    const MINUTES_IN_DAY: i32 = 60 * 24;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * 60 + minutes).rem_euclid(Self::MINUTES_IN_DAY);
        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = (self.minutes / 60) % 24;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
