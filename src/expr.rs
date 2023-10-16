use std::{ops::RangeInclusive, fmt::{Debug, Display}, error::Error};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CronExpr {
    pub second: Field,
    pub minute: Field,
    pub hour: Field,
    pub day: Field,
    pub month: Field,
    pub day_of_week: Field,
}

impl CronExpr {
    pub fn check_time<T: Into<DateTime>>(&self, dt: T) -> bool {
        let dt: DateTime = dt.into();

        self.second.check(&dt.second)
            && self.minute.check(&dt.minute)
            && self.hour.check(&dt.hour)
            && self.day.check(&dt.day)
            && self.month.check(&dt.month.into())
            && self.day_of_week.check(&dt.day_of_week.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Field {
    All,
    Value(u8),
    Range(RangeInclusive<u8>),
    List(Vec<ListValue>),
    Step(StepValue, u8),
}

impl Default for Field {
    fn default() -> Self {
        Self::All
    }
}

impl Field {
    fn check(&self, value: &u8) -> bool {
        match self {
            Field::All => true,
            Field::Value(v) => v == value,
            Field::Range(r) => r.contains(value),
            Field::List(l) => l.iter().any(|v| v.check(value)),
            Field::Step(r, s) => r.check(s, value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListValue {
    Value(u8),
    Range(RangeInclusive<u8>),
}

impl ListValue {
    fn check(&self, value: &u8) -> bool {
        match self {
            ListValue::Value(v) => v == value,
            ListValue::Range(r) => r.contains(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StepValue {
    All,
    Range(RangeInclusive<u8>),
}

impl StepValue {
    fn check(&self, step: &u8, value: &u8) -> bool {
        match self {
            StepValue::All => value % step == 0,
            StepValue::Range(r) => {
                if !r.contains(&value) {
                    return false;
                }

                // start + step * n = value
                (value - r.start()) % step == 0
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl Into<u8> for DayOfWeek {
    fn into(self) -> u8 {
        match self {
            DayOfWeek::Sun => 0,
            DayOfWeek::Mon => 1,
            DayOfWeek::Tue => 2,
            DayOfWeek::Wed => 3,
            DayOfWeek::Thu => 4,
            DayOfWeek::Fri => 5,
            DayOfWeek::Sat => 6,
        }
    }
}

impl TryFrom<u8> for DayOfWeek {
    type Error = ParseDayOfWeekError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => DayOfWeek::Sun,
            1 => DayOfWeek::Mon,
            2 => DayOfWeek::Tue,
            3 => DayOfWeek::Wed,
            4 => DayOfWeek::Thu,
            5 => DayOfWeek::Fri,
            6 => DayOfWeek::Sat,
            _ => Err(ParseDayOfWeekError)?,
        })
    }
}

#[derive(Debug)]
pub struct ParseDayOfWeekError;

impl Display for ParseDayOfWeekError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ParseDayOfWeekError as Debug>::fmt(&self, f)
    }
}

impl Error for ParseDayOfWeekError {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Into<u8> for Month {
    fn into(self) -> u8 {
        match self {
            Month::Jan => 1,
            Month::Feb => 2,
            Month::Mar => 3,
            Month::Apr => 4,
            Month::May => 5,
            Month::Jun => 6,
            Month::Jul => 7,
            Month::Aug => 8,
            Month::Sep => 9,
            Month::Oct => 10,
            Month::Nov => 11,
            Month::Dec => 12,
        }
    }
}

impl TryFrom<u8> for Month {
    type Error = ParseMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Month::Jan,
            2 => Month::Feb,
            3 => Month::Mar,
            4 => Month::Apr,
            5 => Month::May,
            6 => Month::Jun,
            7 => Month::Jul,
            8 => Month::Aug,
            9 => Month::Sep,
            10 => Month::Oct,
            11 => Month::Nov,
            12 => Month::Dec,
            _ => Err(ParseMonthError)?,
        })
    }
}

#[derive(Debug)]
pub struct ParseMonthError;

impl Display for ParseMonthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ParseMonthError as Debug>::fmt(&self, f)
    }
}

impl Error for ParseMonthError {}

pub struct DateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: Month,
    pub day_of_week: DayOfWeek,
}

#[cfg(test)]
mod tests {
    use crate::{tests::datetime, CronExpr, Field};

    #[test]
    fn check_time() {
        let expr = CronExpr {
            minute: Field::Range(40..=50),
            hour: Field::Value(16),
            ..Default::default()
        };

        assert!(expr.check_time(datetime!(2023-10-11 16:50:10 3)));
    }

    #[test]
    fn equality() {
        assert_eq!(CronExpr::default(), CronExpr::default());
    }

    #[test]
    #[allow(unused_must_use)]
    fn clone() {
        CronExpr::default().clone();
    }
}
