// Activity: Implement Heap Sort
// TODO 6:
// 1. Build a max heap
// 2. Repeatedly swap root with last element
// 3. Heapify the reduced heap
pub fn heap_sort(nums: &mut [i32]) {
    let n = nums.len();

    //build max heap
    for i in (0..n / 2).rev() {
        heapify(nums, n, i);
    }

    //Extract elements
    for i in (1..n).rev() {
        nums.swap(0,1);
        heapify(nums, i, 0);
    }
}

fn heapify(nums: &mut [i32], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && nums[left] > nums[largest] {
        largest = left;
    }

    if right < heap_size && nums[right] > nums[largest] {
        largest = right;
    }

    if largest != root {
        nums.swap(root, largest);
        heapify(nums, heap_size, largest);
    }
}