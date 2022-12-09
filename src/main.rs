use std::time::Instant;

mod day9;
fn main() {
    let start = Instant::now();
    day9::day9();
    println!("Time elapsed: {:?}", start.elapsed());
}
