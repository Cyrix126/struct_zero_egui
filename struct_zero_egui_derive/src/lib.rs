#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use convert_case::{Case, Casing};
use darling::{FromAttributes, util::parse_expr};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{self, Attribute, Data, Expr};

#[derive(FromAttributes, Debug)]
#[darling(attributes(egui))]
struct FieldOpts {
    /// Taken from the name field if None
    /// Will be put before the value and underligned
    /// So in any case a title will appear to describe this value
    /// That's very opiniated
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    name: Option<Expr>,
    /// Add a text on hover mouse
    /// Do not add one if None
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    hover: Option<Expr>,
    /// hide the field, false (so visisble) by default.
    hidden: Option<bool>,
    /// the value u32 is the ratio of the space based on the default spacing.
    spacing: Option<f32>,
}

struct FieldParams {
    name: TokenStream,
    hover: TokenStream,
    hidden: bool,
    /// If Some, add a space beetwen the value and the title
    spacing: TokenStream,
}
#[derive(FromAttributes, Debug)]
#[darling(attributes(egui_display))]
struct StructOpts {
    /// Convert the field name Case.
    /// All variants of convert_case::Case are supported
    pub convert_case: Option<String>,
    /// Title that will represent all the data
    /// If None, No title will be displayed
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    title: Option<Expr>,
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    hover_enabled: Option<Expr>,
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    hover_disabled: Option<Expr>,
}

struct GlobalParams<'a> {
    convert_case: Case<'a>,
    title: Option<TokenStream>,
    type_struct: &'a Ident,
    hover_enabled: TokenStream,
    hover_disabled: TokenStream,
}
fn opts2global(ast: &'_ syn::DeriveInput) -> GlobalParams<'_> {
    let attrs_struct = StructOpts::from_attributes(&ast.attrs).expect("Wrong attributes on struct");
    let type_struct = &ast.ident;
    let convert_case = find_case(attrs_struct.convert_case.as_deref());
    let title = attrs_struct.title.map(|t| quote! {#t});
    if title.is_none()
        && (attrs_struct.hover_disabled.is_some() || attrs_struct.hover_enabled.is_some())
    {
        panic!(
            "title struct param should be set if either title_hover_enabled or title_hover_disabled is set"
        );
    }
    let hover_enabled = option_to_stream(&attrs_struct.hover_enabled);
    let hover_disabled = option_to_stream(&attrs_struct.hover_disabled);
    GlobalParams {
        title,
        convert_case,
        type_struct,
        hover_enabled,
        hover_disabled,
    }
}

fn find_case(case: Option<&str>) -> convert_case::Case<'static> {
    if let Some(case) = case {
        match case {
            "snake" => convert_case::Case::Snake,
            "title" => convert_case::Case::Title,
            _ => panic!("Case naming not supported ! Support \"title\"(default), \"snake\""),
        };
    }
    convert_case::Case::Title
}

/// This is the derive trait to add to struct you want to be egui DisplayAble
#[doc = include_str!("../doc/attributs_derive.md")]
#[proc_macro_derive(EguiDisplay, attributes(egui_display, egui))]
pub fn struct_zero_egui_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_egui_display(&ast)
}

fn fields(data: &Data) -> Vec<(&Ident, &Vec<Attribute>)> {
    let data_struct = match data {
        Data::Struct(s) => s,
        _ => panic!("struct_zero_egui can only be used on structs !"),
    };

    data_struct
        .fields
        .iter()
        .map(|field| (field.ident.as_ref().unwrap(), &field.attrs))
        .collect::<Vec<_>>()
}

fn get_opts_field<'a>(opts: &'a FieldOpts, ident: &'a Ident, case: Case<'_>) -> FieldParams {
    let name = if let Some(n) = &opts.name {
        n.into_token_stream()
    } else {
        ident.to_string().to_case(case).into_token_stream()
    };
    let hover = option_to_stream(&opts.hover);
    let spacing = option_to_stream(&opts.spacing);
    let hidden = opts.hidden.unwrap_or_default();
    FieldParams {
        name,
        hover,
        hidden,
        spacing,
    }
}

/// Generate the code for the implementation
fn impl_egui_display(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let global_params = opts2global(ast);

    let mut fields_struct = vec![];

    for (ident, attrs) in fields(&ast.data).into_iter() {
        let opts: FieldOpts = FieldOpts::from_attributes(attrs).expect("Wrong attributs on field");
        let FieldParams {
            name,
            hover,
            hidden,
            spacing,
        } = get_opts_field(&opts, ident, global_params.convert_case);

        if !hidden {
            let field = ident;
            fields_struct.push(quote! {
                        struct_zero_egui::FieldParams {
                    name: #name,
                    value: &self.#field,
                    hover: #hover,
                    spacing: #spacing
                }
            });
        }
    }

    let return_title = if let Some(title) = global_params.title {
        let hover_enabled = global_params.hover_enabled;
        let hover_disabled = global_params.hover_disabled;
        quote! {
                Some(struct_zero_egui::TitleParams {
                    value: #title,
                    hover_enabled: #hover_enabled,
                    hover_disabled: #hover_disabled,
                })
        }
    } else {
        quote! {
            None
        }
    };

    // panic!("{:?}", return_title);
    let name_type_struct = global_params.type_struct;

    quote! {
    impl struct_zero_egui::EguiDisplay for #name_type_struct {

        fn title(&self) -> Option<struct_zero_egui::TitleParams<'static>> {
            #return_title
        }
        fn fields(&self) -> impl Iterator<Item = struct_zero_egui::FieldParams<'_>> {
                [
                                #(#fields_struct),*,
                ].into_iter()
        }
    }
    }
    .into()
}

/// Need to convert to TokenStream, else quote will put it as a litteral with \".
fn option_to_stream<T: darling::ToTokens>(value: &Option<T>) -> TokenStream {
    if let Some(v) = value {
        quote! {
            Some(#v)
        }
    } else {
        quote! {
            None
        }
    }
}
