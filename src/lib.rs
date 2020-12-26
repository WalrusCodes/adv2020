use std::collections::HashMap;
use std::hash::Hash;

pub fn count<T: Eq + Hash + Copy>(stuff: &[T]) -> HashMap<T, usize> {
    let mut out = HashMap::new();
    for s in stuff.iter() {
        *out.entry(*s).or_insert(0) += 1;
    }
    out
}
