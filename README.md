# Rust Iterator Lab: Slices & Closures Edition

A dedicated testing ground for exploring Rust's functional programming features, iterators, and memory-safe collection manipulation. This project serves as a practical sandbox for understanding how Rust handles borrowed data, closures, and ownership transfers under the hood without sacrificing performance.

## Status: Done
Currently following the **Stephen Grider Rust Course**. This project serves as a "Proof of Work" for understanding the nuances of immutable vs. mutable iteration, closure execution, slice referencing, and safe optional handling.

## Engineering Highlights
- **Functional Iteration**: Replacing traditional `for` loops with iterator chains (`.iter()`, `.map()`, `.for_each()`, `.filter()`) to transform and consume collections idiomatically.
- **Slice Referencing**: Passing partial, borrowed views of arrays (e.g., `&colors[1..3]`) to functions to minimize memory overhead and avoid unnecessary heap allocations.
- **Mutable Borrowing**: Executing safe, in-place data modifications using `.iter_mut()` alongside standard library string methods (like `.truncate()`), strictly adhering to Rust's mutability and borrowing rules.
- **Ownership & Consumption**: Leveraging `.into_iter()` to completely consume collections and transfer ownership of elements in memory.
- **Safe Option Handling**: Utilizing `.find()` paired with `.map_or()` to safely unwrap `Option` types, providing default allocations to avoid null reference panics.
- **Collection Transformations**: Leveraging the turbofish syntax (`::<>`) and `.collect()` to gather iterator results into newly allocated data structures, including complex nested vectors (`Vec<Vec<String>>`).

## Current Features
- [x] **Partial View Printing**: Safely reading and displaying specific segments of a vector using immutable slice references.
- [x] **Data Transformation**: Generating entirely new vectors of uppercase strings without mutating the source data.
- [x] **In-Place Mutation**: Modifying existing string data directly within memory by iterating over mutable references.
- [x] **Ownership Transfer**: Moving elements from one vector to another by consuming the source iterator.
- [x] **Nested Iteration**: Exploding strings into multi-dimensional vectors using chained `.map()` calls.
- [x] **Conditional Filtering**: Iterating through vectors of structs and filtering based on specific struct fields using auto-dereferencing.
- [x] **Safe Fallbacks**: Searching for substrings and returning safe, allocated fallbacks when matches are not found.

## Development Environment
Built and tested in a professional-grade systems environment:
- **Host OS**: Windows 10
- **Kernel**: WSL2 (Windows Subsystem for Linux)
- **Environment**: Ubuntu 24.04.3 LTS (Noble Numbat)
- **Toolchain**: Rustc 1.94.0 / Cargo 1.94.0
- **IDE**: VS Code (Remote-WSL Extension)

## Setup & Execution
Ensure you have the Rust toolchain installed. You can verify your environment and launch the project by running the following commands in your Ubuntu CLI:

```bash
# 1. Verify your local environment versions
rustc --version
cargo --version
lsb_release -a

# 2. Clone the project repository
git clone [https://github.com/iHuGi/rust-iterator.git](https://github.com/iHuGi/rust-iterator.git)
cd rust-iterator

# 3. Run the application
cargo run -q