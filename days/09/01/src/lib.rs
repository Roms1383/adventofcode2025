use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    None,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub color: Color,
    pub point: Point,
}

impl Tile {
    pub fn red(x: usize, y: usize) -> Self {
        Tile {
            color: Color::Red,
            point: Point { x, y },
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub trait Rectangle {
    type Rhs;
    fn width(&self, other: &Self::Rhs) -> usize;
    fn height(&self, other: &Self::Rhs) -> usize;
    fn area(&self, other: &Self::Rhs) -> usize;
}

impl Rectangle for Point {
    type Rhs = Point;
    #[inline]
    fn width(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + 1
    }
    #[inline]
    fn height(&self, other: &Point) -> usize {
        self.y.abs_diff(other.y) + 1
    }
    #[inline]
    fn area(&self, other: &Point) -> usize {
        self.width(other) * self.height(other)
    }
}

impl Rectangle for Tile {
    type Rhs = Tile;
    #[inline]
    fn width(&self, other: &Tile) -> usize {
        self.point.width(&other.point)
    }
    #[inline]
    fn height(&self, other: &Tile) -> usize {
        self.point.height(&other.point)
    }
    #[inline]
    fn area(&self, other: &Tile) -> usize {
        self.point.area(&other.point)
    }
}

pub struct Tiles(Vec<Tile>);

impl Tiles {
    pub fn largest_area(&self) -> usize {
        let mut max: usize = 0;
        let mut area: usize;
        let mut seen = HashSet::<(Point, Point)>::new();
        let mut chunk;
        for (i, left) in self.0.iter().filter(|x| x.color == Color::Red).enumerate() {
            chunk = vec![];
            for (_, right) in self
                .0
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, x)| x.color == Color::Red)
                .filter(|(j, x)| i == *j || !seen.contains(&(left.point, x.point)))
            {
                area = left.area(&right);
                if area > max {
                    max = area;
                }
                chunk.push((left.point, right.point));
            }
            seen.extend(chunk.as_slice());
        }
        max
    }
}

impl FromStr for Tiles {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .filter_map(|x| x.split_once(','))
            .map(|(x, y)| Tile::red(x.parse().unwrap(), y.parse().unwrap()))
            .collect::<Vec<_>>();
        Ok(Self(tiles))
    }
}

#[allow(dead_code)]
pub struct TilesDisplay(Tiles);

impl FromStr for TilesDisplay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let color = match c {
                            '.' => Color::None,
                            '#' => Color::Red,
                            _ => panic!("Invalid color"),
                        };
                        Tile {
                            color,
                            point: Point::new(x, y),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Self(Tiles(tiles)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = ".#.\n#.#";
        let tiles = TilesDisplay::from_str(input).unwrap();
        assert_eq!(tiles.0.0.len(), 6);
        assert_eq!(tiles.0.0[0].color, Color::None);
        assert_eq!(tiles.0.0[1].color, Color::Red);
        assert_eq!(tiles.0.0[2].color, Color::None);
        assert_eq!(tiles.0.0[3].color, Color::Red);
        assert_eq!(tiles.0.0[4].color, Color::None);
        assert_eq!(tiles.0.0[5].color, Color::Red);
    }

    #[test]
    fn area() {
        let a = Point { x: 11, y: 1 };
        let b = Point { x: 2, y: 5 };
        assert_eq!(a.area(&b), 50);
    }

    #[test]
    fn example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = Tiles::from_str(input).unwrap();
        let area = tiles.largest_area();
        assert_eq!(area, 50);
    }
}
