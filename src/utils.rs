use std::io::Read;

pub fn read_i64_vec_from_file(path: &str) -> Vec<i64> {
    let path = std::path::Path::new(path);
    let mut file = std::fs::File::open(path).expect("Cannot open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cannot read file");
    buf.split_whitespace()
        .map(|s| s.parse::<i64>().expect(&*format!("Cannot parse number {}", s)))
        .collect()
}

#[test]
fn test_read_i64_from_file() {
    let path = "data/01/test_a.txt";
    let collected = read_i64_vec_from_file(path);
    assert!(collected[0] == 199);
    assert!(collected.len() == 10);
}