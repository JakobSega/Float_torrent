use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Range {
    pub from: u64,
    pub to: u64,
    pub step: u64,
}
pub trait Sequence<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;
    fn k_th(&self, k: usize) -> Option<T>;

    fn range(&self, range: Range) -> Vec<Option<T>> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }

    fn contains(&self, item: T) -> bool;
}
