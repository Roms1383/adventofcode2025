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
    for i in j + 1..range.end {
        if digits[i] > digits[j] {
            j = i;
        }
    }
    j
}

impl Bank {
    pub fn highest_indexes(&self) -> (usize, usize) {
        let start = highest(0..self.0.len() - 1, self.0.as_slice());
        let end = highest(start + 1..self.0.len(), self.0.as_slice());
        (start, end)
    }

    pub fn highest_joltage(&self) -> (usize, usize) {
        let (start, end) = self.highest_indexes();
        (self.0[start], self.0[end])
    }

    pub fn total_joltage(&self) -> usize {
        let (left, right) = self.highest_joltage();
        format!("{}{}", left, right).parse::<usize>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("987654321111111", (9, 8) ; "joltage-987654321111111")]
    #[test_case("811111111111119", (8, 9) ; "joltage-811111111111119")]
    #[test_case("234234234234278", (7, 8) ; "joltage-234234234234278")]
    #[test_case("818181911112111", (9, 2) ; "joltage-818181911112111")]
    fn batteries(given: &str, expected: (usize, usize)) {
        let bank = Bank::from_str(given).unwrap();
        let (start, end) = bank.highest_joltage();
        assert_eq!(start, expected.0);
        assert_eq!(end, expected.1);
    }

    #[test]
    fn joltage() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = Banks::from_str(input).unwrap();
        assert_eq!(banks.total_joltage(), 357);
    }
}
