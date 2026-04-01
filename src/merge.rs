// Activity: Implement Merge Sort
// TODO 4:
// 1. Split the list into halves
// 2. Recursively sort each half
// 3. Merge the sorted halves
pub fn merge_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    let mut left = nums[..mid].to_vec();
    let mut right = nums[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(nums, &left, &right);
}

fn merge(nums: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            nums[k] = left[i];
            i += 1;
        } else {
            nums[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        nums[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        nums[k] = right[j];
        j += 1;
        k += 1;
    }
}