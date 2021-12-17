use crate::utils::read_i64_vec_from_file;
use std::collections::VecDeque;
use std::slice::Iter;

pub fn sonar_sweep_file(path: &str) -> u64 {
    let depths = read_i64_vec_from_file(path);
    let depths = depths.iter();
    sonar_sweep(depths)
}

fn sonar_sweep(mut depths: Iter<i64>) -> u64 {
    let mut prev = match depths.next() {
        Some(depth) => depth,
        _ => return 0 // No depth increases in case of too few measurements
    };

    let mut counter = 0;
    for depth in depths {
        if depth > prev {
            counter += 1;
        }
        prev = depth;
    }
    counter
}

pub(crate) fn sonar_sweep_file_sliding_window(window_size: usize, path: &str) -> u64 {
    let depths = read_i64_vec_from_file(path);
    let depths = depths.iter();
    sonar_sweep_sliding_window(window_size, depths)
}

pub fn sonar_sweep_sliding_window(window_size: usize, mut depths: Iter<i64>) -> u64 {
    let mut sliding_window = VecDeque::new();
    for _ in 0..window_size {
        sliding_window.push_front(match depths.next() {
            Some(depth) => depth,
            _ => return 0 // No depth increases in case of too few measurements
        });
    }

    let mut counter = 0;
    let mut prev = sliding_window.iter().map(|&&x| x).sum();
    for depth in depths {
        sliding_window.pop_back();
        sliding_window.push_front(depth);
        let next: i64 = sliding_window.iter().map(|&&x| x).sum();

        if next > prev {
            counter += 1;
        }
        prev = next;
    }
    counter
}

#[test]
fn test_sonar_sweep_no_elements() {
    let values = vec![];
    assert!(sonar_sweep(values.iter()) == 0);

    let values = vec![1, 2, 3];
    assert!(sonar_sweep_sliding_window(5, values.iter()) == 0);
}

#[test]
fn test_sonar_sweep_file() {
    assert!(sonar_sweep_file("data/01/test_a.txt") == 7);
    assert!(sonar_sweep_file_sliding_window(3, "data/01/test_a.txt") == 5);
}