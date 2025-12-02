use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dial(usize);

impl std::ops::Sub<usize> for Dial {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self((self.0 as isize - rhs as isize).rem_euclid(100) as usize)
    }
}

impl std::ops::Add<usize> for Dial {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self((self.0 + rhs).rem_euclid(100))
    }
}

impl Default for Dial {
    fn default() -> Self {
        Dial(50)
    }
}

impl Dial {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Left(clicks) => self - clicks,
            Rotation::Right(clicks) => self + clicks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Left(usize),
    Right(usize),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('L') => Ok(Rotation::Left(
                s[1..]
                    .parse()
                    .map_err(|_| "invalid left rotation".to_string())?,
            )),
            Some('R') => Ok(Rotation::Right(
                s[1..]
                    .parse()
                    .map_err(|_| "invalid right rotation".to_string())?,
            )),
            _ => Err("invalid rotation".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetics() {
        let zero = Dial(0);
        assert_eq!(zero + 1, Dial(1));
        assert_eq!(zero + 100, Dial(0));
        assert_eq!(zero + 101, Dial(1));
        assert_eq!(zero - 1, Dial(99));
        assert_eq!(zero - 100, Dial(0));
        assert_eq!(zero - 101, Dial(99));
    }

    #[test]
    fn rotations() {
        let zero = Dial(0);
        assert_eq!(zero.rotate(Rotation::Right(1)), Dial(1));
        assert_eq!(zero.rotate(Rotation::Right(100)), Dial(0));
        assert_eq!(zero.rotate(Rotation::Right(101)), Dial(1));
        assert_eq!(zero.rotate(Rotation::Left(1)), Dial(99));
        assert_eq!(zero.rotate(Rotation::Left(100)), Dial(0));
        assert_eq!(zero.rotate(Rotation::Left(101)), Dial(99));
    }

    #[test]
    fn deserialization() {
        let left = "L10";
        let right = "R10";
        assert_eq!(left.parse(), Ok(Rotation::Left(10)));
        assert_eq!(right.parse(), Ok(Rotation::Right(10)));
    }
}
