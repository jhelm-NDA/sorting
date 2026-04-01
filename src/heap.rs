pub fn heap_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }

    // Build max heap
    // Start from the last non-leaf node: (n / 2) - 1
    for i in (0..=(n / 2).saturating_sub(1)).rev() {
        heapify(nums, n, i);
    }

    // One by one extract elements from heap
    for i in (1..n).rev() {
        nums.swap(0, i);          // Move current root to end
        heapify(nums, i, 0);      // Heapify reduced heap
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
