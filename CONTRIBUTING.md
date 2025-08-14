# Contributing to `dnp3time`

We welcome contributions to the `dnp3time` project! Whether it's bug fixes, features, documentation improvements, or other enhancements, your help is appreciated.

## Getting Started

1. **Fork the Repository**  
   To start, fork the repository and clone it locally:
   ```bash
   git clone https://github.com/<your-username>/DNP3TimmingAttack.git
   cd DNP3TimmingAttack
   ```

2. **Set Up Your Development Environment**  
   You need to have the Rust toolchain installed. If it's not already installed, you can do so with:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

3. **Run Tests**  
   Confirm the project is working correctly by building and running the provided tests:
   ```bash
   cargo test
   ```

4. **Make Changes**  
   Edit the code, tests, or documentation to improve the project. Be sure to follow best practices and make your changes as clean and modular as possible.

5. **Submit a Pull Request**  
   After completing and testing your changes:
   - Push your branch to your fork.
   - Create a pull request in the [main repository](https://github.com/ladderlogix/DNP3TimmingAttack).

## Code Style
`dnp3time` follows the Rust 2024 edition and the Rust style guide. Use `cargo fmt` to ensure your code is properly formatted:
```bash
cargo fmt
```

To ensure your code is free of common errors, also run:
```bash
cargo clippy
```

## Reporting Issues
If you find a bug, file an issue on the [GitHub Issues page](https://github.com/ladderlogix/DNP3TimmingAttack/issues). Please include:
- Steps to reproduce the issue.
- Expected vs actual behavior.
- Relevant output logs (if applicable).
- Your environment (OS, Rust version, etc.).

## Testing Contributions
If you add a feature or fix a bug, include tests to verify your changes. Write unit tests wherever applicable and ensure the entire test suite passes:
```bash
cargo test
```

## Feature Requests
We welcome ideas for improvements! If you have a feature in mind, file a feature request in the [GitHub Issues page](https://github.com/ladderlogix/DNP3TimmingAttack/issues).

---

Thank you for your interest in contributing!