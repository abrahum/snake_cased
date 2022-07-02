## snake_cased

features:

- derive

```rust
use snake_cased::{Snaked, SnakedEnum};

#[derive(Snaked)]
struct SnakeCased;

#[derive(SnakedEnum)]
enum AEnum {
    AA,
    Bb,
}

assert_eq!("snake_cased", SnakeCased::snaked());
assert_eq!("a_a", AEnum::AA.snaked_enum());
assert_eq!("bb", AEnum::Bb.snaked_enum());
```