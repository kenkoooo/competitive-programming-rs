#![feature(test)]
extern crate test;

use competitive_programming_rs::data_structure::treap::treap::Treap;
use rand::prelude::*;
use test::Bencher;

#[bench]
fn bench_treap(b: &mut Bencher) {
    b.iter(|| {
        let mut treap = Treap::new(1234);
        let mut rng = StdRng::seed_from_u64(1234);
        for _ in 0..100_000 {
            let x = rng.gen::<u64>();
            let y = rng.gen::<u64>();
            if y % 10 == 0 {
                treap.erase(&x);
            } else {
                treap.insert(x);
            }
        }
    });
}
