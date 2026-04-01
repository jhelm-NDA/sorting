use std::time::Instant;
use rand::rngs::ThreadRng;
use rand::RngExt;

use crate::bubble;
use crate::selection;
use crate::insertion;
use crate::merge;
use crate::quick;
use crate::heap;


pub fn run_sort_benchmarks() {
    // Generate 20000 random numbers
    let mut rng = ThreadRng::default();

    let nums: Vec<i32> = (0..20_000)
        .map(|_| rand::rng().random_range(0..10_000)) // 2. Now this will work
        .collect();

    println!("--- Sorting Benchmark (20,000 random numbers) ---");

    // Bubble Sort
    let mut a = nums.clone();
    let start = Instant::now();
    bubble::bubble_sort(&mut a);
    let bubble_time = start.elapsed();

    // Selection Sort
    let mut b = nums.clone();
    let start = Instant::now();
    selection::selection_sort(&mut b);
    let selection_time = start.elapsed();

    // Insertion Sort
    let mut c = nums.clone();
    let start = Instant::now();
    insertion::insertion_sort(&mut c);
    let insertion_time = start.elapsed();

    // Merge Sort
    let mut d = nums.clone();
    let start = Instant::now();
    merge::merge_sort(&mut d);
    let merge_time = start.elapsed();

    // Quick Sort
    let mut e = nums.clone();
    let start = Instant::now();
    quick::quick_sort(&mut e);
    let quick_time = start.elapsed();

    // Heap Sort
    let mut f = nums.clone();
    let start = Instant::now();
    heap::heap_sort(&mut f);
    let heap_time = start.elapsed();

    println!("\n--- Results ---");
    println!("Bubble Sort:    {:?}", bubble_time);
    println!("Selection Sort: {:?}", selection_time);
    println!("Insertion Sort: {:?}", insertion_time);
    println!("Merge Sort:     {:?}", merge_time);
    println!("Quick Sort:     {:?}", quick_time);
    println!("Heap Sort:      {:?}", heap_time);
    
   
}
