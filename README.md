# range-enum
An attribute macro that generates enum variants from a range.
# Example
```rust
use range_enum::range_enum;

#[range_enum(1..=2)]
enum Range {}

let _: Range = Range::Range1;
```
# Note
Any existing variants or generics are erased by this attribute

