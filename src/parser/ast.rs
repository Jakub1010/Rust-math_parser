
pub trait Expression {
    fn eval(&self) -> f64;
}

pub type ExpressionPointer = Box<dyn Expression>;

// Constant

pub struct Constant {
    value: f64,
}

impl Constant {
    pub fn create(value: f64) -> Self {
        Self { value: value }
    }
}

impl Expression for Constant {
    fn eval(&self) -> f64 {
        self.value
    }
}

// BinaryOp

pub struct BinaryOp {
    op: char,
    lexp: ExpressionPointer,
    rexp: ExpressionPointer,
} 

impl BinaryOp {
    pub fn create(op: char, lexp: ExpressionPointer, rexp: ExpressionPointer) -> Self {
        Self { op: op, lexp: lexp, rexp: rexp }
    }
}

impl Expression for BinaryOp {
    fn eval(&self) -> f64 {
        match self.op {
            '+' => self.lexp.eval() + self.rexp.eval(),
            '-' => self.lexp.eval() - self.rexp.eval(),
            '*' => self.lexp.eval() * self.rexp.eval(),
            '/' => self.lexp.eval() / self.rexp.eval(),
            _   => self.lexp.eval() + self.rexp.eval(), // temp
        }
    }
}
