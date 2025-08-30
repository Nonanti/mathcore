# Contributing to MathCore

First off, thank you for considering contributing to MathCore! It's people like you that make MathCore such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible using the issue template.

**Great Bug Reports** tend to have:
- A quick summary and/or background
- Steps to reproduce
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:
- Use a clear and descriptive title
- Provide a step-by-step description of the suggested enhancement
- Provide specific examples to demonstrate the steps
- Describe the current behavior and explain which behavior you expected to see instead
- Explain why this enhancement would be useful

### Your First Code Contribution

Unsure where to begin contributing? You can start by looking through these issues:
- `good first issue` - issues which should only require a few lines of code
- `help wanted` - issues which should be a bit more involved than beginner issues

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes (`cargo test`)
5. Make sure your code follows the style guidelines (`cargo fmt` and `cargo clippy`)
6. Issue that pull request!

## Development Process

### Setting Up Your Environment

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/mathcore.git
cd mathcore

# Add upstream remote
git remote add upstream https://github.com/Nonanti/mathcore.git

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Code Style

- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Follow Rust naming conventions:
  - Types: `PascalCase`
  - Functions/methods: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
  - Modules: `snake_case`

### Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting PR
- Add integration tests for complex features
- Include doc tests in your documentation

Example test:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_feature() {
        // Test implementation
        assert_eq!(2 + 2, 4);
    }
}
```

### Documentation

- Add documentation comments to all public items
- Include examples in documentation
- Update README.md if adding major features
- Update CHANGELOG.md following the Keep a Changelog format

Example documentation:
```rust
/// Computes the factorial of a number.
///
/// # Examples
///
/// ```
/// use mathcore::factorial;
/// 
/// assert_eq!(factorial(5), 120);
/// ```
pub fn factorial(n: u32) -> u32 {
    // Implementation
}
```

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Format:
```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect the code meaning
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Code change that improves performance
- `test`: Adding missing tests
- `chore`: Changes to the build process or auxiliary tools

### Performance Considerations

- Benchmark performance-critical code
- Avoid unnecessary allocations
- Use iterators when possible
- Consider using `&str` instead of `String` for function parameters
- Profile your code if adding complex algorithms

## Project Structure

```
mathcore/
├── src/
│   ├── lib.rs           # Library root
│   ├── types/           # Core types and structures
│   ├── parser/          # Expression parser
│   ├── engine/          # Evaluation engine
│   ├── calculus/        # Calculus operations
│   ├── solver/          # Equation solvers
│   ├── matrix/          # Matrix operations
│   ├── precision/       # Arbitrary precision
│   ├── ml/              # Optimization algorithms
│   └── differential/    # ODE/PDE solvers
├── tests/               # Integration tests
├── benches/             # Benchmarks
├── examples/            # Example usage
└── docs/                # Additional documentation
```

## Getting Help

- Check the documentation
- Search existing issues
- Ask in discussions
- Contact maintainers

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing!