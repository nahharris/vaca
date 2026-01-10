# AGENTS.md - Vaca Programming Language

This file provides guidelines for AI agents working on the Vaca project, a LISP language interpreter written in Rust.

## Build, Lint, and Test Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo check              # Fast compile check without producing binaries
```

### Testing
```bash
cargo test               # Run all tests
cargo test --release     # Run tests in release mode
cargo test <test_name>   # Run a specific test by name
cargo test --lib         # Run only library tests
cargo test --bins        # Run only binary tests
cargo test --doc         # Run only doc tests
```

### Linting and Formatting
```bash
cargo clippy             # Run linter (warnings indicate style issues)
cargo clippy --fix       # Auto-fix clippy warnings
cargo fmt                # Format code according to rustfmt
cargo fmt --check        # Check formatting without modifying files
```

### Code Coverage
```bash
cargo tarpaulin --out Html  # Generate HTML coverage report
```

### Documentation
```bash
cargo doc --open         # Generate and open documentation
cargo doc --no-deps      # Generate docs without dependencies
```

## Code Style Guidelines

### Imports and Dependencies
- Use absolute paths with `crate::` for internal imports (e.g., `crate::parser::Parseable`)
- Use `use super::*` for relative parent module imports
- Group imports: std library first, then external crates, then internal modules
- Prefer importing traits directly when using their methods (e.g., `use std::io::Write;`)

### Formatting
- Use rustfmt default settings (4 spaces, max width 100)
- Use trailing commas in multi-line collections and function calls
- Place opening braces on the same line as control structures
- Use single-line bodies for simple functions; multi-line for complex logic

### Types and Generics
- Use `Result<T, String>` for fallible operations with simple error messages
- Use `Option<T>` for optional values
- Prefer explicit type annotations for public API types
- Use derive macros for common traits: `Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default`
- Follow Rust naming conventions:
  - Types: PascalCase (e.g., `Form`, `Symbol`, `List`)
  - Functions/variables: snake_case (e.g., `parse`, `accept`, `forms`)
  - Constants: SCREAMING_SNAKE_CASE
  - Type parameters: single uppercase letter or short words (e.g., `T`, `E`, `Error`)

### Error Handling
- Use the `?` operator for propagating errors in fallible functions
- Return `Err(message.to_string())` for simple parse/validation errors
- Use pattern matching with `match` or `let-else` for complex error cases
- Include context in error messages (e.g., "Expected a list", not just "error")

### Documentation
- Write doc comments (`///`) for public types and functions
- Include examples in doc comments using triple backticks with `ignore` tag
- Document enum variants with their purpose
- Use `#[warn(missing_docs)]` in lib.rs to ensure documentation coverage

### Testing
- Place unit tests in `#[cfg(test)] mod tests` blocks at the module level
- Use `#[test]` attribute for test functions
- Use `include_str!` to load sample files in tests
- Use `dbg!` for debugging test output; use `assert!`, `assert_eq!` for assertions
- Name test functions descriptively: `test_parse_symbol` not `test1`

### Project Structure
- Source files in `src/` with modules in `src/parser/`, `src/lexer.rs`, etc.
- Library code in `src/lib.rs`; binary entry in `src/main.rs`
- Sample programs in `src/samples/`
- Use `pub mod` and `pub use` for public API exposure

### Additional Conventions
- Use `edn_format::Value` for parsing; convert to domain types via `From` implementations
- Implement `Parseable` trait for types that can be parsed from EDN values
- Use trait objects sparingly; prefer generics with bounds when possible
- Keep functions focused: single responsibility, under 50 lines when feasible

## Common Patterns

### Parseable Trait Implementation
All domain types implement the `Parseable` trait with `parse()` and `accept()` methods.

### From Implementations
Use `From` trait for converting from EDN types to domain types.

### Pattern Matching
Prefer `let-else` for early-return pattern matching:
```rust
let edn_format::Value::List(list) = value else {
    return Err("Expected a list".to_string())
};
```

### Using matches! Macro
For simple type checks: `matches!(value, edn_format::Value::Symbol(_))`

## Working with EDN Values

### Value Types
`edn_format::Value` includes: Symbol, List, Vector, Map, Keyword, String, Integer, Float, Boolean, Nil.

### Destructuring EDN Values
Always use exhaustiveness checking when destructuring `Value`.

## Performance Considerations

- Use references (`&Value`) when the value doesn't need to be consumed
- Prefer `collect::<Result<Vec<_>, _>>()` over manual loops
- Use `ordered-float::OrderedFloat` for float comparisons
- Avoid unnecessary cloning

## Debugging Tips

- Use `dbg!` macro for quick debug output
- Use `println!("{:#?}", value)` for pretty-printing EDN values
- Run `cargo tree` to visualize the dependency tree

## Git Workflow

- Create feature branches: `git checkout -b feature/my-feature`
- Run `cargo test` and `cargo clippy` before committing
- Use conventional commit format: `feat: add new parser module`

## Common Tasks

### Adding a New Parser Module
1. Create `src/parser/my_type.rs` with type definition
2. Implement `Parseable` trait for the type
3. Add `pub mod my_type;` to `src/parser.rs`
4. Update `Form` enum to include the new type
5. Add tests in a `#[cfg(test)] mod tests` block

### Running a Single Test
```bash
cargo test test_parse_symbol    # Run by test function name
```

### Adding Dependencies
Edit `Cargo.toml` and run `cargo update`


