use super::models::Sequence;

pub struct Fibonacci {
    a: f64,
    b: f64,
}

impl Fibonacci {
    pub fn new(a: f64, b: f64) -> Self {
        Fibonacci { a, b }
    }
}

impl Sequence<f64> for Fibonacci {
    fn name(&self) -> String {
        format!("Fibonacci sequence with first two elements a = {} and b = {}", self.a, self.b)
    }

    fn start(&self) -> f64 {
        self.a
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        if k == 0 {
            Some(self.a)
        } else if k == 1 {
            Some(self.b)
        } else {
            let mut prev = self.a;
            let mut curr = self.b;
            for _ in 2..=k {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            Some(curr)
        }
    }

    fn contains(&self, _item: f64) -> bool {
        panic!("Uh-Oh")
    }
}
