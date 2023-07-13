use types::{EnumExample, NestedStructExample, StructExample};

include!(concat!(env!("OUT_DIR"), "/types_grpc.rs"));

impl From<StructExample> for StructExampleProst {
    fn from(proto3: StructExample) -> Self {
        StructExampleProst {
            string: proto3.string,
            int: proto3.int,
            optional_string: proto3.optional_string,
            vec_string: proto3.vec_string.into_iter().collect(),
        }
    }
}

impl TryFrom<StructExampleProst> for StructExample {
    type Error = ();

    fn try_from(prost: StructExampleProst) -> Result<Self, Self::Error> {
        Ok(Self {
            string: prost.string,
            int: prost.int,
            optional_string: prost.optional_string,
            vec_string: prost.vec_string.into_iter().collect(),
        })
    }
}

impl From<EnumExample> for EnumExampleProst {
    fn from(proto3: EnumExample) -> Self {
        let inner = match proto3 {
            EnumExample::UnitVariant => enum_example_prost::Inner::UnitVariant(()),
            EnumExample::TupleVariant(field_0, field_1) => {
                enum_example_prost::Inner::TupleVariant(EnumExampleProstTupleVariant {
                    field_0: field_0.into(),
                    field_1: field_1.into(),
                })
            }
            EnumExample::StructVariant { string, int } => {
                enum_example_prost::Inner::StructVariant(EnumExampleProstStructVariant {
                    string: string.into(),
                    int: int.into(),
                })
            }
        };
        EnumExampleProst { inner: Some(inner) }
    }
}

impl TryFrom<EnumExampleProst> for EnumExample {
    type Error = ();

    fn try_from(prost: EnumExampleProst) -> Result<Self, Self::Error> {
        match prost.inner.ok_or(())? {
            enum_example_prost::Inner::UnitVariant(()) => Ok(Self::UnitVariant),
            enum_example_prost::Inner::TupleVariant(v) => {
                Ok(Self::TupleVariant(v.field_0, v.field_1))
            }
            enum_example_prost::Inner::StructVariant(v) => Ok(Self::StructVariant {
                string: v.string,
                int: v.int,
            }),
        }
    }
}

impl From<NestedStructExample> for NestedStructExampleProst {
    fn from(proto3: NestedStructExample) -> Self {
        NestedStructExampleProst {
            nested_struct: Some(proto3.nested_struct.into()),
            optional_nested_struct: proto3.optional_nested_struct.map(|v| v.into()),
            vec_nested_struct: proto3
                .vec_nested_struct
                .into_iter()
                .map(|v| v.into())
                .collect(),
            nested_enum: Some(proto3.nested_enum.into()),
            optional_nested_enum: proto3.optional_nested_enum.map(|v| v.into()),
            vec_nested_enum: proto3
                .vec_nested_enum
                .into_iter()
                .map(|v| v.into())
                .collect(),
        }
    }
}

impl TryFrom<NestedStructExampleProst> for NestedStructExample {
    type Error = ();

    fn try_from(prost: NestedStructExampleProst) -> Result<Self, Self::Error> {
        let nested_struct = prost.nested_struct.ok_or(())?.try_into()?;
        let optional_nested_struct = prost
            .optional_nested_struct
            .map(|v: StructExampleProst| v.try_into())
            .transpose()?;
        let vec_nested_struct = prost
            .vec_nested_struct
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let nested_enum = prost.nested_enum.ok_or(())?.try_into()?;
        let optional_nested_enum = prost
            .optional_nested_enum
            .map(|v| v.try_into())
            .transpose()?;
        let vec_nested_enum = prost
            .vec_nested_enum
            .into_iter()
            .map(|v| v.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            nested_struct,
            optional_nested_struct,
            vec_nested_struct,
            nested_enum,
            optional_nested_enum,
            vec_nested_enum,
        })
    }
}

fn main() {
    let example_prost = NestedStructExampleProst::default();
    let example = NestedStructExample::try_from(example_prost).unwrap();
    let example_prost = NestedStructExampleProst::from(example);
    let example: NestedStructExample = example_prost.try_into().unwrap();
    let _: NestedStructExampleProst = example.into();
}
