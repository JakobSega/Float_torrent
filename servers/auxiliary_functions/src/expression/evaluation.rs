//use std::fmt::Binary;

use super::models::{AExpr, BinaryOperation};


impl AExpr {
    pub fn evaluate(&self) -> i64 {
        match self {
            AExpr::Num(x) => *x,
            AExpr::Variable(_) => panic!("This method assumes no variables are in the expression. This assumption has not been met."),
            AExpr::BinOp(left, op, right) => match op {
                BinaryOperation::Add => (*left).evaluate() + (*right).evaluate(),
                BinaryOperation::Sub => (*left).evaluate() - (*right).evaluate(),
                BinaryOperation::Mul => (*left).evaluate() * (*right).evaluate(),
                BinaryOperation::Pow => {
                    let base = (*left).evaluate();
                    let exponent = (*right).evaluate();
                    match u32::try_from(exponent) {
                        Ok(value) => base.pow(value),  // Use `value` which is the correctly typed `u32`
                        Err(_) => panic!("Could not convert exponent to u32 because it is out of range"),
                    }
                },
            },
        }
    }

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        match self {
            AExpr::Num(x) => Some(*x),
            AExpr::Variable(var) => {
                match vars.get(var) {
                    Some(&Some(x)) => Some(x), // Unwrap the Option and dereference the value
                    Some(&None) | None => None,
                }
            },
            AExpr::BinOp(left, op, right) => {
                let left_val = (*left).evaluate_map(vars);
                let right_val = (*right).evaluate_map(vars);
                match (left_val, right_val) {
                    (Some(x), Some(y)) => {
                        match op {
                            BinaryOperation::Add => Some(x + y),
                            BinaryOperation::Sub => Some(x - y),
                            BinaryOperation::Mul => Some(x * y),
                            BinaryOperation::Pow => {
                                match u32::try_from(y) {
                                    Ok(exponent) => Some(x.pow(exponent)),
                                    Err(_) => panic!("Could not convert i32 to u32..."),
                                }
                            },
                        }
                    },
                    _ => None,
                }
            },
        }
    }

    pub fn evaluate_f64(&self) -> f64 {
        match self {
            AExpr::Num(x) => *x as f64,
            AExpr::Variable(_) => panic!("This method assumes no variables are in the expression. This assumption has not been met."),
            AExpr::BinOp(left, op, right) => match op {
                BinaryOperation::Add => (*left).evaluate_f64() + (*right).evaluate_f64(),
                BinaryOperation::Sub => (*left).evaluate_f64() - (*right).evaluate_f64(),
                BinaryOperation::Mul => (*left).evaluate_f64() * (*right).evaluate_f64(),
                BinaryOperation::Pow => {
                    let base = (*left).evaluate_f64();
                    let exponent = (*right).evaluate_f64();
                    match exponent as u32 {
                        value => base.powf(value as f64),
                    }
                },
            },
        }
    }

    pub fn evaluate_map_f64(
        &self,
        vars: &std::collections::HashMap<String, Option<f64>>,
    ) -> Option<f64> {
        match self {
            AExpr::Num(x) => Some(*x as f64),
            AExpr::Variable(var) => {
                match vars.get(var) {
                    Some(&Some(x)) => Some(x), // Unwrap the Option and dereference the value
                    Some(&None) | None => None,
                }
            },
            AExpr::BinOp(left, op, right) => {
                let left_val = (*left).evaluate_map_f64(vars);
                let right_val = (*right).evaluate_map_f64(vars);
                match (left_val, right_val) {
                    (Some(x), Some(y)) => {
                        match op {
                            BinaryOperation::Add => Some(x + y),
                            BinaryOperation::Sub => Some(x - y),
                            BinaryOperation::Mul => Some(x * y),
                            BinaryOperation::Pow => {
                                match y {
                                    y if y.is_finite() => Some(x.powf(y)),
                                    _ => panic!("Could not convert i32 to u32..."),
                                }
                            },
                        }
                    },
                    _ => None,
                }
            },
        }
    }

}