// Activity: Implement Insertion Sort
// TODO 3:
// For each element, shift larger elements right
// Insert the element into the correct position
pub fn insertion_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 1..n {
        let key = nums[i];
        let mut j = i;

        while j > 0 && nums[j - 1] > key {
            nums[j] = nums[j - 1];
            j -=1;
        }

        nums[j] = key;
    }
}