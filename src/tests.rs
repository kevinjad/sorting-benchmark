extern crate test;

use rand::Rng;
use test::Bencher;

use super::*;

fn random_vector(size: i32) -> Vec<i32> {
    let numbers: Vec<i32> = (0..size).map(|_| rand::thread_rng().gen_range(0..20)).collect();
    numbers
}

#[test]
fn test_quick_sort() {
    let mut numbers = random_vector(100);
    quick_sort(&mut numbers);

    for i in 1..numbers.len()-1 {
        assert!(numbers[i] <= numbers[i+1]);
    }
}

#[test]
fn test_merge_sort() {
    let mut numbers = random_vector(100);
    merge_sort(&mut numbers);

    for i in 1..numbers.len()-1 {
        assert!(numbers[i] <= numbers[i+1]);
    }
}

#[bench]
fn bench_quick_sort(bench: &mut Bencher) {
    let mut numbers = random_vector(100);
    bench.iter(||{quick_sort(&mut numbers)});
}

#[bench]
fn bench_merge_sort(bench: &mut Bencher) {
    let mut numbers = random_vector(100);
    bench.iter(||{merge_sort(&mut numbers)});
}