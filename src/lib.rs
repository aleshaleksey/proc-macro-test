#![allow(clippy::derive_partial_eq_without_eq)]
use derive::IntoStruct;

pub trait IntoStruct<S> {
    fn as_struct(&self) -> S;
}

#[derive(IntoStruct, Debug, PartialEq, Clone)]
pub enum MyEnum {
    Yes(String, usize),
    No,
}

#[derive(IntoStruct, Debug, PartialEq, Clone)]
pub enum StupidEnum {
    Monster(String, usize, bool),
    AllTheNumbers(
        u8,
        u16,
        u32,
        u64,
        u128,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        usize,
    ),
    Yes(String),
    No,
}

#[cfg(test)]
mod test_into_struct {
    use super::*;

    #[test]
    fn test1() {
        let a = MyEnum::Yes(String::from("Pineapple Curry!"), 42);
        let x = MyEnumStruct {
            yes_set: true,
            yes_0: String::from("Pineapple Curry!"),
            yes_1: 42,
            no: false,
        };
        assert_eq!(a.as_struct(), x);
    }

    #[test]
    fn test2() {
        let a = MyEnum::No;
        let expected = MyEnumStruct {
            yes_set: false,
            yes_0: String::default(),
            yes_1: 0,
            no: true,
        };
        assert_eq!(a.as_struct(), expected);
    }

    #[test]
    fn test3() {
        let a = StupidEnum::No;

        let x = StupidEnumStruct {
            monster_set: false,
            monster_0: String::default(),
            monster_1: usize::default(),
            monster_2: false,
            allthenumbers_set: false,
            allthenumbers_0: 0u8,
            allthenumbers_1: 0u16,
            allthenumbers_2: 0u32,
            allthenumbers_3: 0u64,
            allthenumbers_4: 0u128,
            allthenumbers_5: 0i8,
            allthenumbers_6: 0i16,
            allthenumbers_7: 0i32,
            allthenumbers_8: 0i64,
            allthenumbers_9: 0i128,
            allthenumbers_10: 0isize,
            allthenumbers_11: 0usize,
            yes_set: false,
            yes_0: String::default(),
            no: true,
        };
        assert_eq!(a.as_struct(), x);
    }

    #[test]
    fn test4() {
        let a = StupidEnum::AllTheNumbers(0, 1, 2, 3, 4, -1, -2, -3, -4, -5, -999, 999);

        let x = StupidEnumStruct {
            monster_set: false,
            monster_0: String::default(),
            monster_1: usize::default(),
            monster_2: false,
            allthenumbers_set: true,
            allthenumbers_0: 0u8,
            allthenumbers_1: 1u16,
            allthenumbers_2: 2u32,
            allthenumbers_3: 3u64,
            allthenumbers_4: 4u128,
            allthenumbers_5: -1i8,
            allthenumbers_6: -2i16,
            allthenumbers_7: -3i32,
            allthenumbers_8: -4i64,
            allthenumbers_9: -5i128,
            allthenumbers_10: -999isize,
            allthenumbers_11: 999usize,
            yes_set: false,
            yes_0: String::default(),
            no: false,
        };
        assert_eq!(a.as_struct(), x);
    }
}
