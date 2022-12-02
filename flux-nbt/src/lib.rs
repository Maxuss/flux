pub mod macros;
pub mod ser;
pub mod snbt;

pub use macros::Nbt;
pub use nbt as bin;
pub use nbt::Value;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_macros() {
        let tag = nbt! {
            "a": true,
            "b": false,
            cmp: {
                another: "Hello!",
                list: [1, 2, 3],
                bytes: [B; 1, 2, 3],
                ints: [I; 1, 2, 3],
                empty: [I;]
            }
        };
        assert_eq!(tag.tag_name(), "TAG_Compound")
    }

    #[test]
    pub fn test_snbt() {
        let value = nbt! {
            first: "Hello, World!",
            second: [I; 1, 2, 3],
            third: {
                a: 1,
                b: [B; 1, 2, 3],
            }
        };
        println!("{}", snbt::to_string(&value))
    }
}
