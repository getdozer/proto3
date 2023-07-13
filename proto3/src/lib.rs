pub trait Proto3 {
    const PROTO3_TYPE: Type;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    String,
    Int32,
    Message(Message),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    pub name: &'static str,
    pub fields: &'static [Field],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub kind: FieldKind,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldKind {
    Normal {
        rule: FieldRule,
        typ: Type,
        number: u32,
    },
    Oneof(&'static [Field]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldRule {
    None,
    Optional,
    Repeated,
}

pub const EMPTY: Message = Message {
    name: "google.protobuf.Empty",
    fields: &[],
};

impl Proto3 for String {
    const PROTO3_TYPE: Type = Type::String;
}

impl Proto3 for i32 {
    const PROTO3_TYPE: Type = Type::Int32;
}
