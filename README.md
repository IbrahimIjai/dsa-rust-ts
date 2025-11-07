# 🚀 DSA Learning Journey

A comprehensive collection of **Data Structures and Algorithms** implementations in both **Rust** and **TypeScript/JavaScript**. Perfect for learning, practicing, and mastering DSA concepts with side-by-side language comparisons.

## 📚 Documentation

- **[🎉 GETTING STARTED](./GETTING_STARTED.md)** - Start here! Complete setup guide
- **[🗺️ ROADMAP](./ROADMAP.md)** - Visual learning path with progress tracking
- **[🎓 LEARNING GUIDE](./LEARNING_GUIDE.md)** - Detailed weekly schedule & tips
- **[🚀 QUICK START](./QUICK_START.md)** - Command reference & troubleshooting
- **[📝 TEMPLATE](./TEMPLATE.md)** - Template for adding new problems

---

## 📚 New Structure - Topic-Based Learning

```
topics/
├── arrays/
│   ├── kadanes-algorithm/
│   │   ├── README.md          # Problem explanation
│   │   ├── solution.ts        # TypeScript implementation
│   │   └── rust/
│   │       ├── Cargo.toml
│   │       └── src/main.rs    # Rust implementation
│   ├── two-sum/
│   └── ...
├── linked-lists/
├── trees/
├── graphs/
├── recursion/
├── dynamic-programming/
└── ...
```

## 🎯 Learning Path

### 1. **Arrays & Strings** (Start here!)
- [x] Two Sum - `topics/arrays/two-sum/`
- [x] Kadane's Algorithm (Maximum Subarray) - `topics/arrays/kadanes-algorithm/`
- [ ] Sliding Window Maximum
- [ ] Product of Array Except Self
- [ ] Container With Most Water

### 2. **Linked Lists**
- [ ] Reverse Linked List
- [ ] Detect Cycle
- [ ] Merge Two Sorted Lists
- [ ] Remove Nth Node From End

### 3. **Stacks & Queues**
- [ ] Valid Parentheses
- [ ] Min Stack
- [ ] Implement Queue using Stacks

### 4. **Trees**
- [ ] Binary Tree Traversal (Inorder, Preorder, Postorder)
- [ ] Maximum Depth of Binary Tree
- [ ] Lowest Common Ancestor
- [ ] Validate BST

### 5. **Graphs**
- [ ] DFS & BFS
- [ ] Number of Islands
- [ ] Course Schedule
- [ ] Dijkstra's Algorithm

### 6. **Dynamic Programming**
- [ ] Fibonacci Numbers
- [ ] Climbing Stairs
- [ ] Longest Common Subsequence
- [ ] Knapsack Problem

### 7. **Sorting & Searching**
- [ ] Binary Search
- [ ] Quick Sort
- [ ] Merge Sort
- [ ] Heap Sort

## 🚀 Quick Start

### Prerequisites

**For TypeScript/JavaScript:**
```bash
npm install
```

**For Rust:**
- Install Rust: https://rustup.rs/

### Running Examples

#### TypeScript/JavaScript
```bash
# Run any TypeScript solution
npm run dev topics/arrays/kadanes-algorithm/solution.ts

# Or use ts-node directly
npx ts-node topics/arrays/two-sum/solution.ts
```

#### Rust
```bash
# Run any Rust solution by package name
cargo run -p kadanes-algorithm
cargo run -p two-sum

# Run with tests
cargo test -p kadanes-algorithm

# Run all tests
cargo test --workspace
```

### Testing Your Understanding

Each problem includes:
- 📖 **README.md** - Problem statement, examples, complexity analysis
- 💻 **solution.ts** - TypeScript implementation with examples
- 🦀 **rust/src/main.rs** - Rust implementation with examples
- ✅ **Tests** - Unit tests in both languages

## 📖 How to Use This Repository

### For Learning
1. **Read the problem** in the topic's README
2. **Try to solve it yourself** first
3. **Compare** your solution with the provided implementations
4. **Run the code** to see it in action
5. **Modify and experiment** to deepen understanding

### For Practice
1. Pick a topic from the learning path
2. Implement in your preferred language first
3. Then implement in the other language
4. Compare both approaches and learn language-specific patterns

### For Interview Prep
1. Focus on common patterns (sliding window, two pointers, etc.)
2. Practice explaining complexity analysis
3. Implement both brute force and optimized solutions
4. Run tests to verify correctness

## 💡 Tips

1. **Don't rush** - Understand the 'why' behind each algorithm
2. **Compare languages** - See how Rust and TypeScript approach the same problem
3. **Run the code** - Don't just read, execute and experiment
4. **Write tests** - Validate your understanding
5. **Explain out loud** - If you can teach it, you know it

## 🌟 Why Both Languages?

**TypeScript/JavaScript:**
- High-level, expressive syntax
- Great for interviews and web development
- Quick prototyping and iteration

**Rust:**
- Systems programming mindset
- Memory safety and performance
- Teaches low-level concepts
- Growing in popularity for backend and systems

Learning both gives you:
- Broader perspective on problem-solving
- Better understanding of time/space tradeoffs
- Appreciation for different programming paradigms

---

## 📚 Reference Library (Legacy)

The `resources/` folder contains additional algorithm and data structure implementations for reference.

## Algorithms

[Damerau Levenshtein Distance](./algorithms/damerau-levenshtein-distance/index.ts)

[Euler's Totient](./algorithms/eulers-totient/index.ts)

[Fibonacci Numbers](./algorithms/fibonacci-numbers/index.ts)

[Fisher–Yates Shuffle](./algorithms/fisher-yates-shuffle/index.ts)

[Levenshtein Distance](./algorithms/levenshtein-distance/index.ts)

[Linear Congruential Generator](./algorithms/linear-congruential-generator/index.ts)

[Lucas Numbers](./algorithms/lucas-numbers/index.ts)

[Mersenne Twister](./algorithms/mersenne-twister/index.ts)

[Phi](./algorithms/phi/index.ts)

[Sieve of Eratosthenes](./algorithms/sieve-of-eratosthenes/index.ts)

### Search Algorithms

[Binary Search](./algorithms/binary-search/index.ts)

[Exponential Search](./algorithms/exponential-search/index.ts)

[Interpolation Search](./algorithms/interpolation-search/index.ts)

[Linear Search](./algorithms/linear-search/index.ts)

[Ternary Search](./algorithms/ternary-search/index.ts)

### Hash Algorithms

[djb2](./algorithms/hash/djb2/index.ts)

[sdbm](./algorithms/hash/sdbm/index.ts)

[loselose](./algorithms/hash/loselose/index.ts)

### Sort Algorithms

[Bubble Sort](./algorithms/sorting/bubble-sort/index.ts)

[Comb Sort](./algorithms/sorting/comb-sort/index.ts)

[Heapsort](./algorithms/sorting/heapsort/index.ts)

[Insertion Sort](./algorithms/sorting/insertion-sort/index.ts)

[Merge Sort](./algorithms/sorting/merge-sort/index.ts)

[Quicksort](./algorithms/sorting/quicksort/index.ts)

[Selection Sort](./algorithms/sorting/selection-sort/index.ts)

[Shellsort](./algorithms/sorting/shellsort/index.ts)

## Data Structures

[AVL Tree](./data-structures/avl-tree/index.ts)

[Binary Search Tree](./data-structures/binary-search-tree/index.ts)

[Binary Tree](./data-structures/binary-tree/index.ts)

[Circular Doubly Linked List](./data-structures/circular-doubly-linked-list/index.ts)

[Circular Linked List](./data-structures/circular-linked-list/index.ts)

[Deque](./data-structures/deque/index.ts)

[Dictionary](./data-structures/dictionary/index.ts)

[Doubly Linked List](./data-structures/doubly-linked-list/index.ts)

[Graph](./data-structures/graph/index.ts)

[Hash Table](./data-structures/hash-table/index.ts)

[Linked List](./data-structures/linked-list/index.ts)

[Queue](./data-structures/queue/index.ts)

[Red Black Tree](./data-structures/red-black-tree/index.ts)

[Skip List](./data-structures/skip-list/index.ts)

[Stack](./data-structures/stack/index.ts)

[Tree](./data-structures/tree/index.ts)

[Trie](./data-structures/trie/index.ts)

[Unrolled Linked List](./data-structures/unrolled-linked-list/index.ts)

## Licence

MIT
