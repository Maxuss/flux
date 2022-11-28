pub mod attr;
pub mod err;
pub mod id;
pub mod item;

pub use flux_macros as macros;
pub use flux_nbt as nbt;

pub use flux_macros::Nbt;
pub use flux_nbt::Nbt;

extern crate self as fluxmc;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use flux_macros::Nbt;
    use flux_nbt::Nbt;
    use nbt::Value;

    #[derive(Nbt)]
    struct ProcMacroTest {
        #[rename("Heloo")]
        hello: String,
        pos: Position,
        #[rename("enum")]
        an_enum: EnumTest,
    }

    #[derive(Nbt)]
    struct Position(i32, i32, i32);

    #[derive(Nbt)]
    #[lower]
    enum EnumTest {
        First,
        Second,
        Third {
            #[rename("Complex")]
            a: i32,
        },
        Fourth(u64, String),
    }

    #[test]
    fn test_proc_macro() {
        let test = ProcMacroTest {
            hello: "World".to_owned(),
            pos: Position(1, 2, 3),
            an_enum: EnumTest::Second,
        };
        let nbt: Value = test.nbt();
        println!("{nbt:#?}")
    }
}
