use std::str::FromStr;

#[derive(Debug)]
pub struct ID(String);

impl ID {
    pub fn new(id: &str) -> Self {
        assert!(id.parse::<usize>().is_ok());
        Self(id.to_string())
    }
    pub fn new_with(id: usize) -> Self {
        Self(id.to_string())
    }
    pub fn invalid(&self) -> bool {
        let chars = self.0.chars().collect::<Vec<_>>();
        let len = chars.len();
        let half = len / 2;
        let mut buffer = Vec::with_capacity(half);
        for i in 1.. {
            if i > half {
                break;
            }
            if len.rem_euclid(i) != 0 {
                continue;
            }
            if same(i, chars.as_slice(), &mut buffer) {
                return true;
            }
        }
        false
    }
    pub fn to_usize(&self) -> usize {
        self.0.parse().expect("invalid id")
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct IDRange {
    start: ID,
    end: ID,
}

/// SAFETY: input length must be divisible by divisor
fn same<'a>(divisor: usize, chars: &'a [char], buffer: &mut Vec<&'a [char]>) -> bool {
    *buffer = chars.chunks(divisor).collect::<Vec<_>>();
    for pair in buffer.windows(2) {
        if pair[0] != pair[1] {
            return false;
        }
    }
    true
}

impl FromStr for IDRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once('-') else {
            return Err(format!("invalid range ({s})"));
        };
        Ok(Self {
            start: ID::new(start),
            end: ID::new(end),
        })
    }
}

pub struct IDRangeIterator {
    current: usize,
    #[allow(dead_code)]
    start: usize,
    end: usize,
}

impl Iterator for IDRangeIterator {
    type Item = ID;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let id = ID::new_with(self.current);
            self.current += 1;
            Some(id)
        }
    }
}

impl IntoIterator for IDRange {
    type Item = ID;

    type IntoIter = IDRangeIterator;

    fn into_iter(self) -> Self::IntoIter {
        IDRangeIterator {
            current: self.start.to_usize(),
            start: self.start.to_usize(),
            end: self.end.to_usize(),
        }
    }
}

pub struct IDRanges(Vec<IDRange>);

impl IDRanges {
    pub fn sum_invalids(self) -> usize {
        let mut invalids = Vec::with_capacity(self.0.len());
        for range in self.0 {
            for id in range {
                if id.invalid() {
                    invalids.push(id.to_usize());
                }
            }
        }
        invalids.iter().sum()
    }
}

impl FromStr for IDRanges {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(',');
        let mut ranges = Vec::new();
        let mut range;
        for x in s {
            range = IDRange::from_str(x)?;
            ranges.push(range);
        }
        Ok(Self(ranges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ids() {
        let id = ID::new("12345");
        assert!(!id.invalid());
        let id = ID::new("123122");
        assert!(!id.invalid());
    }

    #[test]
    fn invalid_ids() {
        let id = ID::new("11");
        assert!(id.invalid());
        let id = ID::new("1212");
        assert!(id.invalid());
    }

    use test_case::test_case;

    #[test_case("11-22", 2 ; "range-11-22")]
    #[test_case("95-115", 2 ; "range-95-115")]
    #[test_case("998-1012", 2 ; "range-998-1012")]
    #[test_case("1188511880-1188511890", 1 ; "range-1188511880-1188511890")]
    #[test_case("222220-222224", 1 ; "range-222220-222224")]
    #[test_case("1698522-1698528", 0 ; "range-1698522-1698528")]
    #[test_case("446443-446449", 1 ; "range-446443-446449")]
    #[test_case("38593856-38593862", 1 ; "range-38593856-38593862")]
    #[test_case("565653-565659", 1 ; "range-565653-565659")]
    #[test_case("824824821-824824827", 1 ; "range-824824821-824824827")]
    #[test_case("2121212118-2121212124", 1 ; "range-2121212118-2121212124")]
    fn example(given: &str, expected: usize) {
        let r: IDRange = given.parse().unwrap();
        assert_eq!(
            r.into_iter()
                .filter(|x| x.invalid())
                .collect::<Vec<_>>()
                .len(),
            expected
        );
    }

    #[test]
    fn ranges() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = IDRanges::from_str(input).unwrap();
        assert_eq!(ranges.sum_invalids(), 4174379265);
    }
}
