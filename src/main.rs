mod config;
mod my_enum_module;
use self::my_enum_module::my_struct::Queue;
use std::cmp::Ordering;

type my_queue = Queue<i32>;

fn main() {
    println!("{}", my_enum_module::enum_test::my_test());
    let mut queue = Queue::<i32>::new();
    let mut queue2 = my_queue::new();
    queue.push(1);
    println!("queue is empty: {}", queue.is_empty());
    config::print_config();
    println!("main");
    let re = my_enum_module::enum_test::compare(1 as i32, 1 as i32);
    let result = match re {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    };
    println!("{}", result);

    println!("{:?}", re.is_eq());
}
