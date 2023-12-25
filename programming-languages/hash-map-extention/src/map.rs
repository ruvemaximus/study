use std::collections::HashMap;
use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct Map<T> {
    base: HashMap<String, T>
}

impl<T> Map<T> {
    pub fn new() -> Self {
        Self {
            base: HashMap::<String, T>::new()
        }
    }
}

impl Index<usize> for Map<i8> {
    type Output = i8;

    fn index(&self, index: usize) -> &Self::Output {
        let mut tmp: Vec<String> = self.base.clone().into_keys().collect();
        tmp.sort();
        self.base.get(&tmp[index]).unwrap()
    }
}

impl Index<String> for Map<i8> {
    type Output = i8;

    fn index(&self, index: String) -> &Self::Output {
        &self.base.get(&index).unwrap()
    }
}

impl IndexMut<String> for Map<i8> {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        // TODO Может быть вместить это в 1 функцию?
        self.base.insert(index.clone(), 0);
        self.base.get_mut(&index).unwrap()
    }
}


#[test]
fn get_by_index() {
    let mut a: Map<i8> = Map::new();

    a["A".to_string()] = 1;
    a["C".to_string()] = 3;
    a["B".to_string()] = 2;

    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[2], 3);
}

#[test]
fn get_by_string() {

}
