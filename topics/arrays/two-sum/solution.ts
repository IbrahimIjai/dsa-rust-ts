/**
 * Two Sum Problem
 * 
 * Find two numbers in an array that add up to a target sum
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */

function twoSum(nums: number[], target: number): number[] {
    const numMap = new Map<number, number>();

    for (let i = 0; i < nums.length; i++) {
        const complement = target - nums[i];
        
        if (numMap.has(complement)) {
            return [numMap.get(complement)!, i];
        }
        
        numMap.set(nums[i], i);
    }

    return []; // No solution found
}

/**
 * Brute Force Approach - O(n²)
 * Useful for understanding the problem
 */
function twoSumBruteForce(nums: number[], target: number): number[] {
    for (let i = 0; i < nums.length; i++) {
        for (let j = i + 1; j < nums.length; j++) {
            if (nums[i] + nums[j] === target) {
                return [i, j];
            }
        }
    }
    return [];
}

// ============ TESTING ============

console.log("=== Two Sum Examples ===\n");

// Example 1
const nums1 = [2, 7, 11, 15];
const target1 = 9;
console.log(`Array: [${nums1}], Target: ${target1}`);
console.log(`Result: [${twoSum(nums1, target1)}]`);
console.log(`Values: ${nums1[0]} + ${nums1[1]} = ${nums1[0] + nums1[1]}\n`);

// Example 2
const nums2 = [3, 2, 4];
const target2 = 6;
console.log(`Array: [${nums2}], Target: ${target2}`);
const result2 = twoSum(nums2, target2);
console.log(`Result: [${result2}]`);
console.log(`Values: ${nums2[result2[0]]} + ${nums2[result2[1]]} = ${nums2[result2[0]] + nums2[result2[1]]}\n`);

// Example 3
const nums3 = [3, 3];
const target3 = 6;
console.log(`Array: [${nums3}], Target: ${target3}`);
console.log(`Result: [${twoSum(nums3, target3)}]\n`);

// Example 4 - Negative numbers
const nums4 = [-1, -2, -3, -4, -5];
const target4 = -8;
console.log(`Array: [${nums4}], Target: ${target4}`);
const result4 = twoSum(nums4, target4);
console.log(`Result: [${result4}]`);
console.log(`Values: ${nums4[result4[0]]} + ${nums4[result4[1]]} = ${nums4[result4[0]] + nums4[result4[1]]}\n`);

// Compare performance
console.log("=== Performance Comparison ===");
const largeArray = Array.from({ length: 10000 }, (_, i) => i);
const largeTarget = 19997;

console.time("Hash Map Approach");
twoSum(largeArray, largeTarget);
console.timeEnd("Hash Map Approach");

console.time("Brute Force Approach");
twoSumBruteForce(largeArray, largeTarget);
console.timeEnd("Brute Force Approach");

export { twoSum, twoSumBruteForce };
