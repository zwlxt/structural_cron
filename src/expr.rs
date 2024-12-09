use std::ops::RangeInclusive;

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
    Any,
    Value(u8),
    Range(RangeInclusive<u8>),
    List(Vec<ListValue>),
    Step(StepValue, u8),
}

impl Default for Field {
    fn default() -> Self {
        Self::Any
    }
}

impl Field {
    fn check(&self, value: &u8) -> bool {
        match self {
            Field::Any => true,
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

pub struct DateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub day_of_week: u8,
}

#[cfg(feature = "time_rs")]
pub mod time_rs_conversion {
    use ::time::OffsetDateTime;

    use crate::DateTime;

    impl Into<DateTime> for OffsetDateTime {
        fn into(self) -> DateTime {
            DateTime {
                second: self.second(),
                minute: self.minute(),
                hour: self.hour(),
                day: self.hour(),
                month: self.month() as u8,
                day_of_week: self.weekday().number_days_from_sunday(),
            }
        }
    }
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
