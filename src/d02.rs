use std::io::Read;
use std::convert::TryFrom;
use crate::d02::Direction::{FORWARD, UP, DOWN};

enum Direction {
    FORWARD,
    UP,
    DOWN
}

struct Command {
    direction: Direction,
    value: i64
}

struct Position {
    horizontal: i64,
    depth: i64,
    aim: i64
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (command_str, val_str) = match line.split_once(" ") {
            Some(str_pair) => str_pair,
            _ => return Err(())
        };
        let direction = match command_str {
            "forward" => FORWARD,
            "up" => UP,
            "down" => DOWN,
            _ => return Err(())
        };
        let value = match val_str.parse::<i64>() {
            Ok(x) => x,
            Err(_) => return Err(())
        };
        Ok(Command { direction, value })
    }
}

impl Command {
    fn apply(&self, orig_pos: (i64, i64)) -> (i64, i64) {
        let (horizontal, depth) = orig_pos;
        match self.direction {
            FORWARD => (horizontal + self.value, depth),
            UP => (horizontal, depth - self.value),
            DOWN => (horizontal, depth + self.value)
        }
    }

    fn apply_with_aim(&self, orig_pos: Position) -> Position {
        let (horizontal, depth, aim) = (orig_pos.horizontal, orig_pos.depth, orig_pos.aim);
        match self.direction {
            FORWARD => Position { horizontal: horizontal + self.value, depth: depth + aim * self.value, aim },
            UP => Position { horizontal, depth, aim: aim - self.value},
            DOWN => Position { horizontal, depth, aim: aim + self.value}
        }
    }
}

/// Returns final (horizontal position, depth) of the submarine, not taking aim into account
pub fn dive_simple(path: &str) -> (i64, i64) {
    let path = std::path::Path::new(path);
    let mut file = std::fs::File::open(path).expect("Cannot open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cannot read file");
    buf.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Command::try_from(s).expect("Ill-formatted file"))
        .fold((0, 0), |pos, cmd| cmd.apply(pos))
}

/// Returns final (horizontal position, depth) of the submarine, taking aim into account
pub fn dive_with_aim(path: &str) -> (i64, i64) {
    let path = std::path::Path::new(path);
    let mut file = std::fs::File::open(path).expect("Cannot open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cannot read file");
    let final_pos = buf.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Command::try_from(s).expect("Ill-formatted file"))
        .fold(Position{ horizontal: 0, depth: 0, aim: 0},
              |pos, cmd| cmd.apply_with_aim(pos));
    (final_pos.horizontal, final_pos.depth)
}

#[test]
fn test_dive() {
    assert!(dive_simple("data/02/test_a.txt") == (15, 10));
    assert!(dive_with_aim("data/02/test_a.txt") == (15, 60));
}