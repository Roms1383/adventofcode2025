use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

pub struct Goods {
    fresh: Vec<RangeInclusive<usize>>,
    ingredients: HashSet<usize>,
}

impl Goods {
    pub fn count_fresh(&self) -> usize {
        let mut count = 0;
        for ingredient in self.ingredients.iter() {
            if self.fresh.iter().any(|x| x.contains(ingredient)) {
                count += 1;
            }
        }
        count
    }
}

impl FromStr for Goods {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fresh = Vec::new();
        let mut ingredients = HashSet::new();
        for line in s.lines().take_while(|x| !x.is_empty()) {
            let (start, end) = line.split_once('-').expect("invalid range");
            fresh.push(RangeInclusive::new(
                start.parse().expect("invalid digit"),
                end.parse().expect("invalid digit"),
            ));
        }
        for line in s.lines().skip_while(|x| x.contains('-') || x.is_empty()) {
            ingredients.insert(line.parse().expect("invalid digit"));
        }
        Ok(Self { fresh, ingredients })
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
        assert_eq!(goods.count_fresh(), 3);
    }
}
