use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Mul;

#[derive(Debug)]
pub struct Group<T: Sized> {
    map: HashMap<T, T>,
}


impl <T> Mul<Group<T>> for Group<T> where T: Hash + Eq + Sized + Clone{
    type Output = Group<T>;

    fn mul(self, rhs: Group<T>) -> Self::Output {
        let mut map: HashMap<T, T> = HashMap::new();

        for key in self.map.keys() {
            if self.get(&rhs.get(key)) != *key {
                map.insert(key.clone(), self.get(key));
            }
        }

        for key in rhs.map.keys() {
            let res = self.get(&rhs.get(key));
            if res != *key {
                map.insert(key.clone(), self.get(&rhs.get(key)));
            }
        }
        Group { map }
    }
}

impl <T, const N: usize> From<[&[T]; N]> for Group<T> where T: Hash + Eq + Sized + Clone {
    fn from(value: [&[T]; N]) -> Self {
        let mut group: Group<T> = Group::id();
        for cycle in value {
            group = group * Group::from(cycle);
        }
        group
    }
}

impl <T> From<&[T]> for Group<T> where T: Hash + Eq + Sized + Clone {
    fn from(value: &[T]) -> Self {
        let mut map: HashMap<T, T> = HashMap::new();
        for i in 0..(value.len()-1) {
            map.insert(value[i].clone(), value[i+1].clone());
        }
        map.insert(value[value.len()-1].clone(), value[0].clone());
        Group {map}
    }
}

impl <T> From<Vec<Vec<T>>> for Group<T> where T: Hash + Eq + Sized + Clone {

    fn from(value: Vec<Vec<T>>) -> Self {
        let mut group: Group<T> = Group::id();
        for cycle in value {
            let u: &[_] = &cycle;
            group = group * Group::from(u);
        }
        group
    }
}

impl <T, const N: usize> From<[(T, T); N]> for Group<T> where T: Eq + Hash {
    fn from(value: [(T, T); N]) -> Self {
        Group { map: HashMap::from(value) }
    }
}

impl <T> PartialEq for Group<T> where T: Hash + Eq + Sized + Clone {
    fn eq(&self, other: &Self) -> bool {
        for key in self.map.keys() {
            if self.get(key) != other.get(key) {
                return false;
            }
        }

        for key in other.map.keys() {
            if self.get(key) != other.get(key) {
                return false;
            }
        }

        return true;
    }
}

impl <T> Eq for Group<T> where T: Hash + Eq + Sized + Clone {}

impl <T> Group <T> where T: Hash + Eq + Sized + Clone {

    pub fn get(&self, index: &T) -> T {
        if let Some(res) = self.map.get(index) {
            res.clone()
        } else {
            index.clone()
        }
    }

    pub fn cycles(&self) -> Vec<Vec<T>> {
        let mut visited: HashSet<T> = HashSet::new();
        let mut cycles: Vec<Vec<T>> = Vec::new();
        for key in self.map.keys() {
            if !visited.contains(key) {
                let mut curr = key.clone();
                let mut cycle: Vec<T> = Vec::new();
                loop {
                    cycle.push(curr.clone());
                    visited.insert(curr.clone());
                    curr = self.get(&curr);
                    if curr == *key {
                        break;
                    }
                }
                cycles.push(cycle);
            }
        }
        cycles
    }

    pub fn id() -> Group<T> {
        Group { map: HashMap::new() }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
        let group: Group<i32> = Group::id();
        for i in -100..100 {
            assert_eq!(i, group.get(&i));
        }
    }

    #[test]
    fn test_get_small() {
        let group: Group<i32> = Group::from([(0, 1), (1, 3), (3, 2), (2, 0), (4, 5), (5, 4)]);
        assert_eq!(-1, group.get(&-1));
        assert_eq!(1, group.get(&0));
        assert_eq!(3, group.get(&1));
        assert_eq!(0, group.get(&2));
        assert_eq!(2, group.get(&3));
        assert_eq!(5, group.get(&4));
        assert_eq!(4, group.get(&5));
        assert_eq!(10, group.get(&10));
        assert_eq!(321, group.get(&321));
    }

    #[test]
    fn test_eq() {
        let group1: Group<i32> = Group::from([(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 4)]);
        let group2: Group<i32> = Group::id();
        let group3: Group<i32> = Group::from([(3, 0), (2, 3), (1, 2), (0, 1), (5, 4), (4, 5)]);
        let group4: Group<i32> = Group::from([(1, 1)]);

        assert_eq!(group1, group1);
        assert_ne!(group1, group2);
        assert_eq!(group1, group3);
        assert_ne!(group1, group4);

        assert_ne!(group2, group1);
        assert_eq!(group2, group2);
        assert_ne!(group2, group3);
        assert_eq!(group2, group4);

        assert_eq!(group3, group1);
        assert_ne!(group3, group2);
        assert_eq!(group3, group3);
        assert_ne!(group3, group4);

        assert_ne!(group4, group1);
        assert_eq!(group4, group2);
        assert_ne!(group4, group3);
        assert_eq!(group4, group4);
    }
}
