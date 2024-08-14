//use crate::expression::models::AExpr;
//use crate::expression::evaluation;
use crate::sequence::models::Sequence;
//use std::collections::HashMap;
use std::fmt::Write;

pub struct LinearCombination<'a> {
    sequences: Vec<Box<&'a dyn Sequence<f64>>>,
    scalers: Vec<f64>,
}

impl<'a> Sequence<f64> for LinearCombination<'a> {
    fn name(&self) -> String {
        let mut name = String::new();
        write!(name, "Linear Combination of the following sequences:\n").unwrap();
        for seq in &self.sequences {
            write!(name, "*** {}\n", seq.name()).unwrap();
        }
        write!(name, "Scalers (first scaler is multiplied with the Constant sequence 1):\n").unwrap();
        write!(name, "{:?}", self.scalers).unwrap(); // Assuming you want to display scalers
        name
    }

    fn start(&self) -> f64 {
        assert_eq!(self.sequences.len() + 1, self.scalers.len(), "The number of sequences should be exactly one less than the number of scalers...");
        let mut result: f64 = self.scalers[0];
        let scalers = &self.scalers[1..];
        for (sequence, scaler) in self.sequences.iter().zip(scalers.iter()) {
            result += sequence.start() * scaler;
        }
        result
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        assert_eq!(self.sequences.len() + 1, self.scalers.len(), "The number of sequences should be exactly one less than the number of scalers...");
        let mut result: f64 = self.scalers[0];
        let scalers = &self.scalers[1..];
        for (sequence, scaler) in self.sequences.iter().zip(scalers.iter()) {
            if let Some(x) = sequence.k_th(k) {
                result += scaler * x;
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Linear combination!")
    }
}

impl<'a> LinearCombination<'a> {
    fn new(sequences: Vec<Box<&'a dyn Sequence<f64>>>, scalers: Vec<f64>) -> Box<LinearCombination<'a>> {
        Box::new(LinearCombination { sequences, scalers })
    }
}

pub fn linear_combination<'a>(
    sequences: Vec<Box<&'a dyn Sequence<f64>>>,
    scalers: Vec<f64>,
) -> Box<dyn Sequence<f64> + 'a> {
    LinearCombination::new(sequences, scalers)
}
