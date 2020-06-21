#![feature(test)]
extern crate test;

use competitive_programming_rs::string::rolling_hash::rolling_hash;
use rand::distributions::{IndependentSample, Range};
use rand::{SeedableRng, StdRng};
use test::Bencher;

const BASE: u64 = 1_000_000_007;

#[bench]
fn bench_rolling_hash_construction(b: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let n = 100000;
    let between = Range::new(0, 26);
    let t = (0..n)
        .map(|_| between.ind_sample(&mut rng) as u8 + 'a' as u8)
        .collect::<Vec<_>>();
    b.iter(|| rolling_hash::RollingHash::new(&t, BASE));
}
