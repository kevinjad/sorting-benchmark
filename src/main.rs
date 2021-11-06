#![feature(test)]

use rand::Rng;

mod tests;
fn main() {
    let mut numbers = vec![5, 4, 7, 9, 6, 1];
    merge_sort(&mut numbers);
    println!("{:?}", numbers);
}

fn swap(numbers: &mut Vec<i32>, part_idx: usize, end: usize) {
    numbers.swap(part_idx, end);
}

fn partition(numbers: &mut Vec<i32>, start: usize, end: usize, part_idx: usize) -> usize {
    let part_value = numbers[part_idx];
    swap(numbers, part_idx, end - 1);
    let mut i = start;
    for j in start..(end - 1) {
        if numbers[j] < part_value {
            swap(numbers, i, j);
            i = i + 1;
        }
    }
    swap(numbers, i, end - 1);
    i
}

fn quick_sort_helper(numbers: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mut part_idx = rand::thread_rng().gen_range(start..end);
    part_idx = partition(numbers, start, end, part_idx);
    quick_sort_helper(numbers, start, part_idx);
    quick_sort_helper(numbers, part_idx + 1, end);
}

fn quick_sort(numbers: &mut Vec<i32>) {
    quick_sort_helper(numbers, 0, numbers.len());
}

fn merge(numbers: &mut Vec<i32>, start: usize, end: usize, mid: usize) {
    let mut i = start;
    let mut j = mid+1;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    while i <= mid {
        left.push(numbers[i]);
        i = i + 1;
    }

    while j <= end {
        right.push(numbers[j]);
        j = j + 1;
    }

    i=0;
    j=0;
    let mut k = start;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            numbers[k] = left[i];
            i = i + 1;
        }
        else {
            numbers[k] = right[j];
            j = j+1;
        }
        k = k+1;
    }
    if i < left.len() {
        while i < left.len() {
            numbers[k] = left[i];
            i  = i +1;
            k = k + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            numbers[k] = right[j];
            j = j +1;
            k = k + 1;
        }
    }
}

fn merge_sort_helper(numbers: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
    let mid = (start + end)/2;
    merge_sort_helper(numbers, start, mid);
    merge_sort_helper(numbers, mid+1, end);
    merge(numbers, start, end, mid);
    }
}

fn merge_sort(numbers: &mut Vec<i32>) {
    merge_sort_helper(numbers, 0, numbers.len()-1)
}