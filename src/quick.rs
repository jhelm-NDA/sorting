// Activity: Implement Quick Sort
// TODO 5:
// 1. Choose a pivot
// 2. Partition into < pivot and > pivot
// 3. Recursively sort partitions
pub fn quick_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    quick(nums, 0, n as isize -1);
}

fn quick(nums: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition(nums, low, high);
        quick(nums, low, p - 1);
        quick(nums, p + 1, high);
    }
}

fn partition(nums: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = nums[high as usize];
    let mut i = low -1;

    for j in low..high {
        if nums[j as usize] <= pivot {
            i += 1;
            nums.swap(i as usize, j as usize);
        }
    }
    nums.swap((i + 1) as usize, high as usize);
   
    i + 1
}