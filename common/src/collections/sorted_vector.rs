use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct SortedVec<T>(Vec<T>)
where
    T: Ord;

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        SortedVec(Vec::new())
    }

    pub fn insert(&mut self, item: T) {
        self.0
            .insert(self.0.binary_search(&item).unwrap_or_else(|pos| pos), item);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0.binary_search(item).is_ok()
    }
}

impl<T: Ord + PartialEq> PartialEq for SortedVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Ord + Eq> Eq for SortedVec<T> {}

impl<T: Ord + Hash> Hash for SortedVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Ord + Hash> SortedVec<T> {
    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Ord + PartialOrd> PartialOrd for SortedVec<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for SortedVec<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Ord> FromIterator<T> for SortedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec: Vec<T> = iter.into_iter().collect();
        vec.sort();
        SortedVec(vec)
    }
}

impl<'a, T: Ord> IntoIterator for &'a SortedVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: Ord> Index<usize> for SortedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let sv: SortedVec<i32> = SortedVec::new();
        assert_eq!(sv.as_slice(), &[]);
    }

    #[test]
    fn test_insert_sorted() {
        let mut sv: SortedVec<i32> = SortedVec::new();
        sv.insert(3);
        sv.insert(1);
        sv.insert(4);
        sv.insert(2);
        assert_eq!(sv.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_insert_duplicated() {
        let mut sv: SortedVec<i32> = SortedVec::new();
        sv.insert(2);
        sv.insert(1);
        sv.insert(4);
        sv.insert(2);
        assert_eq!(sv.as_slice(), &[1, 2, 2, 4]);
    }

    #[test]
    fn test_from_iterator() {
        let sv: SortedVec<i32> = [3, 1, 4, 1, 5, 9].into_iter().collect();
        assert_eq!(sv.as_slice(), &[1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_equality() {
        let sv1: SortedVec<i32> = [1, 2, 3].into_iter().collect();
        let sv2: SortedVec<i32> = [3, 2, 1].into_iter().collect();
        assert_eq!(sv1, sv2);
    }

    #[test]
    fn test_hash_consistency() {
        let sv1: SortedVec<i32> = [1, 3, 2].into_iter().collect();
        let sv2: SortedVec<i32> = [3, 2, 1].into_iter().collect();
        assert_eq!(sv1.compute_hash(), sv2.compute_hash());
    }

    #[test]
    fn test_as_hashmap_key() {
        let sv1: SortedVec<i32> = [1, 2, 3].into_iter().collect();
        let sv2: SortedVec<i32> = [3, 2, 1].into_iter().collect();

        let mut map: HashMap<SortedVec<i32>, String> = HashMap::new();
        map.insert(sv1.clone(), "value1".to_string());

        map.insert(sv2.clone(), "value2".to_string());

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&sv1), Some(&"value2".to_string()));
        assert_eq!(map.get(&sv2), Some(&"value2".to_string()));
    }

    #[test]
    fn test_ordering() {
        let sv1: SortedVec<i32> = [1, 2].into_iter().collect();
        let sv2: SortedVec<i32> = [1, 2, 3].into_iter().collect();
        assert!(sv1 < sv2);
    }

    #[test]
    fn test_into_iterator_borrowing() {
        let sv: SortedVec<i32> = [3, 1, 2].into_iter().collect();
        let mut iter = (&sv).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_for_loop() {
        let sv: SortedVec<i32> = [3, 1, 4, 1, 5].into_iter().collect();
        let mut sum = 0;
        for &val in &sv {
            sum += val;
        }
        assert_eq!(sum, 1 + 1 + 3 + 4 + 5);
    }

    #[test]
    fn test_contains() {
        let sv: SortedVec<i32> = [3, 1, 4, 1, 5, 9].into_iter().collect();
        assert!(sv.contains(&1));
        assert!(sv.contains(&3));
        assert!(sv.contains(&4));
        assert!(sv.contains(&5));
        assert!(sv.contains(&9));
        assert!(!sv.contains(&2));
        assert!(!sv.contains(&0));
        assert!(!sv.contains(&10));
    }

    #[test]
    fn test_contains_empty() {
        let sv: SortedVec<i32> = SortedVec::new();
        assert!(!sv.contains(&42));
    }

    #[test]
    fn test_index() {
        let sv: SortedVec<i32> = [3, 1, 2].into_iter().collect();
        assert_eq!(sv[0], 1);
        assert_eq!(sv[1], 2);
        assert_eq!(sv[2], 3);
    }
}
