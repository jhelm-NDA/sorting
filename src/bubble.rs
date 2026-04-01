// Activity: Implement Bubble Sort
// TODO 1:
// Loop through the list repeatedly
// Swap adjacent elements if they are out of order
pub fn bubble_sort(nums: &mut [i32]) {
    let n = nums.len();
    for i in 0..n {
        for j in 0..n-1-i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }  
        }
    }

}