# spectacles-model
A collection of data types for working with various Spectacles modules.

## Usage
All of the structs and enums support (de)serialization using serde-json. An example would be the following:

```rust
use spectacles_model::guild::Guild;
let guild: Guild = <JSONVariableHere>;
```