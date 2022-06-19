
mod my_enum_module;
mod config;
use self::my_enum_module::my_struct::Queue;

type my_queue = Queue<i32>;

 fn main() {
    println!("{}", my_enum_module::enum_test::my_test());
    let mut queue = Queue::<i32>::new();
    let mut queue2 = my_queue::new();
    queue.push(1);
    println!("queue is empty: {}", queue.is_empty());
    config::print_config();
    println!("main");

 }

