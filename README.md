# serde-dig
Adds new method to serde_json::Value objects that allows recursive exploration

# Example usage:

```rust
 use serde_json::json;
 use serde_dig::{Dig};
 fn main() {
     // The type of `john` is `serde_json::Value`
     let john = json!({
         "name": "John Doe",
         "age": 43,
         "phones": [
             "+44 1234567",
             "+44 2345678"
         ]
     });
     println!("first phone number: {}", john.get_deep(&["phones".into(), 0.into()]).unwrap());
 }
```
