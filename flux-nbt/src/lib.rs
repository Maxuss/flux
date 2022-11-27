pub mod macros;
#[doc(hidden)]
pub mod snbt;

pub use nbt as bin;
pub use nbt::Value;
pub use snbt as str;

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
}
