use super::models::Sequence;
// Implementirajte artimetično zaporedje


pub struct Geometric<T> {
    a : T,
    q : T
}

impl<T> Geometric<T> {
    pub fn new(a : T, q : T) -> Self {
        Geometric{a : a, q : q}
    }
}



impl Sequence<f64> for Geometric<f64>{
    fn name(&self) -> String {
        format!("Geometric sequence with a = {} and q = {}", self.a, self.q)
    }
    fn start(&self) -> f64 {
        self.a
    }
    fn k_th(&self, k: usize) -> Option<f64> {
        if k <= i64::MAX as usize {
            Some(self.a * (self.q).powf((k as i64) as f64))
        } else { None }
    }
    fn contains(&self, _item: f64) -> bool {
        panic!("Uh-Oh")
    }
    
}



