// 268. Missing Number
pub fn missing_number(mut nums: Vec<i32>) -> i32 {
    let length = nums.len();

    nums.sort();

    for i in 0..nums.len() {
        if (i as i32) != (nums[i] as i32)  {
            return i as i32;
        }
    }
    return length as i32;
}