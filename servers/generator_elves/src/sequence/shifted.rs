use super::models::Sequence;

pub struct Shifted<'a, T> {
    base_sequence : &'a dyn Sequence<T>,
    shift : usize
}


impl<T> Shifted<'_, T> {
    fn new(shift: usize, base_sequence: &dyn Sequence<T>) -> Box<Shifted<T>> {
        Box::new(Shifted{base_sequence : base_sequence, shift : shift})
    }
}

pub fn shifted_sequence(
    base_sequence: &dyn Sequence<i64>,
    shift: usize,
) -> Box<dyn Sequence<i64> + '_> {
    Shifted::new(shift, base_sequence)
}

impl<T> Sequence<T> for Shifted<'_, T>{ 
    fn name(&self) -> String {
        format!("Shifted sequence. Its base sequence is {}. Its shift it {}", (self.base_sequence).name(), self.shift)
    }

    fn start(&self) -> T {
        match (self.base_sequence).k_th(self.shift) {
            Some(a) => a,
            None => panic!("The shift is probably negative.")
        }
    }
    fn k_th(&self, k: usize) -> Option<T> {
        (self.base_sequence).k_th(self.shift + k)
    }

    fn contains(&self, _item: T) -> bool {
        true
    }
}