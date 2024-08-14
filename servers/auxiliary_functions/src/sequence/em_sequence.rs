use super::models::Sequence;

pub struct EMSequence;

impl EMSequence {
    pub fn new() -> Self {
        EMSequence
    }
}

impl Sequence<f64> for EMSequence {
    fn name(&self) -> String {
        String::from("Euler-Mascheroni sequence approximation")
    }

    fn start(&self) -> f64 {
        1.0
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        if k == 0 {
            return None;
        }

        let mut harmonic_sum = 0.0;
        for i in 1..=k {
            harmonic_sum += 1.0 / i as f64;
        }

        Some(harmonic_sum - (k as f64).ln())
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Uh-Oh")
    }
}
