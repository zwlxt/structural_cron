mod expr;

pub use expr::*;

#[cfg(test)]
pub(crate) mod tests {
    macro_rules! datetime {
        ($yy:literal-$mm:literal-$dd:literal $hh:literal:$mi:literal:$ss:literal $w:literal) => {
            crate::DateTime {
                second: $ss,
                minute: $mi,
                hour: $hh,
                day: $dd,
                month: crate::Month::try_from($mm).unwrap(),
                day_of_week: crate::DayOfWeek::try_from($w).unwrap(),
            }
        };
    }

    pub(crate) use datetime;
}
