use super::models::Sequence;
// Implementirajte artimetično zaporedje


pub struct Arithmetic<T> {
    a_0 : T,
    d : T
}

impl<T> Arithmetic<T> {
    pub fn new(a_0 : T, d : T) -> Self {
        Arithmetic{a_0 : a_0, d : d}
    }
}

impl Sequence<i64> for Arithmetic<i64>{
    fn name(&self) -> String {
        format!("Arithemtic sequence with a_0 = {} and d = {}", self.a_0, self.d)
    }
    fn start(&self) -> i64 {
        self.a_0
    }
    fn k_th(&self, k: usize) -> Option<i64> {
        if k <= i64::MAX as usize {
            Some(self.a_0 + (k as i64 - 1) * self.d)
        } else { None }
    }
    fn contains(&self, item: i64) -> bool {
        let numerator = item - self.a_0;
        let denominator = self.d;
        if denominator == 0 {
            return self.a_0 == item
        } else {
            numerator % denominator == 0
        }
    }
}
