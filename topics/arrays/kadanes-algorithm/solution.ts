/**
 * Kadane's Algorithm - Maximum Subarray Sum
 *
 * Find the contiguous subarray with the largest sum
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */

function kadanesAlgorithm(arr: number[]): number {
  if (arr.length === 0) {
    throw new Error("Array must not be empty");
  }

  let maxSum = arr[0];
  let currentSum = arr[0];

  for (let i = 1; i < arr.length; i++) {
    // Either extend the existing subarray or start a new one
    currentSum = Math.max(arr[i], currentSum + arr[i]);

    // Update the maximum sum if current is larger
    maxSum = Math.max(maxSum, currentSum);
  }

  return maxSum;
}

/**
 * Extended version that also returns the subarray indices
 */
function kadanesWithIndices(arr: number[]): {
  maxSum: number;
  start: number;
  end: number;
} {
  if (arr.length === 0) {
    throw new Error("Array must not be empty");
  }

  let maxSum = arr[0];
  let currentSum = arr[0];
  let start = 0;
  let end = 0;
  let tempStart = 0;

  for (let i = 1; i < arr.length; i++) {
    if (arr[i] > currentSum + arr[i]) {
      currentSum = arr[i];
      tempStart = i;
    } else {
      currentSum = currentSum + arr[i];
    }

    if (currentSum > maxSum) {
      maxSum = currentSum;
      start = tempStart;
      end = i;
    }
  }

  return { maxSum, start, end };
}

// ============ TESTING ============

console.log("=== Kadane's Algorithm Examples ===\n");

// Example 1: Mixed positive and negative numbers
const arr1 = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
console.log(`Array: [${arr1}]`);
console.log(`Maximum Sum: ${kadanesAlgorithm(arr1)}`);
const result1 = kadanesWithIndices(arr1);
console.log(`Subarray: [${arr1.slice(result1.start, result1.end + 1)}]`);
console.log(`Indices: [${result1.start}, ${result1.end}]\n`);

// Example 2: All negative numbers
const arr2 = [-3, -2, -5, -1, -4];
console.log(`Array: [${arr2}]`);
console.log(`Maximum Sum: ${kadanesAlgorithm(arr2)}`);
const result2 = kadanesWithIndices(arr2);
console.log(`Subarray: [${arr2.slice(result2.start, result2.end + 1)}]\n`);

// Example 3: All positive numbers
const arr3 = [1, 2, 3, 4, 5];
console.log(`Array: [${arr3}]`);
console.log(`Maximum Sum: ${kadanesAlgorithm(arr3)}`);
const result3 = kadanesWithIndices(arr3);
console.log(`Subarray: [${arr3.slice(result3.start, result3.end + 1)}]\n`);

// Example 4: Single element
const arr4 = [5];
console.log(`Array: [${arr4}]`);
console.log(`Maximum Sum: ${kadanesAlgorithm(arr4)}\n`);

export { kadanesAlgorithm, kadanesWithIndices };
