# Rust Iterator Exhaustion Bug

This repository demonstrates a common error in Rust where an iterator is exhausted, resulting in a runtime panic. The code attempts to access elements from a vector iterator after all elements have been consumed.

## How to reproduce

1. Clone the repository
2. Run `cargo run`

The code will compile but will result in a runtime panic as it attempts to access elements after exhausting the iterator.