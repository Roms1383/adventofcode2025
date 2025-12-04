use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Empty,
    Roll,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    y: usize,
    x: usize,
}

impl Key {
    pub fn adjacent(&self, other: &Self) -> bool {
        let same_x = self.x.eq(&other.x);
        let same_y = self.y.eq(&other.y);
        let diff_x = self.x.abs_diff(other.x);
        let diff_y = self.y.abs_diff(other.y);
        (same_x && diff_y == 1) || (same_y && diff_x == 1) || (diff_x == 1 && diff_y == 1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid(BTreeMap<Key, Value>);

impl Grid {
    pub fn accessible(&self, key: &Key) -> bool {
        self.0
            .iter()
            .filter(|(_, v)| **v == Value::Roll)
            .filter(|(k, _)| k.adjacent(key))
            .collect::<Vec<_>>()
            .len()
            < 4
    }
    pub fn accessibles(&self) -> Vec<Key> {
        self.0
            .iter()
            .filter(|(_, v)| **v == Value::Roll)
            .filter_map(|(k, _)| if self.accessible(k) { Some(k) } else { None })
            .copied()
            .collect::<Vec<Key>>()
    }
    pub fn count_accessibles(&self) -> usize {
        self.accessibles().len()
    }
    pub fn count_recurse_accessibles(&self) -> usize {
        let mut me = self.clone();
        let mut current: Vec<Key>;
        let mut total: usize = 0;
        let mut count: usize;
        loop {
            current = me.accessibles();
            count = current.len();
            if count == 0 {
                break;
            } else {
                total += count;
            }
            for key in current.iter() {
                *me.0.get_mut(key).unwrap() = Value::Empty;
            }
        }
        total
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => map.insert(Key { x, y }, Value::Empty),
                    '@' => map.insert(Key { x, y }, Value::Roll),
                    _ => panic!("invalid value ({char})"),
                };
            }
        }
        Ok(Self(map))
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut last_y = 0;
        for (k, v) in self.0.iter() {
            if k.y > last_y {
                out.push('\n');
                last_y = k.y;
            }
            if *v == Value::Roll {
                if self.accessible(k) {
                    out.push('x');
                } else {
                    out.push('@');
                }
            } else {
                out.push('.');
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    const EXPECTED: &str = "..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.";

    #[test]
    fn grid() {
        let grid = Grid::from_str(INPUT);
        assert!(grid.is_ok());
        let grid = grid.unwrap();
        assert_eq!(grid.to_string(), EXPECTED);
        assert_eq!(grid.count_recurse_accessibles(), 43);
    }
}
