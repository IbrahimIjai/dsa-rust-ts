# Two Sum

**Difficulty:** Easy  
**Topic:** Arrays, Hash Table

## Problem Statement

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

## Example

```
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]
Explanation: nums[0] + nums[1] = 2 + 7 = 9
```

## Approaches

### 1. Brute Force (O(n²))
Check every pair of numbers

### 2. Hash Map (O(n))
Use a hash map to store numbers we've seen and their indices

## Complexity

- **Time Complexity:** O(n)
- **Space Complexity:** O(n)

## Implementations

- [JavaScript/TypeScript](./solution.ts)
- [Rust](./rust/src/main.rs)

## How to Run

### JavaScript/TypeScript
```bash
npm run dev topics/arrays/two-sum/solution.ts
```

### Rust
```bash
cargo run -p two-sum
```

## Related Problems

- Three Sum
- Four Sum
- Two Sum II (sorted array)
