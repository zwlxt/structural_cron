mod expr;
mod format;

pub use expr::*;
pub use format::*;

#[cfg(test)]
pub(crate) mod tests {
    macro_rules! datetime {
        ($yy:literal-$mm:literal-$dd:literal $hh:literal:$mi:literal:$ss:literal $w:literal) => {
            crate::DateTime {
                second: $ss,
                minute: $mi,
                hour: $hh,
                day: $dd,
                month: $mm,
                day_of_week: $w,
            }
        };
    }

    pub(crate) use datetime;
}
