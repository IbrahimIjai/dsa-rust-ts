# Quick Reference Guide

## Running Code

### TypeScript/JavaScript
```bash
# Install dependencies first
npm install

# Run any solution
npm run dev topics/arrays/kadanes-algorithm/solution.ts
npx ts-node topics/arrays/two-sum/solution.ts

# Direct node execution (if already compiled)
node topics/arrays/kadanes-algorithm/solution.js
```

### Rust
```bash
# Run a specific problem
cargo run -p kadanes-algorithm
cargo run -p two-sum

# Run with release optimizations (faster)
cargo run --release -p kadanes-algorithm

# Run tests for a specific problem
cargo test -p kadanes-algorithm

# Run all tests
cargo test --workspace

# Run tests with output
cargo test -- --nocapture
```

## Adding a New Problem

### Step 1: Create folder structure
```bash
mkdir -p topics/<category>/<problem-name>/rust/src
```

### Step 2: Create README.md
Copy from `TEMPLATE.md` or use this structure:
- Problem statement
- Examples
- Approach explanation
- Complexity analysis
- Links to implementations

### Step 3: Create TypeScript solution
File: `topics/<category>/<problem-name>/solution.ts`
- Function implementation
- Test cases
- Example usage

### Step 4: Create Rust solution
File: `topics/<category>/<problem-name>/rust/src/main.rs`
- Function implementation
- Main function with examples
- Unit tests in `#[cfg(test)]` module

### Step 5: Create Cargo.toml
File: `topics/<category>/<problem-name>/rust/Cargo.toml`
```toml
[package]
name = "problem-name"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "problem-name"
path = "src/main.rs"
```

### Step 6: Update main README
Add your problem to the learning path checklist.

## Common Commands

### Project Setup
```bash
# Install Node dependencies
npm install

# Check Rust installation
rustc --version
cargo --version
```

### Development
```bash
# Run TypeScript in watch mode (requires nodemon)
npm install -g nodemon
nodemon --exec ts-node topics/arrays/kadanes-algorithm/solution.ts

# Format Rust code
cargo fmt --all

# Lint Rust code
cargo clippy --all
```

### Testing
```bash
# TypeScript (if you set up Jest)
npm test

# Rust - all tests
cargo test --workspace

# Rust - specific package
cargo test -p kadanes-algorithm

# Rust - with output
cargo test -- --nocapture
```

## File Naming Conventions

- **Folders**: kebab-case (e.g., `two-sum`, `kadanes-algorithm`)
- **TypeScript files**: camelCase or kebab-case (e.g., `solution.ts`)
- **Rust files**: snake_case (e.g., `main.rs`)
- **Package names**: kebab-case in Cargo.toml

## Tips & Tricks

### TypeScript
```bash
# Run with specific Node version (using nvm)
nvm use 20
npm run dev topics/arrays/two-sum/solution.ts

# Compile TypeScript to JavaScript
npx tsc

# Run compiled JavaScript
node dist/topics/arrays/two-sum/solution.js
```

### Rust
```bash
# Check if code compiles without building
cargo check

# Build without running
cargo build

# Build with optimizations
cargo build --release

# Clean build artifacts
cargo clean

# See dependency tree
cargo tree

# Update dependencies
cargo update
```

### Performance Testing

**Rust:**
```bash
# Run with release optimizations for accurate timing
cargo run --release -p kadanes-algorithm

# Use cargo-criterion for benchmarking (install first)
cargo install cargo-criterion
cargo criterion
```

**TypeScript:**
```javascript
// Use console.time in your code
console.time('Algorithm');
// ... your code
console.timeEnd('Algorithm');
```

## Troubleshooting

### TypeScript Issues

**Cannot find module 'ts-node'**
```bash
npm install ts-node typescript @types/node --save-dev
```

**Type errors**
```bash
# Check tsconfig.json is properly configured
npx tsc --noEmit
```

### Rust Issues

**Package not found**
```bash
# Make sure Cargo.toml includes the workspace member
# Check that rust/Cargo.toml has the correct members path
cargo clean
cargo build
```

**Compilation errors**
```bash
# Get more detailed error messages
cargo build --verbose

# Check formatting
cargo fmt --check
```

## Useful Resources

- **TypeScript**: https://www.typescriptlang.org/docs/
- **Rust**: https://doc.rust-lang.org/book/
- **Cargo**: https://doc.rust-lang.org/cargo/
- **ts-node**: https://typestrong.org/ts-node/

## Environment Setup

### VS Code Extensions (Recommended)
- **Rust**: rust-analyzer
- **TypeScript**: ESLint, Prettier
- **General**: GitLens, Error Lens

### Shell Aliases (Optional)
Add to your `.bashrc` or `.zshrc`:
```bash
# Quick run aliases
alias tsrun='npm run dev'
alias rustrun='cargo run -p'
alias rusttest='cargo test -p'
```

Usage:
```bash
tsrun topics/arrays/kadanes-algorithm/solution.ts
rustrun kadanes-algorithm
rusttest two-sum
```
