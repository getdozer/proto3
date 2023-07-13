use proto3_codegen::proto3::Proto3;
use types::NestedStructExample;

fn main() -> std::io::Result<()> {
    let proto_code = proto3_codegen::gen(NestedStructExample::PROTO3_TYPE, "types_prost");

    std::fs::write("src/types_prost.proto", &proto_code)?;

    prost_build::compile_protos(&["src/types_prost.proto"], &["src/"])?;

    Ok(())
}
