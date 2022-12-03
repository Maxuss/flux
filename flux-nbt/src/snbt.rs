use nbt::Value;

pub fn to_string(nbt: &Value) -> String {
    let mut buf = String::new();
    nbt.snbt_fmt(&mut buf);
    buf
}

pub trait Snbt {
    fn snbt_fmt(&self, into: &mut String);
}

impl Snbt for Value {
    fn snbt_fmt(&self, into: &mut String) {
        match self {
            Value::Byte(v) => into.push_str(&format!("{v}b")),
            Value::Short(v) => into.push_str(&format!("{v}s")),
            Value::Int(v) => into.push_str(&v.to_string()),
            Value::Long(v) => into.push_str(&format!("{v}L")),
            Value::Float(v) => into.push_str(&format!("{v}f")),
            Value::Double(v) => into.push_str(&format!("{v}d")),
            Value::ByteArray(bytes) => {
                into.push_str("[B;");
                let len = bytes.len();
                for (index, part) in bytes.iter().enumerate() {
                    into.push_str(&part.to_string());
                    if index + 1 < len {
                        into.push(',')
                    }
                }
                into.push(']')
            }
            Value::String(st) => into.push_str(&format!("'{st}'")),
            Value::List(ls) => {
                into.push('[');
                let len = ls.len();
                for (index, part) in ls.iter().enumerate() {
                    part.snbt_fmt(into);
                    if index + 1 < len {
                        into.push(',')
                    }
                }
                into.push(']')
            }
            Value::Compound(cmp) => {
                into.push('{');
                let len = cmp.len();
                for (index, (key, value)) in cmp.iter().enumerate() {
                    into.push_str(&format!("{key}:"));
                    value.snbt_fmt(into);
                    if index + 1 < len {
                        into.push(',')
                    }
                }
                into.push('}')
            }
            Value::IntArray(ints) => {
                into.push_str("[I;");
                let len = ints.len();
                for (index, part) in ints.iter().enumerate() {
                    into.push_str(&part.to_string());
                    if index + 1 < len {
                        into.push(',')
                    }
                }
                into.push(']')
            }
            Value::LongArray(longs) => {
                into.push_str("[B;");
                let len = longs.len();
                for (index, part) in longs.iter().enumerate() {
                    into.push_str(&part.to_string());
                    if index + 1 < len {
                        into.push(',')
                    }
                }
                into.push(']')
            }
        }
    }
}
