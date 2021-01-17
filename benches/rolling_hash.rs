#![feature(test)]
extern crate test;

use competitive_programming_rs::string::rolling_hash::rolling_hash;
use rand::distributions::Uniform;
use rand::prelude::*;
use test::Bencher;

const BASE: u64 = 1_000_000_007;

#[bench]
fn bench_rolling_hash_construction(b: &mut Bencher) {
    let mut rng = StdRng::seed_from_u64(1234);
    let n = 100000;
    let t: Vec<u8> = (0..n).map(|_| rng.sample(Uniform::from(0..26))).collect();
    b.iter(|| rolling_hash::RollingHash::new(&t, BASE));
}
