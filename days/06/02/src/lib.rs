use std::str::FromStr;

pub struct Operation {
    operands: Vec<usize>,
    operator: Operator,
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines_count = s.lines().count();
        let line_len = s.lines().last().unwrap().chars().count();
        let mut operands: Vec<usize> = vec![];
        let mut operations = vec![];
        let mut digit;
        let mut skip = false;
        for char_idx in 0..line_len {
            if skip {
                operands = vec![];
                skip = false;
                continue;
            }
            digit = String::new();
            for line in s.lines().take(lines_count - 1) {
                let char = line.chars().nth_back(char_idx).expect("missing rev char");
                if char.is_ascii_digit() {
                    digit.push(char);
                }
            }
            if !digit.is_empty() {
                operands.push(digit.parse::<usize>().unwrap());
            }
            match s
                .lines()
                .last()
                .unwrap()
                .chars()
                .nth_back(char_idx)
                .unwrap()
            {
                '+' => {
                    operations.push(Operation {
                        operands: operands.to_vec(),
                        operator: Operator::Add,
                    });
                    skip = true;
                }
                '*' => {
                    operations.push(Operation {
                        operands: operands.to_vec(),
                        operator: Operator::Mul,
                    });
                    skip = true;
                }
                ' ' => {}
                _ => unreachable!(),
            }
        }
        let operations = Operations(operations);
        Ok(operations)
    }
}

impl Operation {
    pub fn calculate(&self) -> usize {
        let mut out: usize;
        match self.operator {
            Operator::Add => {
                out = *self.operands.first().expect("missing first digit");
                for operand in self.operands.iter().skip(1) {
                    out = out.saturating_add(*operand);
                }
            }
            Operator::Mul => {
                out = *self.operands.first().expect("missing first digit");
                for operand in self.operands.iter().skip(1) {
                    out = out.saturating_mul(*operand);
                }
            }
        };
        out
    }
}

pub struct Operations(Vec<Operation>);

impl Operations {
    pub fn calculate(&self) -> usize {
        self.0.iter().map(|x| x.calculate()).sum()
    }
}

pub enum Operator {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    use super::*;

    // because IDE will constantly remove trailing spaces...
    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn example() {
        let operations = Operations::from_str(INPUT).unwrap();
        assert_eq!(operations.calculate(), 3263827);
    }
}
