# Contributing to CPV

Thank you for your interest in contributing to CPV! This document provides guidelines and instructions for contributing.

## Development Process

1. Fork the repository
2. Create a new branch for your feature
3. Write tests for your changes
4. Implement your changes
5. Run tests and lints
6. Submit a pull request

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/). Each commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- feat: New feature
- fix: Bug fix
- docs: Documentation changes
- style: Code style changes (formatting, etc)
- refactor: Code changes that neither fix bugs nor add features
- test: Adding or modifying tests
- chore: Changes to build process or auxiliary tools

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Add tests for new features
- Update documentation as needed

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

## Pull Request Process

1. Update the README.md with details of changes if needed
2. Add tests for new features
3. Ensure all tests pass
4. Update documentation if needed
5. The PR will be merged once you have approval from maintainers

## Questions?

Feel free to open an issue for any questions about contributing!
