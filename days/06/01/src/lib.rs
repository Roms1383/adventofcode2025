use std::str::FromStr;

pub struct Operation {
    operands: Vec<usize>,
    operator: Operator,
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

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

pub struct Operations(Vec<Operation>);

impl Operations {
    pub fn calculate(&self) -> usize {
        self.0.iter().map(|x| x.calculate()).sum()
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            _ => Err(format!("invalid operator ({s})")),
        }
    }
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operators = s
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|x| Operator::from_str(x).expect("invalid operator"))
            .collect::<Vec<_>>();
        let len = s.lines().count();
        let count = operators.len();
        let mut mixed_operands: Vec<Vec<usize>> = Vec::with_capacity(len - 1);
        let mut operations: Vec<Operation> = Vec::with_capacity(len - 1);
        for line in s.lines().take(len - 1) {
            mixed_operands.push(
                line.split_whitespace()
                    .map(|x| x.parse().expect("invalid digit"))
                    .collect::<Vec<_>>(),
            );
        }
        for i in 0..count {
            operations.push(Operation {
                operands: mixed_operands.iter().map(|x| x[i]).collect(),
                operator: operators[i],
            });
        }
        Ok(Operations(operations))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Operations;

    #[test]
    pub fn example() {
        let input = r#"123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   + "#;
        let operations = Operations::from_str(input).unwrap();
        assert_eq!(operations.calculate(), 4277556);
    }
}
