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
