import timeit
from collections import Counter
import leetcode_rs
import numpy as np

# ✅ Pure Python two_sum
def two_sum_py(nums, target):
    lookup = {}
    for i, num in enumerate(nums):
        if target - num in lookup:
            return [lookup[target - num], i]
        lookup[num] = i
    return []

# ✅ Pure Python binary_search
def binary_search_py(nums, target):
    left, right = 0, len(nums) - 1
    while left <= right:
        mid = (left + right) // 2
        if nums[mid] == target:
            return mid
        elif nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1

def top_k_frequent_py(nums, k):
    freq = Counter(nums)
    buckets = [[] for _ in range(max(freq.values()) + 1)]
    for num, count in freq.items():
        buckets[count].append(num)

    result = []
    for i in range(len(buckets) - 1, -1, -1):
        for val in buckets[i]:
            result.append(val)
            if len(result) == k:
                return result
    return result

# ✅ Test Data
nums_tkf = np.array([1, 1, 1, 2, 2, 3], dtype=np.int32)
k = 2

# ✅ Setup data
nums_small = [2, 7, 11, 15]
nums_large = np.array(range(100_000), dtype=np.int32) 
target_ts = 9
target_bs = 99_999

# ✅ Warm-up
print("Rust two_sum:", leetcode_rs.two_sum(nums_small, target_ts))
print("Rust binary_search:", leetcode_rs.binary_search(nums_large, target_bs))
print("Python two_sum:", two_sum_py(nums_small, target_ts))
print("Python binary_search:", binary_search_py(nums_large, target_bs))

# ✅ Benchmark two_sum
print("\n--- Two Sum ---")
rust_ts = timeit.timeit(lambda: leetcode_rs.two_sum(nums_small, target_ts), number=10000)
py_ts = timeit.timeit(lambda: two_sum_py(nums_small, target_ts), number=10000)
print(f"Rust:  {rust_ts:.6f} sec total, {rust_ts/10000:.9f} sec per call")
print(f"Python: {py_ts:.6f} sec total, {py_ts/10000:.9f} sec per call")

# ✅ Benchmark binary_search
print("\n--- Binary Search ---")
rust_bs = timeit.timeit(lambda: leetcode_rs.binary_search(nums_large, target_bs), number=10000)
py_bs = timeit.timeit(lambda: binary_search_py(nums_large, target_bs), number=10000)
print(f"Rust:  {rust_bs:.6f} sec total, {rust_bs/10000:.9f} sec per call")
print(f"Python: {py_bs:.6f} sec total, {py_bs/10000:.9f} sec per call")

# ✅ Benchmark top_k_frequent
print("\n--- Top K Frequent ---")
rust_tkf = timeit.timeit(lambda: leetcode_rs.top_k_frequent(nums_tkf, k), number=10000)
py_tkf = timeit.timeit(lambda: top_k_frequent_py(nums_tkf, k), number=10000)
print(f"Rust:  {rust_tkf:.6f} sec total, {rust_tkf/10000:.9f} sec per call")
print(f"Python: {py_tkf:.6f} sec total, {py_tkf/10000:.9f} sec per call")
