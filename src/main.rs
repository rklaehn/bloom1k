use fnv::FnvHasher;
use std::hash::{Hash, Hasher};
pub struct Bloom1K([u64; 128]);

impl Default for Bloom1K {
    fn default() -> Self {
        Self([0; 128])
    }
}

impl Bloom1K {

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn all() -> Self {
        Self([0xFFFF_FFFF_FFFF_FFFFu64; 128])
    }

    pub fn union(&mut self, rhs: Bloom1K) {
        for i in 0..self.0.len() {
            self.0[i] |= rhs.0[i]
        }
    }

    pub fn intersection(&mut self, rhs: Bloom1K) {
        for i in 0..self.0.len() {
            self.0[i] &= rhs.0[i]
        }
    }

    fn get_bit(&self, offset: usize) -> bool {
        let word = (offset >> 6) & 255;
        let bit = offset & 63;
        let mask = 1u64 << bit;
        (self.0[word] & mask) != 0
    }

    fn set_bit(&mut self, offset: usize) {
        let word = (offset >> 6) & 255;
        let bit = offset & 63;
        let mask = 1u64 << bit;
        self.0[word] |= mask;
    }

    pub fn insert<T: Hash>(&mut self, value: T) {
        let mut hasher = FnvHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let offsets = offsets(hash);
        self.set_bit(offsets[0]);
        self.set_bit(offsets[1]);
        self.set_bit(offsets[2]);
        self.set_bit(offsets[3]);
        self.set_bit(offsets[4]);
    }

    pub fn contains<T: Hash>(&mut self, value: T) -> bool {
        let mut hasher = FnvHasher::default();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        let offsets = offsets(hash);
        self.get_bit(offsets[0])
            && self.get_bit(offsets[1])
            && self.get_bit(offsets[2])
            && self.get_bit(offsets[3])
            && self.get_bit(offsets[4])
    }
}

fn offsets(hash: u64) -> [usize; 5] {
    [
        ((hash >> 0) & 0xFFF) as usize,
        ((hash >> 12) & 0xFFF) as usize,
        ((hash >> 24) & 0xFFF) as usize,
        ((hash >> 36) & 0xFFF) as usize,
        ((hash >> 48) & 0xFFF) as usize,
    ]
}

fn main() {
    let mut filter = Bloom1K::empty();
    let n = 500;
    for i in 0..n {
        filter.insert(i);
    }
    let mut total = 0;
    for i in 0..(n+ 1000) {
        let c = filter.contains(i);
        println!("filter contains {}: {}", i, c);
        if c {
            total += 1;
        }
    }
    println!("n: {}, {} / 1000", n, total - n);
}
