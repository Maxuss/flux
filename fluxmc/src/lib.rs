pub mod attr;
pub mod err;
pub mod id;
pub mod item;

pub use flux_macros as macros;
pub use flux_nbt as nbt;

pub use flux_nbt::Nbt;

extern crate self as fluxmc;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::f32::consts::PI;

    use serde::Serialize;

    #[derive(Serialize)]
    struct SerdeTest {
        #[serde(rename = "Helloo")]
        hello: String,
        pos: Position,
        #[serde(rename = "enum")]
        an_enum: EnumTest,
        #[serde(rename = "array")]
        an_array: Vec<&'static str>,
    }

    #[derive(Serialize)]
    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    enum EnumTest {
        First,
        Second,
        Third {
            #[serde(rename = "Complex")]
            a: i32,
        },
        Fourth(u64, String),
    }

    #[test]
    fn test_proc_macro() {
        let test = SerdeTest {
            hello: "World".to_owned(),
            pos: Position {
                x: PI * 2.,
                y: PI,
                z: PI / 2.,
            },
            an_enum: EnumTest::Third { a: 123456 },
            an_array: vec!["abc", "def", "ghi"],
        };
        let nbt = flux_nbt::ser::to_snbt(&test).unwrap();
        println!("{nbt}")
    }
}
