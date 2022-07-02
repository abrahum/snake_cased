#[cfg(feature = "derive")]
pub use snake_cased_derive::{Snaked, SnakedEnum};

pub trait Snaked {
    fn snaked() -> &'static str;
}

pub trait SnakedEnum {
    fn snaked_enum(&self) -> &'static str;
}

#[test]
fn example() {
    pub struct AStruct;
    impl Snaked for AStruct {
        fn snaked() -> &'static str {
            "a_struct"
        }
    }
    assert_eq!("a_struct", AStruct::snaked());

    #[allow(dead_code)]
    pub enum AEnum {
        AA,
        Bb,
        C,
    }
    impl SnakedEnum for AEnum {
        fn snaked_enum(&self) -> &'static str {
            match self {
                Self::AA => "a_a",
                Self::Bb => "bb",
                Self::C => "c",
            }
        }
    }
    assert_eq!("a_a", AEnum::AA.snaked_enum());
    assert_eq!("bb", AEnum::Bb.snaked_enum());
    assert_eq!("c", AEnum::C.snaked_enum());
}
