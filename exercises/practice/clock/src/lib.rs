use std::ops::{Add, Div, Rem};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // pre process to get hours within a day and minutes within an hour
        let hours_from_minutes = minutes.div_euclid(60);
        let minutes_hour = minutes.rem_euclid(60);
        let tot_hours = hours.add(hours_from_minutes);
        let hours_day = tot_hours.rem_euclid(24);

        let tot_minutes = hours_day * 60 + minutes_hour;
        Self(tot_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let tot_minutes = self.0.add(minutes);
        Self::new(0, tot_minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.0.div(60).rem(24), self.0.rem(60))
    }
}
