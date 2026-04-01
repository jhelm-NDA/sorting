mod bubble;
mod selection;
mod insertion;
mod merge;
mod quick;
mod heap;
mod benchmark;

use rand::RngExt;

fn main() {

    let nums: Vec<i32> = (0..10)
        .map(|_| rand::rng().random_range(0..100)) // 2. Now this will work
        .collect();

    println!("Original: {:?}", nums);

    let mut a = nums.clone();
    bubble::bubble_sort(&mut a);
    println!("Bubble:   {:?}", a);

    let mut b = nums.clone();
    selection::selection_sort(&mut b);
    println!("Selection:{:?}", b);

    let mut c = nums.clone();
    insertion::insertion_sort(&mut c);
    println!("Insertion:{:?}", c);

    let mut d = nums.clone();
    merge::merge_sort(&mut d);
    println!("Merge:    {:?}", d);

    let mut e = nums.clone();
    quick::quick_sort(&mut e);
    println!("Quick:    {:?}", e);

    let mut f = nums.clone();
    heap::heap_sort(&mut f);
    println!("Heap:     {:?}", f);

    benchmark::run_sort_benchmarks();
}
