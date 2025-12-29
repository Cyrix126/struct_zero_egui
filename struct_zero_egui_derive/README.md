# README


## WIP


the crate struct_zero_egui_derive bring the declarative macro.

## Features:

TODO

## Examples

See the folder [example](../struct_zero_egui/examples)

## How to Use

Apply trait EguiDisplayon your struct.

```rust,ignore
#[derive(EguiDisplay)]
struct StructExample {
        name: String,
        email: String
}

```
And then you can call the method show()
```rust,ignore
if let Some(struct) = StructExample::show(ui);
```

See the documentation locally, later online.
 
```bash, ignore
cargo doc --open
```
