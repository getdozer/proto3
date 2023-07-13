# Proto3

Generate `.proto` code from Rust code.

## PoC

Currently this project includes two crates that are supposed to be used by users: `proto3` and `proto3-codegen`.

A Rust struct implements `proto3::Proto3` trait so it can be used in codegen.

The PoC includes two other crates that uses them. `poc/types` includes several types that implement `proto3::Proto3` trait.

`poc/types-prost` uses `proto3-codegen` in its build script to generate `poc/types-prost/src/types_prost.proto`. To make the generated code actually useful, the build script also uses `prost-build` to generate Rust code from the `.proto` file.

That Rust code is included in `poc/types-prost` and conversion functions are implemented between types in `poc/types` and their `prost::Message` counterparts.

The next step is to automate the `proto3::Proto3` implementation and the conversion function implementation, probably using procedural macros.

The end goal looks like this:

In `poc/types`:

```rust
#[derive(Proto3)]
struct NestedStructExample {
    ...
}
```

In `poc/types-prost`:

```rust
impl_conversion!(NestedStructExample);
```
