# Rust Iterator Lab: Slices & Closures Edition

A dedicated testing ground for exploring Rust's functional programming features, iterators, and memory-safe collection manipulation. This project serves as a practical sandbox for understanding how Rust handles borrowed data and closures under the hood without sacrificing performance.

## Status: Work in Progress
Currently following the **Stephen Grider Rust Course**. This project serves as a "Proof of Work" for understanding the nuances of immutable vs. mutable iteration, closure execution, and slice referencing.

## Engineering Highlights
- **Functional Iteration**: Replacing traditional `for` loops with iterator chains (`.iter()`, `.map()`, `.for_each()`) to transform and consume collections idiomatically.
- **Slice Referencing**: Passing partial, borrowed views of arrays (e.g., `&colors[1..3]`) to functions to minimize memory overhead and avoid unnecessary heap allocations.
- **Mutable Borrowing**: Executing safe, in-place data modifications using `.iter_mut()` alongside standard library string methods (like `.truncate()`), strictly adhering to Rust's mutability and borrowing rules.
- **Collection Transformations**: Leveraging the turbo-fish syntax (`::<>`) and `.collect::<Vec<String>>()` to gather iterator results into newly allocated data structures.

## Current Features
- [x] **Partial View Printing**: Safely reading and displaying specific segments of a vector using immutable slice references.
- [x] **Data Transformation**: Generating entirely new vectors of uppercase strings without mutating the source data.
- [x] **In-Place Mutation**: Modifying existing string data directly within memory by iterating over mutable references.

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