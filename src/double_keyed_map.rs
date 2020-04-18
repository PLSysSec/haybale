//! This implementation is taken almost entirely from
//! https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys/45795699.

// we have some methods on `DoubleKeyedMap` that may not currently be used by callers, but they still make sense to be part of `DoubleKeyedMap`
#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Hash)]
pub struct Pair<A, B>(A, B);

impl<A: Clone, B: Clone> Clone for Pair<A, B> {
    fn clone(&self) -> Self {
        Pair(self.0.clone(), self.1.clone())
    }
}

#[derive(PartialEq, Eq, Hash)]
struct BorrowedPair<'a, 'b, A: 'a, B: 'b>(&'a A, &'b B);

trait KeyPair<A, B> {
    /// Obtains the first element of the pair.
    fn a(&self) -> &A;
    /// Obtains the second element of the pair.
    fn b(&self) -> &B;
}

impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for Pair<A, B>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<'a, A: Hash, B: Hash> Hash for (dyn KeyPair<A, B> + 'a) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a().hash(state);
        self.b().hash(state);
    }
}

impl<'a, A: Eq, B: Eq> PartialEq for (dyn KeyPair<A, B> + 'a) {
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() && self.b() == other.b()
    }
}

impl<'a, A: Eq, B: Eq> Eq for (dyn KeyPair<A, B> + 'a) {}

// The main event
pub struct DoubleKeyedMap<A: Eq + Hash, B: Eq + Hash, V> {
    map: HashMap<Pair<A, B>, V>,
}

// pass-through for selected HashMap methods. More can be added as needed.
impl<A: Eq + Hash, B: Eq + Hash, V> DoubleKeyedMap<A, B, V> {
    pub fn new() -> Self {
        DoubleKeyedMap {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, a: &A, b: &B) -> Option<&V> {
        self.map.get(&BorrowedPair(a, b) as &dyn KeyPair<A, B>)
    }

    pub fn get_mut(&mut self, a: &A, b: &B) -> Option<&mut V> {
        self.map.get_mut(&BorrowedPair(a, b) as &dyn KeyPair<A, B>)
    }

    pub fn insert(&mut self, a: A, b: B, v: V) {
        self.map.insert(Pair(a, b), v);
    }

    pub fn remove(&mut self, a: &A, b: &B) -> Option<V> {
        self.map.remove(&BorrowedPair(a, b) as &dyn KeyPair<A, B>)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    // for now we just expose `Pair` in this method
    pub fn entry(&mut self, a: A, b: B) -> std::collections::hash_map::Entry<Pair<A, B>, V> {
        self.map.entry(Pair(a, b))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&A, &B, &V)> {
        self.map.iter().map(|(Pair(a, b), v)| (a, b, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = (&A, &B)> {
        self.map.keys().map(|Pair(a, b)| (a, b))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.map.values_mut()
    }
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone, V: Clone> Clone for DoubleKeyedMap<A, B, V> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
        }
    }
}

impl<A, B> KeyPair<A, B> for Pair<A, B>
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    fn a(&self) -> &A {
        &self.0
    }
    fn b(&self) -> &B {
        &self.1
    }
}

impl<'a, 'b, A, B> KeyPair<A, B> for BorrowedPair<'a, 'b, A, B>
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn a(&self) -> &A {
        self.0
    }
    fn b(&self) -> &B {
        self.1
    }
}
