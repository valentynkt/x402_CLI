# Contributing to x402-dev

Thank you for your interest in contributing to x402-dev! We welcome contributions from developers of all skill levels.

---

## 🚀 Quick Start

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/x402-dev
cd x402-dev

# 2. Build the project
cargo build

# 3. Run tests
cargo test

# 4. Make your changes
# ... edit code ...

# 5. Test your changes
cargo test
cargo clippy
cargo fmt

# 6. Submit a pull request
git push origin your-branch-name
```

---

## 🎯 Ways to Contribute

### 🐛 Report Bugs
Found a bug? [Open an issue](https://github.com/valentynkit/x402-dev/issues/new) with:
- Clear title describing the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version)
- Error messages or logs

### 💡 Suggest Features
Have an idea? [Start a discussion](https://github.com/valentynkit/x402-dev/discussions/new) with:
- Problem statement (what pain point does this solve?)
- Proposed solution
- Example use cases
- Alternatives you've considered

### 📝 Improve Documentation
Documentation improvements are always welcome:
- Fix typos or unclear wording
- Add examples or code snippets
- Improve error messages
- Translate docs (future)

### 🔧 Submit Code
Ready to code? Great! Follow the workflow below.

---

## 🛠️ Development Workflow

### Prerequisites

- **Rust 1.75+** - Install from [rustup.rs](https://rustup.rs)
- **Git** - Version control
- **Code editor** - VS Code with rust-analyzer recommended

### Setup

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/x402-dev
   cd x402-dev
   ```

3. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/valentynkit/x402-dev
   ```

4. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Making Changes

1. **Write clean code:**
   - Follow Rust naming conventions
   - Add comments for complex logic
   - Keep functions small and focused
   - Use descriptive variable names

2. **Follow KISS & YAGNI principles:**
   - **KISS** (Keep It Simple, Stupid) - Prefer simple solutions over clever ones
   - **YAGNI** (You Aren't Gonna Need It) - Don't add features speculatively

3. **Add tests:**
   ```bash
   # Unit tests in same file
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_your_feature() {
           // Test code here
       }
   }
   ```

4. **Run the test suite:**
   ```bash
   cargo test
   ```

5. **Format your code:**
   ```bash
   cargo fmt
   ```

6. **Check for lints:**
   ```bash
   cargo clippy -- -D warnings
   ```

### Committing

1. **Write clear commit messages:**
   ```
   feat(cli): add --timeout flag to check command

   - Add timeout parameter to prevent hanging
   - Default to 10 seconds
   - Update help text and examples
   ```

   **Format:** `<type>(<scope>): <description>`

   **Types:**
   - `feat` - New feature
   - `fix` - Bug fix
   - `docs` - Documentation only
   - `test` - Adding tests
   - `refactor` - Code refactoring
   - `perf` - Performance improvement
   - `chore` - Build/tooling changes

2. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat(cli): add --timeout flag to check command"
   ```

### Submitting

1. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a pull request:**
   - Go to GitHub and click "New Pull Request"
   - Fill out the PR template
   - Link related issues
   - Request review

3. **Respond to feedback:**
   - Address review comments
   - Push additional commits if needed
   - Be patient and respectful

---

## ✅ Code Quality Standards

### Testing Requirements

- **All new features** must include tests
- **Bug fixes** should include regression tests
- **Aim for >80% test coverage** on new code
- **Run full test suite** before submitting PR

### Code Style

- **Follow `rustfmt`** - Run `cargo fmt` before committing
- **Pass `clippy`** - Fix all warnings: `cargo clippy -- -D warnings`
- **Use meaningful names** - `user_count` not `uc`
- **Keep functions under 50 lines** - Split larger functions
- **Document public APIs** - Add doc comments with examples

### Documentation

- **Update README.md** if user-facing changes
- **Add doc comments** for public functions:
  ```rust
  /// Validates x402 protocol compliance for an endpoint
  ///
  /// # Arguments
  /// * `url` - The API endpoint to check
  ///
  /// # Returns
  /// * `Ok(ValidationResult)` - Compliance check results
  /// * `Err(CliError)` - Network or parse errors
  ///
  /// # Example
  /// ```
  /// let result = check_endpoint("http://localhost:3402")?;
  /// assert!(result.is_valid);
  /// ```
  pub fn check_endpoint(url: &str) -> Result<ValidationResult>
  ```

- **Update CHANGELOG.md** for user-visible changes
- **Add examples** for new commands

---

## 🐛 Reporting Security Issues

**Do NOT** open public issues for security vulnerabilities.

Email security concerns to: security@x402-dev.com (or maintainer email)

Include:
- Description of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We'll respond within 48 hours.

---

## 📋 PR Checklist

Before submitting, ensure:

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] No warnings: `cargo clippy -- -D warnings`
- [ ] Formatted: `cargo fmt`
- [ ] Documentation updated (if needed)
- [ ] CHANGELOG.md updated (if user-facing)
- [ ] Commit messages follow convention
- [ ] PR description explains changes
- [ ] Linked related issues

---

## 🎨 Project Structure

```
x402-dev/
├── crates/
│   ├── x402-cli/          # CLI binary
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs     # Command definitions
│   │   │   ├── commands/  # Command implementations
│   │   │   └── config.rs  # Configuration
│   │   └── tests/         # Integration tests
│   │
│   ├── x402-core/         # Core library
│   │   ├── src/
│   │   │   ├── policy/    # Policy engine
│   │   │   └── testing/   # Test framework
│   │   └── tests/         # Unit tests
│   │
│   ├── x402-server/       # Mock server
│   │   └── src/
│   │       ├── server.rs
│   │       └── handlers.rs
│   │
│   └── x402-domain/       # Domain types
│       └── src/
│           ├── types.rs
│           └── validation.rs
│
├── examples/              # Example projects
├── docs/                  # Documentation
└── tests/                 # E2E tests
```

---

## 💬 Communication

- **GitHub Issues** - Bug reports, feature requests
- **GitHub Discussions** - Questions, ideas, showcase
- **Pull Requests** - Code contributions
- **Email** - Security issues only

---

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## 🙏 Thank You!

Every contribution helps make x402-dev better for everyone. Whether you:
- Report a bug
- Fix a typo
- Add a feature
- Improve documentation
- Help other users

**Your time and effort are appreciated!** ❤️

---

**Questions?** Open a [discussion](https://github.com/valentynkit/x402-dev/discussions) or check our [documentation](docs/).

Happy coding! 🚀
