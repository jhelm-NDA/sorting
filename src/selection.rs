// Activity: Implement Selection Sort
// TODO 2:
// For each index i:
//   Find the smallest element in the rest of the list
//   Swap it with index i
pub fn selection_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 0..n {
        let mut min_index = i;
        for j in i + 1..n {
            if nums[j] < nums[min_index] {
                min_index = j;
            }
        }
        nums.swap(i,min_index);
    }
}