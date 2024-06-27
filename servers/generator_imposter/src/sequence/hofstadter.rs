use super::models::Sequence;
use std::collections::HashMap;



pub struct HofstadterSequence;

impl HofstadterSequence {
    pub fn new() -> Self {
        HofstadterSequence
    }

    fn hofstadter_sequence(n: usize, memo: &mut HashMap<usize, f64>) -> f64 {
        if n == 0 {
            return 0.0;
        }

        if let Some(&result) = memo.get(&n) {
            return result;
        }

        let gn_minus_1 = HofstadterSequence::hofstadter_sequence(n - 1, memo) as usize;
        let result = n as f64 - HofstadterSequence::hofstadter_sequence(gn_minus_1, memo);
        memo.insert(n, result);
        
        result
    }
}

impl Sequence<f64> for HofstadterSequence {
    fn name(&self) -> String {
        String::from("Hofstadter Sequence")
    }

    fn start(&self) -> f64 {
        0.0
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        let mut memo = HashMap::new();
        Some(HofstadterSequence::hofstadter_sequence(k, &mut memo))
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Uh-Oh")
    }
}
