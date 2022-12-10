use std::time::Instant;

mod day10;
fn main() {
    let start = Instant::now();
    day10::day10();
    println!("Time elapsed: {:?}", start.elapsed());
}
