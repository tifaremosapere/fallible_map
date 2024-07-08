# Fallible Map

[![Crates.io](https://img.shields.io/crates/v/fallible_map.svg)](https://crates.io/crates/fallible_map)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green)
[![Repository](https://img.shields.io/badge/github-repository-orange)](https://github.com/tifaremosapere/fallible_map)
[![Homepage](https://img.shields.io/badge/homepage-Ti%20faremo%20sapere-brightgreen)](https://tifaremosapere.it)

`fallible_map` provides utilities for fallible mapping over `Option` types and iterators, allowing the use of functions that can return `Result`s.

This library includes traits to enhance `Option` and `Iterator` types with methods to handle fallible operations gracefully.

## Overview

This crate offers extensions for optional values and iterators to perform fallible mapping operations, returning results that properly reflect potential errors during computation.

These extensions can be useful in scenarios where operations may fail, and error handling is required.

## Features

- **ExtractOption trait:** A helper trait to extract the inner value of an optional container;
- **FallibleMapExt trait:** Extends `Option` with methods for fallible operations, such as `try_map`, `try_unwrap_or`, and `try_and_then`;
- **TryMapIteratorExt trait:** Extends iterators with a `try_map` method, allowing the use of functions that return `Result`s during iteration.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fallible_map = "^0.1"
```

## Usage

### Examples

#### Using FallibleMapExt with `Option`

```rust
use fallible_map::FallibleMapExt;

fn main() -> Result<(), String> {
    let some_number: Option<i32> = Some(2);

    let result: Result<Option<i32>, String> = some_number.try_map(|num| {
        if num % 2 == 0 {
            Ok(num * 2)
        } else {
            Err("Odd number".to_string())
        }
    });

    assert_eq!(result, Ok(Some(4)));

    Ok(())
}
```

#### Using TryMapIteratorExt with `Iterator`

```rust
use fallible_map::TryMapIteratorExt;

fn main() -> Result<(), String> {
    let numbers = vec![1, 2, 3, 4, 5];

    let mapped_numbers: Result<Vec<i32>, String> = numbers.into_iter().try_map(|x| {
        if x % 2 == 0 {
            Ok(x * 2)
        } else {
            Err(format!("Failed to process {}", x))
        }
    });

    match mapped_numbers {
        Ok(nums) => println!("Mapped successfully: {:?}", nums),
        Err(e) => println!("Error occurred: {}", e),
    }

    Ok(())
}
```

#### Using FallibleMapExt with `try_and_then`

```rust
use fallible_map::FallibleMapExt;

fn main() -> Result<(), String> {
    let some_number: Option<i32> = Some(2);

    let result: Result<Option<i32>, String> = some_number.try_and_then(|num| {
        if num % 2 == 0 {
            Ok(Some(num * 2))
        } else {
            Err("Odd number".to_string())
        }
    });

    assert_eq!(result, Ok(Some(4)));

    let none_number: Option<i32> = None;

    let result = none_number.try_and_then(|num| {
        if num % 2 == 0 {
            Ok(Some(num * 2))
        } else {
            Err("Odd number".to_string())
        }
    });

    assert_eq!(result, Ok(None));

    Ok(())
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.

## Contribution

Contributions are welcome! Please feel free to submit a pull request, open an issue, or suggest features and improvements.