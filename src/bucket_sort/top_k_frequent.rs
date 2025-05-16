// 692
pub fn top_k_frequent(nums: &[i32], k: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut counts: HashMap<i32, usize> = HashMap::new();

    for n in nums {
        *counts.entry(*n).or_insert(0) += 1
    }

    let max_freq = *counts.values().max().unwrap();

    let mut buckets: Vec<Vec<i32>> = vec![vec![]; max_freq + 1];

    for (num, freq) in counts {
        buckets[freq].push(num);
    }

    let mut result = vec![];
    for i in (0..=max_freq).rev() {
        for &num in &buckets[i] {
            result.push(num);
            if result.len() == k as usize {
                return result;
            }
        }
    }
    vec![]
}