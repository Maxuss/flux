use nbt::Value;

#[macro_export]
macro_rules! nbt {
    (
        $($k:tt: $v:tt),* $(,)*
    ) => {{
        #[allow(unused_imports)]
        use $crate::Nbt;
        $crate::Value::Compound(std::collections::HashMap::<String, $crate::Value>::from([
            $(
            ($crate::__nbt_key!($k), $crate::__nbt_val!($v)),
            )*
        ]))
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nbt_key {
    ($str:literal) => {
        $str.to_string()
    };
    ([$var:ident]) => {
        $var.to_owned()
    };
    ($name:ident) => {
        stringify!($name).to_owned()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nbt_val {
    ({
        $(
            $k:ident: $v:tt
        ),* $(,)*
    }) => {
        nbt!($($k:$v,)*)
    };
    ($var:ident) => {
        $var.nbt()
    };
    ($lit:literal) => {
        $lit.nbt()
    };
    ([$($ele:tt),* $(,)*]) => {
        $crate::Value::List(vec![$($crate::__nbt_val!($ele),)*])
    };
    ([I;$($ele:tt),* $(,)*]) => {
        $crate::Value::IntArray(vec![$($ele,)*])
    };
    ([L;$($ele:tt),* $(,)*]) => {
        $crate::Value::LongArray(vec![$($ele,)*])
    };
    ([B;$($ele:tt),* $(,)*]) => {
        $crate::Value::ByteArray(vec![$($ele,)*])
    };
    ($($tks:tt)*) => {
        $($tks)*
    }
}

macro_rules! existing_impls {
    ($($i:ty),*) => {
        $(
            impl Nbt for $i {
                fn nbt(&self) -> Value {
                    Value::from(self.to_owned())
                }
            }
        )*
    };
}

pub trait Nbt {
    fn nbt(&self) -> Value;
}

impl Nbt for bool {
    fn nbt(&self) -> Value {
        Value::Byte(*self as i8)
    }
}

impl Nbt for u8 {
    fn nbt(&self) -> Value {
        Value::Byte(*self as i8)
    }
}

impl Nbt for u16 {
    fn nbt(&self) -> Value {
        Value::Short(*self as i16)
    }
}

impl Nbt for u32 {
    fn nbt(&self) -> Value {
        Value::Int(*self as i32)
    }
}

impl Nbt for u64 {
    fn nbt(&self) -> Value {
        Value::Long(*self as i64)
    }
}

existing_impls!(
    i8,
    i16,
    i32,
    i64,
    &str,
    String,
    &[i8],
    &[i32],
    &[i64],
    Vec<i8>,
    Vec<i32>,
    Vec<i64>,
    f32,
    f64
);
