// 912
pub fn sort_an_array(nums: &[i32]) -> Vec<i32> {

    if nums.is_empty() {
        return nums.to_vec();
    }
    
    let min_val = *nums.iter().min().unwrap();
    let max_val = *nums.iter().max().unwrap();
    let range = (max_val - min_val + 1) as usize;
    
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; range];
   
    for &num in nums {
        let index = (num - min_val) as usize;
        buckets[index].push(num);
    }

    let mut results = vec![];
    for i in 0..buckets.len() {
        for j in buckets[i].clone() {
            results.push(j);
        }
    }
    results
}


