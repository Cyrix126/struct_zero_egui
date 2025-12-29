# Derive attributs

Here's an explanation of every attributs you can use to customize the egui widget created from your struct

## Struct attributs


- ##### `#[egui_display(title= String )]`

Put a title that will be displayed above your data. Optional. Default is None.

Example: 
```rust,ignore
#[egui_display(title= "My Beautiful Data")]
```

- ##### `#[egui_display(convert_case= String)]`

Convert all the name of fields to the case style given.
For now, only Snake and Title is supported. Title is default.

Example: 
```rust,ignore
#[egui_display(convert_case= "Snake")]
```

- ##### `#[egui_display(hover_enabled= String)]`

Will give the content on_hover_text() for the title widget.

Example: 
```rust,ignore
#[egui_display(hover_enabled= "It's enabled !")]
```

- ##### `#[egui_display(hover_enabled= String)]`

Will give the content on_disabled_hover_text() for the title widget.

Example: 
```rust,ignore
#[egui_display(hover_disabled= "It's disabled !")]
```

## Fields attributs

- ##### `#[egui(name= String)]`

Taken from the name field if None
Will be put before the value and underligned
So in any case a title will appear to describe this value
That's very opiniated
- ##### `#[egui(hover= String)]`

Will give the content on_hover_text() for the field widget 

- ##### `#[egui(hidden= bool)]`

Will skip the field from being displayed

- ##### `#[egui(spacing = f32)]`

Add a space between the name of the field and it's value. The float number is a coefficient to the spacing style.
