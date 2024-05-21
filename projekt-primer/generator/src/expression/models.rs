use std::fmt::Write; // Required for write!

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Pow,
}

#[derive(Debug)]
pub enum AExpr {
    Num(i64),
    Variable(String),
    BinOp(Box<AExpr>, BinaryOperation, Box<AExpr>),
}

impl AExpr {
    pub fn represent(&self) -> String {
        let mut repr = String::new();
        match self {
            AExpr::Num(x) => {
                write!(repr, "{}", x).unwrap();
            },
            AExpr::Variable(var) => {
                write!(repr, "({})", var).unwrap();
            },
            AExpr::BinOp(left, op, right) => {
                write!(repr, " ({} ", (*left).represent()).unwrap();
                match op {
                    BinaryOperation::Add => write!(repr, "+").unwrap(),
                    BinaryOperation::Mul => write!(repr, "*").unwrap(),
                    BinaryOperation::Sub => write!(repr, "-").unwrap(),
                    BinaryOperation::Pow => write!(repr, "^").unwrap(),
                }
                write!(repr, "{})", (*right).represent()).unwrap();
            },
        }
        repr // Ensure the repr is returned for all match arms outside the match block
    }
}
