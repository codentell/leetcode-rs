use pyo3::prelude::*;
use numpy::{PyReadonlyArray1};
mod array;
mod search;

#[pyfunction]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    array::two_sum::two_sum(nums, target)
}

#[pyfunction]
fn binary_search(nums: PyReadonlyArray1<i32>, target: i32) -> i32 {
    let nums_slice = nums.as_slice().unwrap();  
    search::binary_search::binary_search(nums_slice, target)
}


#[pymodule]
fn leetcode_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(two_sum, m)?)?;
    m.add_function(wrap_pyfunction!(binary_search, m)?)?;
    Ok(())
}