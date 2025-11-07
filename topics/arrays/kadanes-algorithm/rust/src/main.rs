/// Kadane's Algorithm - Maximum Subarray Sum
///
/// Find the contiguous subarray with the largest sum
/// Time Complexity: O(n)
/// Space Complexity: O(1)

fn kadanes_algorithm(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        panic!("Array must not be empty");
    }

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in &arr[1..] {
        // Either extend the existing subarray or start a new one
        current_sum = num.max(current_sum + num);

        // Update the maximum sum if current is larger
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

/// Extended version that also returns the subarray indices
fn kadanes_with_indices(arr: &[i32]) -> (i32, usize, usize) {
    if arr.is_empty() {
        panic!("Array must not be empty");
    }

    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;

    for i in 1..arr.len() {
        if arr[i] > current_sum + arr[i] {
            current_sum = arr[i];
            temp_start = i;
        } else {
            current_sum = current_sum + arr[i];
        }

        if current_sum > max_sum {
            max_sum = current_sum;
            start = temp_start;
            end = i;
        }
    }

    (max_sum, start, end)
}

fn main() {
    println!("=== Kadane's Algorithm Examples ===\n");

    // Example 1: Mixed positive and negative numbers
    let arr1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Array: {:?}", arr1);
    println!("Maximum Sum: {}", kadanes_algorithm(&arr1));
    let (max_sum, start, end) = kadanes_with_indices(&arr1);
    println!("Subarray: {:?}", &arr1[start..=end]);
    println!("Indices: [{}, {}]\n", start, end);

    // Example 2: All negative numbers
    let arr2 = vec![-3, -2, -5, -1, -4];
    println!("Array: {:?}", arr2);
    println!("Maximum Sum: {}", kadanes_algorithm(&arr2));
    let (max_sum, start, end) = kadanes_with_indices(&arr2);
    println!("Subarray: {:?}\n", &arr2[start..=end]);

    // Example 3: All positive numbers
    let arr3 = vec![1, 2, 3, 4, 5];
    println!("Array: {:?}", arr3);
    println!("Maximum Sum: {}", kadanes_algorithm(&arr3));
    let (max_sum, start, end) = kadanes_with_indices(&arr3);
    println!("Subarray: {:?}\n", &arr3[start..=end]);

    // Example 4: Single element
    let arr4 = vec![5];
    println!("Array: {:?}", arr4);
    println!("Maximum Sum: {}\n", kadanes_algorithm(&arr4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_numbers() {
        let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(kadanes_algorithm(&arr), 6);
    }

    #[test]
    fn test_all_negative() {
        let arr = vec![-3, -2, -5, -1, -4];
        assert_eq!(kadanes_algorithm(&arr), -1);
    }

    #[test]
    fn test_all_positive() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(kadanes_algorithm(&arr), 15);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![5];
        assert_eq!(kadanes_algorithm(&arr), 5);
    }

    #[test]
    fn test_with_indices() {
        let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let (max_sum, start, end) = kadanes_with_indices(&arr);
        assert_eq!(max_sum, 6);
        assert_eq!(start, 3);
        assert_eq!(end, 6);
    }
}
