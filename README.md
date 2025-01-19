Rust macro that splatters attributes across multiple items.

### Syntax

```rust
splat_attribs! {
  ~attributes~:
  ~items~
}
```

### Example
```rust
use splat_attribs::splat_attribs;

fn main() {
  println!("{Casiopea} < {TSquare} < {Dimension}")
}

splat_attribs! {
  #[allow(non_upper_case_globals)]
  #[doc = "Applied to all items"]:

  const Casiopea: u32 = 10 / 10;
  const TSquare: u32 = 11 / 10;
  const Dimension: u32 = u32::MAX / 10;
}
```
