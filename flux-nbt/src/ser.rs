use std::sync::{Arc, Mutex};

use nbt::Value;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use thiserror::Error;

use crate::snbt::Snbt;

pub fn to_nbt<T: Serialize>(obj: &T) -> Option<Value> {
    let mut ser = Obj2CmpSerializer::new();
    ser.scopes.lock().unwrap().push(ScopeTy::None);
    ser.push_anonymous_node();
    ser.serialize(obj)
}

pub fn to_snbt<T: Serialize>(obj: &T) -> Option<String> {
    let mut out = String::new();
    to_nbt(obj)?.snbt_fmt(&mut out);
    Some(out)
}

#[derive(Debug, Clone)]
enum CompoundNode {
    Vacant(String),
    Occupied(String, Value),
    Unnamed(Value),
}

#[derive(Debug, Clone, Copy)]
enum ScopeTy {
    None,
    Compound(usize),
    Array(usize),
}

#[derive(Debug, Error)]
pub enum SerError {
    #[error("Can only serialize keys of String type")]
    InvalidKey,
    #[error("A serde error has occurred: {0}")]
    Custom(String),
}

impl serde::ser::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Obj2CmpSerializer {
    nodes: Arc<Mutex<Vec<CompoundNode>>>,
    scopes: Arc<Mutex<Vec<ScopeTy>>>,
    output: Arc<Mutex<std::collections::HashMap<String, Value>>>,
}

pub struct Seq<'a> {
    ser: &'a mut Obj2CmpSerializer,
}

pub struct Struct<'a> {
    ser: &'a mut Obj2CmpSerializer,
}

impl Obj2CmpSerializer {
    pub fn new() -> Self {
        let scopes = Arc::new(Mutex::new(Vec::with_capacity(8)));
        let nodes = Arc::new(Mutex::new(Vec::with_capacity(64)));
        let output = Arc::new(Mutex::new(std::collections::HashMap::new()));

        Self {
            nodes,
            scopes,
            output,
        }
    }

    pub fn serialize<T: Serialize>(&mut self, obj: &T) -> Option<Value> {
        let mut copy = Obj2CmpSerializer {
            nodes: self.nodes.clone(),
            scopes: self.scopes.clone(),
            output: self.output.clone(),
        };
        copy.push_cmp();
        obj.serialize(&mut copy).unwrap();
        match self.pop_node() {
            CompoundNode::Unnamed(value) => Some(value),
            _ => None,
        }
    }

    pub fn finish(&mut self) -> Option<Value> {
        match self.pop_node() {
            CompoundNode::Unnamed(value) => Some(value),
            _ => None,
        }
    }

    fn push_node(&mut self, name: String) {
        self.nodes.lock().unwrap().push(CompoundNode::Vacant(name))
    }

    fn push_anonymous_node(&mut self) {
        self.nodes
            .lock()
            .unwrap()
            .push(CompoundNode::Unnamed(Value::Byte(0)))
    }

    fn mutate_node(&mut self, value: Value) {
        let mut nodes = self.nodes.lock().unwrap();
        let last = nodes.last_mut();
        match last {
            Some(last) => match last {
                CompoundNode::Vacant(name) => {
                    *last = CompoundNode::Occupied(name.to_owned(), value)
                }
                CompoundNode::Occupied(_, old_value) => *old_value = value,
                CompoundNode::Unnamed(old_value) => *old_value = value,
            },
            None => {
                // we are probably in array
                self.nodes
                    .lock()
                    .unwrap()
                    .push(CompoundNode::Unnamed(value))
            }
        }
    }

    fn pop_node(&mut self) -> CompoundNode {
        self.nodes.lock().unwrap().pop().unwrap()
    }

    fn push_array(&mut self) {
        self.scopes
            .lock()
            .unwrap()
            .push(ScopeTy::Array(self.nodes.lock().unwrap().len()));
    }

    fn push_cmp(&mut self) {
        self.scopes
            .lock()
            .unwrap()
            .push(ScopeTy::Compound(self.nodes.lock().unwrap().len()));
    }

    fn pop_scope(&mut self) {
        let last = self.scopes.lock().unwrap().pop().unwrap();
        match last {
            ScopeTy::None => unreachable!(),
            ScopeTy::Compound(cmp) => {
                let map = std::collections::HashMap::<String, Value>::from_iter(
                    self.nodes
                        .lock()
                        .unwrap()
                        .drain(cmp..)
                        .map(|node| match node {
                            CompoundNode::Occupied(name, value) => (name, value),
                            _ => unreachable!(),
                        }),
                );
                self.mutate_node(Value::Compound(map))
            }
            ScopeTy::Array(arr) => {
                let arr: Vec<Value> = self
                    .nodes
                    .lock()
                    .unwrap()
                    .drain(arr..)
                    .map(|node| match node {
                        CompoundNode::Unnamed(value) => value,
                        other => panic!("Did not expect the {other:?} node"),
                    })
                    .collect();
                macro_rules! unwrap {
                    ($value:ident as $pattern:ident) => {
                        match $value {
                            Value::$pattern(value) => value,
                            _ => panic!("Heterogenous array encountered!"),
                        }
                    };
                }
                let val = match arr.first().unwrap() {
                    Value::Byte(_) => {
                        // a byte array
                        Value::ByteArray(
                            arr.into_iter().map(|each| unwrap!(each as Byte)).collect(),
                        )
                    }
                    Value::Int(_) => {
                        Value::IntArray(arr.into_iter().map(|each| unwrap!(each as Int)).collect())
                    }
                    Value::Long(_) => Value::LongArray(
                        arr.into_iter().map(|each| unwrap!(each as Long)).collect(),
                    ),
                    _ => Value::List(arr),
                };
                self.mutate_node(val)
            }
        }
    }
}

impl Default for Obj2CmpSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused_variables)]
impl<'a> Serializer for &'a mut Obj2CmpSerializer {
    type Ok = ();

    type Error = SerError;

    type SerializeSeq = Seq<'a>;

    type SerializeTuple = Seq<'a>;

    type SerializeTupleStruct = Seq<'a>;

    type SerializeTupleVariant = Seq<'a>;

    type SerializeMap = Struct<'a>;

    type SerializeStruct = Struct<'a>;

    type SerializeStructVariant = Struct<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Byte(v as i8));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Byte(v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Short(v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Int(v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Long(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i8(v as i8)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i16(v as i16)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Float(v));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::Double(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::String(v.to_owned()));
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.mutate_node(Value::ByteArray(v.iter().map(|num| *num as i8).collect()));
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.pop_node();
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.push_array();
        Ok(Seq { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.push_array();
        Ok(Seq { ser: self })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.push_array();
        Ok(Seq { ser: self })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.push_array();
        Ok(Seq { ser: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.push_cmp();
        Ok(Struct { ser: self })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.push_cmp();
        Ok(Struct { ser: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.push_cmp();
        Ok(Struct { ser: self })
    }
}

impl<'a> SerializeSeq for Seq<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_anonymous_node();
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeTuple for Seq<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_anonymous_node();
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for Seq<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_anonymous_node();
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for Seq<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_anonymous_node();
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeMap for Struct<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_anonymous_node();
        key.to_owned().serialize(&mut self.ser.clone())?;
        let name = match self.ser.pop_node() {
            CompoundNode::Unnamed(Value::String(str)) => str,
            _ => return Err(SerError::InvalidKey),
        };
        self.ser.push_node(name);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeStruct for Struct<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_node(key.to_owned());
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}

impl<'a> SerializeStructVariant for Struct<'a> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.push_node(key.to_owned());
        value.serialize(&mut self.ser.clone())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.pop_scope();
        Ok(())
    }
}
