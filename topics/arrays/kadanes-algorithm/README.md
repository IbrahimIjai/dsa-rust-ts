# Kadane's Algorithm

**Difficulty:** Medium  
**Topic:** Arrays, Dynamic Programming

## Problem Statement

Find the contiguous subarray within an array (containing at least one number) which has the largest sum.

## Example

```
Input: [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6
```

## Approach

Kadane's algorithm uses dynamic programming to solve the maximum subarray problem in O(n) time.

**Key Idea:**
- At each position, we decide whether to extend the current subarray or start a new one
- `current_sum = max(arr[i], current_sum + arr[i])`
- Track the maximum sum seen so far

## Complexity

- **Time Complexity:** O(n)
- **Space Complexity:** O(1)

## Implementations

- [JavaScript/TypeScript](./solution.js)
- [Rust](./rust/src/main.rs)

## How to Run

### JavaScript/TypeScript
```bash
npm run dev topics/arrays/kadanes-algorithm/solution.js
```

### Rust
```bash
cargo run -p kadanes-algorithm
```

## Related Problems

- Maximum Product Subarray
- Best Time to Buy and Sell Stock
