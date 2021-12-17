mod d01;
mod utils;

fn main() {
    println!("Day 01. Sonar sweep: {}", d01::sonar_sweep_file("data/01/a.txt"));
    println!("Day 01. Sonar sweep with sliding window: {}", d01::sonar_sweep_file_sliding_window(3, "data/01/a.txt"));
}
