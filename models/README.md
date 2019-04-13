[![crates-io-badge]][crates-io-link]
![Downloads](https://img.shields.io/crates/d/spectacles-model.svg?style=for-the-badge)
[![docs-badge]][docs-link]


# Spectacles Models
A collection of data structures for working with various Spectacles components.

## Usage
Each struct and enum in this crate supports JSON (de)serialization using Serde JSON.

#### Example: Deserializing a JSON payload
```rust
// In this example, we attempt to deserialize a Guild struct.
use spectacles_model::guild::Guild;

fn main() {
    // Here, we create a String for demonstration purposes. 
    // In reality, you cou be getting the payload from a variety of sources.
    let example_json = String::from("{}");
    // We use the from_str function to deserialize the string to a Guild object.
    // The function returns a result, with the struct is successful deserialization, or an error if deserialization failed.
    let guild: Guild = serde_json::from_str(&example_json).expect("Failed to deserialize JSON");
}
```

[crates-io-link]: https://crates.io/crates/spectacles-model
[crates-io-badge]: https://img.shields.io/crates/v/spectacles-model.svg?style=for-the-badge
[docs-link]: https://docs.rs/spectacles-model
[docs-badge]: https://img.shields.io/badge/Documentation-docs.rs-red.svg?style=for-the-badge