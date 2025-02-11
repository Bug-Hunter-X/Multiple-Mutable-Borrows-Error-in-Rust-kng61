# Multiple Mutable Borrows in Rust
This example demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable. Rust's ownership system prevents this to avoid data races and ensure memory safety.

The `bug.rs` file contains the erroneous code, and `bugSolution.rs` provides a corrected version.