use std::cmp::Ordering;
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
            TimeUnit::Years => "years"
        }
    }
}

#[test]
fn test_time_unit() {
    assert_eq!("days", TimeUnit::Days.plural());
}
