use crate::sequence::models::Sequence;

// Implementirajte konstantno zaporedje
#[derive(Debug)]
pub struct Constant<T> {
    a_0 : T
}
impl<T> Constant<T> {
    pub fn new(a_0 : T) -> Self {
        Constant{a_0 : a_0}
    }
}

impl Sequence<i64> for Constant<i64> {
    fn name(&self) -> String {
       format!("Constant sequence with a_0 = {:?}", self.a_0) 
    }
    fn start(&self) -> i64 {
        self.a_0
    }
    fn k_th(&self, _k: usize) -> Option<i64> {
        Some(self.a_0)
    }

    fn contains(&self, item: i64) -> bool {
        self.a_0 == item
    }
}

impl Sequence<f64> for Constant<f64> {
    fn name(&self) -> String {
       format!("Constant sequence with a_0 = {:?}", self.a_0) 
    }
    fn start(&self) -> f64 {
        self.a_0
    }
    fn k_th(&self, _k: usize) -> Option<f64> {
        Some(self.a_0)
    }

    fn contains(&self, item: f64) -> bool {
        self.a_0 == item
    }
}