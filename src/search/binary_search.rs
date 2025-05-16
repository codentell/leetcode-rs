pub fn binary_search(nums: &[i32], target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }
    
    
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid] == target {
            return mid as i32
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 { break; }
            right = mid - 1;
        }
    }
    -1
}