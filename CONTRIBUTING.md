# Contributing to CFX

We welcome contributions to CFX. Please follow this standard fork/branch workflow to contribute.

## Workflow

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally.
3. **Create a branch** for your feature or bug fix: `git checkout -b feature/your-feature-name`.
4. **Make your code changes**. Ensure they align with the project's security and implementation guidelines.
5. **Commit your changes**. Write clear, concise commit messages.
6. **Push to your fork**: `git push origin feature/your-feature-name`.
7. **Submit a Pull Request** (PR) against the `main` branch.

## Code Standards & Validation

CFX is a security-critical application. Before submitting a pull request, ensure you have run the following local validation commands:

- **Formatting**: Run `cargo fmt --all` to format the code to Rust standards.
- **Testing**: Run `cargo test --all-targets` to ensure all integration and unit tests pass.
- **Linting**: Run `cargo clippy --all-targets --all-features -- -D warnings` to catch any linting errors.

Our automated CI workflow will run these checks on every Pull Request. Pull requests will not be merged if any checks fail.
