pub use proto3;

use std::collections::{BTreeSet, HashSet};

use proto3::*;

fn type_name(typ: Type) -> &'static str {
    match typ {
        Type::String => "string",
        Type::Int32 => "int32",
        Type::Message(msg) => msg.name,
    }
}

fn field_code(field: Field) -> String {
    match field.kind {
        FieldKind::Normal { rule, typ, number } => {
            let rule = match rule {
                FieldRule::None => "",
                FieldRule::Optional => "optional ",
                FieldRule::Repeated => "repeated ",
            };
            format!("{}{} {} = {};", rule, type_name(typ), field.name, number)
        }
        FieldKind::Oneof(fields) => {
            let mut code = format!("oneof {} {{", field.name);
            for field in fields {
                code.push_str(&format!("\n    {}", field_code(*field)));
            }
            code.push_str("\n}");
            code
        }
    }
}

fn message_code(message: Message) -> String {
    let mut code = format!("message {} {{", message.name);
    for field in message.fields {
        code.push_str(&format!("\n    {}", field_code(*field)));
    }
    code.push_str("\n}");
    code
}

#[derive(Debug, Clone, Copy)]
struct ExternalMessage {
    message: Message,
    import: &'static str,
    used: bool,
}

fn add_fields_recursive(
    fields: &[Field],
    external: &mut [ExternalMessage],
    generated: &HashSet<Message>,
    not_generated: &mut BTreeSet<Message>,
) {
    for field in fields {
        match field.kind {
            FieldKind::Normal { typ, .. } => {
                if let Type::Message(message) = typ {
                    if let Some(external) = external.iter_mut().find(|item| item.message == message)
                    {
                        external.used = true;
                        continue;
                    }

                    if !generated.contains(&message) {
                        not_generated.insert(message);
                    }
                }
            }
            FieldKind::Oneof(fields) => {
                add_fields_recursive(fields, external, generated, not_generated);
            }
        }
    }
}

pub fn gen(root_type: proto3::Type, package: &str) -> String {
    let mut external = [ExternalMessage {
        message: EMPTY,
        import: "google/protobuf/empty.proto",
        used: false,
    }];

    let mut code = "syntax = \"proto3\";\n\n".to_string();
    code.push_str(&format!("package {};\n\n", package));

    let Type::Message(message) = root_type else {
        return code;
    };

    let mut generated = HashSet::new();

    let mut not_generated = BTreeSet::new();
    if !generated.contains(&message) {
        not_generated.insert(message);
    }

    while let Some(message) = not_generated.pop_first() {
        code.push_str(&format!("{}\n\n", message_code(message)));
        assert!(generated.insert(message));

        add_fields_recursive(
            message.fields,
            &mut external,
            &generated,
            &mut not_generated,
        );
    }

    for external in external.iter().filter(|item| item.used) {
        code.push_str(&format!("import \"{}\";\n", external.import));
    }

    code
}
