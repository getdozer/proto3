use proto3::Proto3;

pub struct StructExample {
    pub string: String,
    pub int: i32,
    pub optional_string: Option<String>,
    pub vec_string: Vec<String>,
}

pub enum EnumExample {
    UnitVariant,
    TupleVariant(String, i32),
    StructVariant { string: String, int: i32 },
}

pub struct NestedStructExample {
    pub nested_struct: StructExample,
    pub optional_nested_struct: Option<StructExample>,
    pub vec_nested_struct: Vec<StructExample>,
    pub nested_enum: EnumExample,
    pub optional_nested_enum: Option<EnumExample>,
    pub vec_nested_enum: Vec<EnumExample>,
}

const STRUCT_EXAMPLE_PROST_FIELDS: [proto3::Field; 4] = [
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::String,
            number: 1,
        },
        name: "string",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Int32,
            number: 2,
        },
        name: "int",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Optional,
            typ: proto3::Type::String,
            number: 3,
        },
        name: "optional_string",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Repeated,
            typ: proto3::Type::String,
            number: 4,
        },
        name: "vec_string",
    },
];

impl Proto3 for StructExample {
    const PROTO3_TYPE: proto3::Type = proto3::Type::Message(proto3::Message {
        name: "StructExampleProst",
        fields: &STRUCT_EXAMPLE_PROST_FIELDS,
    });
}

const ENUM_EXAMPLE_PROST_FIELDS: [proto3::Field; 1] = [proto3::Field {
    kind: proto3::FieldKind::Oneof(&ENUM_EXAMPLE_PROST_INNER_FIELDS),
    name: "inner",
}];

const ENUM_EXAMPLE_PROST_INNER_FIELDS: [proto3::Field; 3] = [
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Message(proto3::EMPTY),
            number: 1,
        },
        name: "unit_variant",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Message(proto3::Message {
                name: "EnumExampleProstTupleVariant",
                fields: &ENUM_EXAMPLE_PROST_TUPLE_VARIANT_FIELDS,
            }),
            number: 2,
        },
        name: "tuple_variant",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Message(proto3::Message {
                name: "EnumExampleProstStructVariant",
                fields: &ENUM_EXAMPLE_PROST_STRUCT_VARIANT_FIELDS,
            }),
            number: 3,
        },
        name: "struct_variant",
    },
];

const ENUM_EXAMPLE_PROST_TUPLE_VARIANT_FIELDS: [proto3::Field; 2] = [
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::String,
            number: 1,
        },
        name: "field_0",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Int32,
            number: 2,
        },
        name: "field_1",
    },
];

const ENUM_EXAMPLE_PROST_STRUCT_VARIANT_FIELDS: [proto3::Field; 2] = [
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::String,
            number: 1,
        },
        name: "string",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: proto3::Type::Int32,
            number: 2,
        },
        name: "int",
    },
];

impl Proto3 for EnumExample {
    const PROTO3_TYPE: proto3::Type = proto3::Type::Message(proto3::Message {
        name: "EnumExampleProst",
        fields: &ENUM_EXAMPLE_PROST_FIELDS,
    });
}

const NESTED_STRUCT_EXAMPLE_PROST_FIELDS: [proto3::Field; 6] = [
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: StructExample::PROTO3_TYPE,
            number: 1,
        },
        name: "nested_struct",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Optional,
            typ: StructExample::PROTO3_TYPE,
            number: 2,
        },
        name: "optional_nested_struct",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Repeated,
            typ: StructExample::PROTO3_TYPE,
            number: 3,
        },
        name: "vec_nested_struct",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::None,
            typ: EnumExample::PROTO3_TYPE,
            number: 4,
        },
        name: "nested_enum",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Optional,
            typ: EnumExample::PROTO3_TYPE,
            number: 5,
        },
        name: "optional_nested_enum",
    },
    proto3::Field {
        kind: proto3::FieldKind::Normal {
            rule: proto3::FieldRule::Repeated,
            typ: EnumExample::PROTO3_TYPE,
            number: 6,
        },
        name: "vec_nested_enum",
    },
];

impl Proto3 for NestedStructExample {
    const PROTO3_TYPE: proto3::Type = proto3::Type::Message(proto3::Message {
        name: "NestedStructExampleProst",
        fields: &NESTED_STRUCT_EXAMPLE_PROST_FIELDS,
    });
}
