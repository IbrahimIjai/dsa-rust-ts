#!/bin/bash

# Script to create a new DSA problem structure
# Usage: ./new-problem.sh <category> <problem-name>

if [ $# -ne 2 ]; then
    echo "Usage: ./new-problem.sh <category> <problem-name>"
    echo "Example: ./new-problem.sh arrays binary-search"
    exit 1
fi

CATEGORY=$1
PROBLEM=$2
BASE_PATH="topics/$CATEGORY/$PROBLEM"

echo "Creating new problem: $PROBLEM in category: $CATEGORY"

# Create directory structure
mkdir -p "$BASE_PATH/rust/src"

# Copy template README
if [ -f "TEMPLATE.md" ]; then
    cp TEMPLATE.md "$BASE_PATH/README.md"
    echo "✓ Created README.md"
else
    echo "⚠ TEMPLATE.md not found, creating basic README"
    cat > "$BASE_PATH/README.md" << EOF
# ${PROBLEM^}

**Difficulty:** Medium  
**Topic:** $CATEGORY

## Problem Statement

[Add problem description here]

## Implementations

- [TypeScript](./solution.ts)
- [Rust](./rust/src/main.rs)

## How to Run

### TypeScript
\`\`\`bash
npm run dev topics/$CATEGORY/$PROBLEM/solution.ts
\`\`\`

### Rust
\`\`\`bash
cargo run -p $PROBLEM
\`\`\`
EOF
fi

# Create TypeScript solution template
cat > "$BASE_PATH/solution.ts" << 'EOF'
/**
 * [Problem Name]
 * 
 * [Brief description]
 * Time Complexity: O(?)
 * Space Complexity: O(?)
 */

function solution(input: any): any {
    // TODO: Implement solution
    return null;
}

// ============ TESTING ============

console.log("=== [Problem Name] Examples ===\n");

// Example 1
console.log("Example 1:");
console.log(solution([]));

export { solution };
EOF
echo "✓ Created solution.ts"

# Create Rust Cargo.toml
cat > "$BASE_PATH/rust/Cargo.toml" << EOF
[package]
name = "$PROBLEM"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "$PROBLEM"
path = "src/main.rs"
EOF
echo "✓ Created Cargo.toml"

# Create Rust main.rs template
cat > "$BASE_PATH/rust/src/main.rs" << 'EOF'
/// [Problem Name]
/// 
/// [Brief description]
/// Time Complexity: O(?)
/// Space Complexity: O(?)

fn solution(input: &[i32]) -> i32 {
    // TODO: Implement solution
    0
}

fn main() {
    println!("=== [Problem Name] Examples ===\n");

    // Example 1
    let input1 = vec![];
    println!("Example 1: {:?}", solution(&input1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![];
        assert_eq!(solution(&input), 0);
    }
}
EOF
echo "✓ Created main.rs"

echo ""
echo "✅ Successfully created problem structure at: $BASE_PATH"
echo ""
echo "Next steps:"
echo "1. Edit $BASE_PATH/README.md with problem details"
echo "2. Implement solution in $BASE_PATH/solution.ts"
echo "3. Implement solution in $BASE_PATH/rust/src/main.rs"
echo "4. Run TypeScript: npm run dev topics/$CATEGORY/$PROBLEM/solution.ts"
echo "5. Run Rust: cargo run -p $PROBLEM"
echo "6. Update main README.md with the new problem"
