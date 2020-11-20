# serde-dig
Adds new method to `serde_json::Value` objects that allows recursive exploration

I wanted a function similar to Ruby's [Hash#dig](https://ruby-doc.org/core-2.3.0_preview1/Hash.html#method-i-dig) but 
came up empty with searching.

When you want something done, just do it. 

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
     let path:&[DigIndex] = &["phones".into(), 0.into()];
     println!("first phone number: {}", john.get_deep(path).unwrap());
 }
```

This only really works for `serde_json::Value` objects. If this was to be more general, a similar algebraic union of all major types would have to be used. This is because the return type is not 100% known (nested vectors, maps, oh my!). 
