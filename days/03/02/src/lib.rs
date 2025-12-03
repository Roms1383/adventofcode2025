use std::{ops::Range, str::FromStr};

pub struct Banks(Vec<Bank>);
pub struct Bank(Vec<usize>);

impl FromStr for Banks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut banks = Vec::new();
        for line in s.lines() {
            let bank = Bank::from_str(line)?;
            banks.push(bank);
        }
        Ok(Banks(banks))
    }
}

impl Banks {
    pub fn total_joltage(&self) -> usize {
        self.0.iter().map(|bank| bank.total_joltage()).sum()
    }
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        Ok(Bank(digits))
    }
}

pub fn highest(range: Range<usize>, digits: &[usize]) -> usize {
    assert!((0..=digits.len()).contains(&range.start));
    assert!((0..=digits.len()).contains(&range.end));
    let mut j = range.start;
    if digits[j] == 9 {
        return j;
    }
    for i in j + 1..range.end {
        if digits[i] == 9 {
            return i;
        }
        if digits[i] > digits[j] {
            j = i;
        }
    }
    j
}

impl Bank {
    pub fn highest_indexes(&self) -> [usize; 12] {
        let len = self.0.len();
        let mut indexes = [0; 12];
        let mut left = 11;
        let mut start = 0;
        let mut end = len - left;
        let mut idx = 0;
        loop {
            start = highest(start..end, &self.0);
            indexes[idx] = start;
            if left < 1 {
                break;
            }
            left -= 1;
            start += 1;
            idx += 1;
            end = len - left;
            if (start..end).len() < 2 {
                break;
            }
        }
        for i in start..len {
            if idx >= 12 {
                break;
            }
            indexes[idx] = i;
            idx += 1;
        }
        indexes
    }

    pub fn highest_joltage(&self) -> [usize; 12] {
        let indexes = self.highest_indexes();
        let mut joltage = [0; 12];
        for i in 0..12 {
            joltage[i] = self.0[indexes[i]];
        }
        joltage
    }

    pub fn total_joltage(&self) -> usize {
        let [
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
            eight,
            nine,
            ten,
            eleven,
            twelve,
        ] = self.highest_joltage();
        format!("{one}{two}{three}{four}{five}{six}{seven}{eight}{nine}{ten}{eleven}{twelve}")
            .parse::<usize>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("987654321111111", 987654321111 ; "joltage-987654321111111")]
    #[test_case("811111111111119", 811111111119 ; "joltage-811111111111119")]
    #[test_case("234234234234278", 434234234278 ; "joltage-234234234234278")]
    #[test_case("818181911112111", 888911112111 ; "joltage-818181911112111")]
    fn batteries(given: &str, expected: usize) {
        let bank = Bank::from_str(given).unwrap();
        let result = bank.total_joltage();
        assert_eq!(result, expected);
    }

    #[test]
    fn joltage() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = Banks::from_str(input).unwrap();
        assert_eq!(banks.total_joltage(), 3121910778619);
    }
}
