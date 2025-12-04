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
pub struct Grid {
    map: BTreeMap<Key, Value>,
    max_y: usize,
    max_x: usize,
}

impl Grid {
    pub fn accessible(&self, key: &Key) -> bool {
        let mut count = 0;
        let top = key.y == 0;
        let bottom = key.y == self.max_y;
        let left = key.x == 0;
        let right = key.x == self.max_x;
        // there can never be more than 3 rolls in corners
        if (top || bottom) && (left || right) {
            return true;
        }
        // horizontal
        if !left
            && self.map.get(&Key {
                y: key.y,
                x: key.x - 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        if !right
            && self.map.get(&Key {
                y: key.y,
                x: key.x + 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        // vertical
        if !top
            && self.map.get(&Key {
                y: key.y - 1,
                x: key.x,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        if !bottom
            && self.map.get(&Key {
                y: key.y + 1,
                x: key.x,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        // diagonal
        // upper-left
        if !top
            && !left
            && self.map.get(&Key {
                y: key.y - 1,
                x: key.x - 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        // upper-right
        if !top
            && !right
            && self.map.get(&Key {
                y: key.y - 1,
                x: key.x + 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        // lower-left
        if !bottom
            && !left
            && self.map.get(&Key {
                y: key.y + 1,
                x: key.x - 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        // lower-right
        if !bottom
            && !right
            && self.map.get(&Key {
                y: key.y + 1,
                x: key.x + 1,
            }) == Some(&Value::Roll)
        {
            count += 1;
        }
        count < 4
    }
    pub fn accessibles(&self) -> Vec<Key> {
        self.map
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
                *me.map.get_mut(key).unwrap() = Value::Empty;
            }
        }
        total
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::new();
        let mut max_y: usize = 0;
        let mut max_x: usize = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => map.insert(Key { x, y }, Value::Empty),
                    '@' => map.insert(Key { x, y }, Value::Roll),
                    _ => panic!("invalid value ({char})"),
                };
                max_x = x;
            }
            max_y = y;
        }
        Ok(Self { map, max_x, max_y })
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut out = String::new();
        let mut last_y = 0;
        for (k, v) in self.map.iter() {
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
