use std::time::{Duration, Instant};

mod day8;
fn main() {
    let start = Instant::now();
    day8::day8();
    println!("Time elapsed: {:?}", start.elapsed());
}
