pub mod attr;
pub mod effect;
pub mod err;
pub mod id;
pub mod item;
pub mod loc;
pub mod text;
pub mod util;

pub use flux_macros as macros;
pub use flux_nbt as nbt;

pub use flux_nbt::Nbt;

extern crate self as fluxmc;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::f32::consts::PI;

    use flux_nbt::ser::{to_nbt, to_snbt};
    use lobsterchat::lobster;
    use serde::Serialize;

    use crate::item::{ItemDisplay, ItemStack, Material};

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

    #[test]
    fn test_snbt() {
        let test = vec!["abc", "def", "ghi"];
        let snbt = to_snbt(&test).unwrap();
        println!("{snbt}")
    }

    #[test]
    fn test_items() {
        let mut item = ItemStack::new(Material::DiamondSword);
        let mut lore = ItemDisplay::new();
        lore.set_name(lobster("<gold>Cool Sword").italic(false));
        lore.set_lore(vec![
            lobster("Line 1"),
            lobster("<yellow>Line 2").bold(true),
        ]);
        item.meta.display = Some(lore);
        println!("{}", item)
    }
}
