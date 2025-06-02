use pyo3::prelude::*;
use numpy::{PyReadonlyArray1};
mod array;
mod search;
mod bucket_sort;

// 0001
#[pyfunction]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    array::two_sum::two_sum(nums, target)
}

#[pyfunction]
fn binary_search(nums: PyReadonlyArray1<i32>, target: i32) -> i32 {
    let nums_slice = nums.as_slice().unwrap();  
    search::binary_search::binary_search(nums_slice, target)
}

#[pyfunction]
fn top_k_frequent(nums: PyReadonlyArray1<i32>, k: i32) -> Vec<i32> {
    let nums_slice = nums.as_slice().unwrap();      
    bucket_sort::top_k_frequent::top_k_frequent(nums_slice, k)
}


#[pymodule]
fn leetcode_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_sum, m)?)?;
    m.add_function(wrap_pyfunction!(top_k_frequent,m)?)?;
    m.add_function(wrap_pyfunction!(binary_search, m)?)?;
    Ok(())
}