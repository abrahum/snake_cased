use snake_cased::{Snaked, SnakedEnum};

fn main() {
    assert_eq!("snake_cased", SnakeCased::snaked());
    assert_eq!("a_a", AEnum::AA.snaked_enum());
    assert_eq!("bb", AEnum::Bb.snaked_enum());
}

#[derive(Snaked)]
struct SnakeCased;

#[derive(SnakedEnum)]
enum AEnum {
    AA,
    Bb,
}
