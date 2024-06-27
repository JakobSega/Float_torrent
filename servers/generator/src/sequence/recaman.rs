use super::models::Sequence;

use std::collections::HashSet;



pub struct RecamanSequence;

impl RecamanSequence {
    pub fn new() -> Self {
        RecamanSequence
    }

    fn recaman_sequence(k: usize) -> Vec<f64> {
        let mut sequence = Vec::with_capacity(k + 1);
        let mut seen = HashSet::new();
        let mut current = 0;

        sequence.push(current as f64);
        seen.insert(current);

        for n in 1..=k {
            let next = if current >= n && !seen.contains(&(current - n)) {
                current - n
            } else {
                current + n
            };
            sequence.push(next as f64);
            seen.insert(next);
            current = next;
        }

        sequence
    }
}

impl Sequence<f64> for RecamanSequence {
    fn name(&self) -> String {
        String::from("Recamán's Sequence")
    }

    fn start(&self) -> f64 {
        0.0
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        let sequence = RecamanSequence::recaman_sequence(k);
        sequence.get(k).cloned()
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Uh-Oh")
        
    }
}


