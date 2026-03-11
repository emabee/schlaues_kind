use rand::{RngExt, rng};
use std::borrow::Cow;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Copy, Clone, Default, EnumIter)]
pub enum Operator {
    #[default]
    Multiply,
    Divide,
    Subtract,
}
impl Operator {
    pub fn name(&'_ self) -> Cow<'_, str> {
        match self {
            Self::Multiply => t!("_multiply"),
            Self::Divide => t!("_divide"),
            Self::Subtract => t!("_subtract"),
        }
    }
    pub fn symbol(&self) -> char {
        match self {
            Self::Multiply => '×',
            Self::Divide => '÷',
            Self::Subtract => '-',
        }
    }
    pub fn calculate(&self, operand1: u32, operand2: u32) -> u32 {
        match self {
            Self::Multiply => operand1 * operand2,
            Self::Divide => operand1 / operand2,
            Self::Subtract => operand1 - operand2,
        }
    }
    pub fn new_operands(&self) -> (u32, u32) {
        let mut rng = rng();
        match self {
            Self::Multiply => (rng.random_range(1..10), rng.random_range(1..10)),
            Self::Divide => {
                let divisor = rng.random_range(1..10);
                let dividend = rng.random_range(1..10) * divisor;
                (dividend, divisor)
            }
            Self::Subtract => {
                let (v1, v2) = (rng.random_range(1..100), rng.random_range(1..100));
                (v1.max(v2), v1.min(v2))
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Operation {
    operand1: u32,
    operand2: u32,
    operator: Operator,
}

impl Operation {
    pub fn new(op: Operator) -> Self {
        let (operand1, operand2) = op.new_operands();
        Self {
            operand1,
            operand2,
            operator: op,
        }
    }
    pub fn first(&self) -> &u32 {
        &self.operand1
    }
    pub fn second(&self) -> &u32 {
        &self.operand2
    }
    pub fn operator(&self) -> &Operator {
        &self.operator
    }
    pub fn symbol(&self) -> char {
        self.operator.symbol()
    }
    pub fn new_operands(&mut self) {
        (self.operand1, self.operand2) = self.operator.new_operands();
    }
    pub fn result(&self) -> u32 {
        self.operator.calculate(self.operand1, self.operand2)
    }
}
