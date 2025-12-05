use std::{ops::RangeInclusive, str::FromStr};

use range_set_blaze::{MultiwaySortedDisjoint, RangeSetBlaze};

pub trait RangeExt {
    fn count(&self) -> usize;
}

impl RangeExt for RangeInclusive<usize> {
    fn count(&self) -> usize {
        self.end().abs_diff(*self.start()) + 1
    }
}

pub struct Goods {
    fresh: Vec<RangeInclusive<usize>>,
}

impl Goods {
    pub fn triage_ranges(&self) -> Vec<RangeInclusive<usize>> {
        let mut sorted = self.fresh.clone();
        sorted.sort_by(|a, b| a.start().cmp(b.start()));
        sorted
            .into_iter()
            .map(|x| RangeSetBlaze::from_iter([x]).into_ranges())
            .union()
            .collect()
    }
    pub fn count_fresh_ranges(&self) -> usize {
        self.triage_ranges().iter().map(|x| x.count()).sum()
    }
}

impl FromStr for Goods {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh = Vec::new();
        for line in s.lines().take_while(|x| !x.is_empty()) {
            let (start, end) = line.split_once('-').expect("invalid range");
            fresh.push(RangeInclusive::new(
                start.parse().expect("invalid digit"),
                end.parse().expect("invalid digit"),
            ));
        }
        Ok(Self { fresh })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn fresh() {
        let goods = Goods::from_str(INPUT).unwrap();
        assert_eq!(goods.count_fresh_ranges(), 14);
    }
}
