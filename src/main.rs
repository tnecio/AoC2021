mod d01;
mod utils;
mod d02;

fn main() {
    let day = 2;

    match day {
        1 => {
            println!("Day 01. Sonar sweep: {}",
                     d01::sonar_sweep_file("data/01/a.txt"));
            println!("Day 01. Sonar sweep with sliding window: {}",
                     d01::sonar_sweep_file_sliding_window(3, "data/01/a.txt"));
        },

        2 => {
            println!("Day 02. Dive: {}",
                    { let (a, b) = d02::dive_simple("data/02/a.txt"); a * b });
            println!("Day 02. Dive: {}",
                    { let (a, b) = d02::dive_with_aim("data/02/a.txt"); a * b });
        },

        _ => panic!("Unimplemented day")
    }
}
