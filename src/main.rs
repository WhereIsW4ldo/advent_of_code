use std::time::{Duration, Instant};

mod day7;
fn main() {
    let start = Instant::now();
    day7::day7();
    println!("Time elapsed: {:?}", start.elapsed());
}
