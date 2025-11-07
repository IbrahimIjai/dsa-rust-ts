/// Two Sum Problem
///
/// Find two numbers in an array that add up to a target sum
/// Time Complexity: O(n)
/// Space Complexity: O(n)
use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = num_map.get(&complement) {
            return vec![index, i];
        }

        num_map.insert(num, i);
    }

    vec![] // No solution found
}

/// Brute Force Approach - O(n²)
/// Useful for understanding the problem
fn two_sum_brute_force(nums: &[i32], target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    vec![]
}

fn main() {
    println!("=== Two Sum Examples ===\n");

    // Example 1
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    println!("Array: {:?}, Target: {}", nums1, target1);
    let result1 = two_sum(&nums1, target1);
    println!("Result: {:?}", result1);
    println!(
        "Values: {} + {} = {}\n",
        nums1[result1[0]],
        nums1[result1[1]],
        nums1[result1[0]] + nums1[result1[1]]
    );

    // Example 2
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    println!("Array: {:?}, Target: {}", nums2, target2);
    let result2 = two_sum(&nums2, target2);
    println!("Result: {:?}", result2);
    println!(
        "Values: {} + {} = {}\n",
        nums2[result2[0]],
        nums2[result2[1]],
        nums2[result2[0]] + nums2[result2[1]]
    );

    // Example 3
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("Array: {:?}, Target: {}", nums3, target3);
    println!("Result: {:?}\n", two_sum(&nums3, target3));

    // Example 4 - Negative numbers
    let nums4 = vec![-1, -2, -3, -4, -5];
    let target4 = -8;
    println!("Array: {:?}, Target: {}", nums4, target4);
    let result4 = two_sum(&nums4, target4);
    println!("Result: {:?}", result4);
    println!(
        "Values: {} + {} = {}\n",
        nums4[result4[0]],
        nums4[result4[1]],
        nums4[result4[0]] + nums4[result4[1]]
    );

    // Performance comparison
    println!("=== Performance Comparison ===");
    let large_array: Vec<i32> = (0..10000).collect();
    let large_target = 19997;

    use std::time::Instant;

    let start = Instant::now();
    two_sum(&large_array, large_target);
    println!("Hash Map Approach: {:?}", start.elapsed());

    let start = Instant::now();
    two_sum_brute_force(&large_array, large_target);
    println!("Brute Force Approach: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(&nums, target), vec![0, 1]);
    }

    #[test]
    fn test_multiple_solutions() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(&nums, target), vec![1, 2]);
    }

    #[test]
    fn test_duplicate_numbers() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(&nums, target), vec![0, 1]);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        assert_eq!(two_sum(&nums, target), vec![2, 4]);
    }
}
