use std::{
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dial {
    points: usize,
    counts: usize,
}

impl std::cmp::PartialEq<usize> for Dial {
    fn eq(&self, other: &usize) -> bool {
        self.points == *other
    }
}

impl std::ops::SubAssign<usize> for Dial {
    fn sub_assign(&mut self, mut rhs: usize) {
        let is_zero = self.points == 0;
        let mut wraps = 0;
        wraps += rhs.div_euclid(100);
        rhs = rhs.rem_euclid(100);
        if rhs > self.points {
            if !is_zero {
                wraps += 1;
            }
            rhs -= self.points;
            self.points = 100 - rhs;
        } else {
            self.points -= rhs;
        }
        if self.points == 0 {
            wraps += 1;
        }
        self.counts += wraps;
    }
}

impl std::ops::AddAssign<usize> for Dial {
    fn add_assign(&mut self, mut rhs: usize) {
        let is_zero = self.points == 0;
        let mut wraps = 0;
        wraps += rhs.div_euclid(100);
        rhs = rhs.rem_euclid(100);
        let overflow = rhs + self.points > 99;
        if overflow {
            if !is_zero {
                wraps += 1;
            }
            self.points = (rhs + self.points) - 100;
        } else {
            self.points += rhs;
        }
        if !overflow && self.points == 0 {
            wraps += 1;
        }
        self.counts += wraps;
    }
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            points: 50,
            counts: 0,
        }
    }
}

impl Dial {
    #[cfg(test)]
    pub fn new(points: usize) -> Self {
        Dial { points, counts: 0 }
    }
    pub fn is_zero(&self) -> bool {
        self.points == 0
    }
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left(clicks) => self.sub_assign(clicks),
            Rotation::Right(clicks) => self.add_assign(clicks),
        }
    }
    pub fn password(&self) -> usize {
        self.counts
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Left(usize),
    Right(usize),
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Left(clicks) => format!("L{}", clicks),
                Rotation::Right(clicks) => format!("R{}", clicks),
            }
        )
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Rotation::Left(
                s[1..]
                    .parse()
                    .map_err(|_| format!("invalid left rotation ({s})"))?,
            )),
            Some('R') => Ok(Rotation::Right(
                s[1..]
                    .parse()
                    .map_err(|_| format!("invalid right rotation ({s})"))?,
            )),
            _ => Err(format!("invalid rotation ({s})")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_sub() {
        let mut zero = Dial::new(0);
        zero -= 1;
        assert_eq!(zero, 99);
        let mut zero = Dial::new(0);
        zero -= 100;
        assert_eq!(zero, 0);
        let mut zero = Dial::new(0);
        zero -= 101;
        assert_eq!(zero, 99);
    }

    #[test]
    fn wrapping_add() {
        let mut zero = Dial::new(0);
        zero += 1;
        assert_eq!(zero, 1);
        let mut zero = Dial::new(0);
        zero += 100;
        assert_eq!(zero, 0);
        let mut zero = Dial::new(0);
        zero += 101;
        assert_eq!(zero, 1);
    }

    #[test]
    fn rotations() {
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Left(1));
        assert_eq!(zero, 99);
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Left(100));
        assert_eq!(zero, 0);
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Left(101));
        assert_eq!(zero, 99);
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Right(1));
        assert_eq!(zero, 1);
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Right(100));
        assert_eq!(zero, 0);
        let mut zero = Dial::new(0);
        zero.rotate(Rotation::Right(101));
        assert_eq!(zero, 1);
    }

    #[test]
    fn deserialization() {
        let left = "L10";
        let right = "R10";
        assert_eq!(left.parse(), Ok(Rotation::Left(10)));
        assert_eq!(right.parse(), Ok(Rotation::Right(10)));
    }

    #[test]
    fn example() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::Left(68));
        dial.rotate(Rotation::Left(30));
        dial.rotate(Rotation::Right(48));
        dial.rotate(Rotation::Left(5));
        dial.rotate(Rotation::Right(60));
        dial.rotate(Rotation::Left(55));
        dial.rotate(Rotation::Left(1));
        dial.rotate(Rotation::Left(99));
        dial.rotate(Rotation::Right(14));
        dial.rotate(Rotation::Left(82));
        assert_eq!(dial.password(), 6);
    }

    #[test]
    fn large_rotation() {
        let mut dial = Dial::default();
        dial.rotate(Rotation::Right(1000));
        assert_eq!(dial, 50);
        assert_eq!(dial.password(), 10);
    }
}
