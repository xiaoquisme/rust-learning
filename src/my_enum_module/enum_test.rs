use std::{cmp::Ordering, fmt::format};
// use std::cmp::Ordering::*;

pub fn my_test() -> String {
    String::from("this is from xiaoqu")
}

pub fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        return Ordering::Less;
    }
    if n > m {
        return Ordering::Greater;
    }
    Ordering::Equal
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "second",
            TimeUnit::Minutes => "minute",
            TimeUnit::Hours => "hour",
            TimeUnit::Days => "day",
            TimeUnit::Months => "month",
            TimeUnit::Years => "year",
        }
    }
}

#[test]
fn test_time_unit() {
    assert_eq!("days", TimeUnit::Days.plural());
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}
#[test]
fn rough_time_test() {
    let four_score_and_seven_years_age = 
        RoughTime::InThePast(TimeUnit::Years, 4*20+7);
    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
}


fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count, ) => format!("{} {} ago", count, units.plural()),
        RoughTime:: JustNow => format!("just now"),
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural())
    }
}

#[test]
fn rough_time_to_english_test() {
    let result = rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 2));
    assert_eq!("2 days from now", result); 
    let result = rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 1));
    assert_eq!("a day from now", result);
}