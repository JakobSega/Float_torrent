use crate::expression::models::AExpr;
//use crate::expression::evaluation;
use crate::sequence::models::Sequence;
use std::collections::HashMap;
use std::fmt::Write;

pub struct Combined<'a, T> {
    sequences : Vec<Box<&'a dyn Sequence<T>>>,
    expression : AExpr
}

impl Sequence<i64> for Combined<'_, i64> {
    fn name(&self) -> String {
        let mut name = String::new();
        write!(name, "Combined sequence, made up of the following sequences:\n").unwrap();
        for seq in &self.sequences {
            write!(name, "*** {}\n", seq.name()).unwrap();
        }
        write!(name, "These sequences are combined with the ecpression:\n").unwrap();
        write!(name, "{}", (self.expression).represent()).unwrap();
        name
    }

    fn start(&self) -> i64 {
        let mut varse = HashMap::new();
        for seq in &self.sequences {
            varse.insert(seq.name(), Some(seq.start()));
        }
        match (self.expression).evaluate_map(&varse) {
            Some(x) => x,
            None => panic!("Could not evaluate the expression...")
        }

    }

    fn k_th(&self, k: usize) -> Option<i64> {
        let mut varse = HashMap::new();
        for seq in &self.sequences {
            varse.insert(seq.name(), seq.k_th(k));
        }
        (self.expression).evaluate_map(&varse)
        
    }

    fn contains(&self, _item: i64) -> bool {
        panic!("Shifted")
    }
}



impl Sequence<f64> for Combined<'_, f64> {
    fn name(&self) -> String {
        let mut name = String::new();
        write!(name, "Combined sequence, made up of the following sequences:\n").unwrap();
        for seq in &self.sequences {
            write!(name, "*** {}\n", seq.name()).unwrap();
        }
        write!(name, "These sequences are combined with the ecpression:\n").unwrap();
        write!(name, "{}", (self.expression).represent()).unwrap();
        name
    }

    fn start(&self) -> f64 {
        let mut varse = HashMap::new();
        for seq in &self.sequences {
            varse.insert(seq.name().to_string(), Some(seq.start()));
        }
        match (self.expression).evaluate_map_f64(&varse) {
            Some(x) => x,
            None => panic!("Could not evaluate the expression...")
        }
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        let mut varse = HashMap::new();
        for seq in &self.sequences {
            varse.insert(seq.name().to_string(), Some(seq.k_th(k).unwrap() as f64));
        }
        (self.expression).evaluate_map_f64(&varse)
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Shifted")
    }
}

impl<T> Combined<'_, T> {
    fn new(sequences: Vec<Box<&dyn Sequence<T>>>, expression: AExpr) -> Box<Combined<T>> {
        Box::new(Combined{sequences : sequences, expression : expression})
    }
}

pub fn combined_sequence(
    sequences: Vec<Box<&dyn Sequence<i64>>>,
    expression: AExpr,
) -> Box<dyn Sequence<i64> + '_> {
    Combined::new(sequences, expression)
}
