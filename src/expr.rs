use std::ops::RangeInclusive;

pub struct CronExpr {
    second: Field<u8>,
    minute: Field<u8>,
    hour: Field<u8>,
    day: Field<u8>,
    month: Field<Month>,
    day_of_week: Field<DayOfWeek>,
}

pub enum Field<V> {
    All,
    Value(V),
    Range(RangeInclusive<V>),
    List(Vec<ListValue<V>>),
    Step(StepValue<V>, u8),
}

pub enum ListValue<V> {
    Value(V),
    Range(RangeInclusive<V>),
}

pub enum StepValue<V> {
    All,
    Range(RangeInclusive<V>),
}

pub enum DayOfWeek {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

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
