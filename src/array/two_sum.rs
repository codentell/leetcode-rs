pub fn two_sum(nums: Vec<i32>, target: i32 ) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    
    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - nums[i];

        if map.contains_key(&complement) {
            let j = map[&complement];
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}