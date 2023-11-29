# Macaroni 🍝🧀

Macaroni is the missing standard library for Rust macros!
It provides a collection of useful macros that can be used
in any Rust project. Macaroni is designed to be easy to use, easy to integrate, and easy to extend.

## Getting Started

To start using Macaroni in your Rust project, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
macaroni = "0.1.0"
```
## Usage

The following macros are currently available in Macaroni:

### `collect!`

The `collect!` macro can be used to create a tuple, array, map, or vector from a list of values. For example:

```rust
use std::collections::HashMap;
use macaroni::collect;

fn main() {
    let tuple: (u32, u32, u32) = collect![1, 2, 3];
    println!("Tuple: {:?}", tuple);
    let map: HashMap<&str, u32> = collect!["a" => 1, "b" => 2, "c" => 3];
}
``` 

## Documentation

For detailed documentation and a full list of available macros, please refer to our Documentation Page.

## License

Macaroni is distributed under the terms of the GPL-3 license. See [COPYING](COPYING) for details.
