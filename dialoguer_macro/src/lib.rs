extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::{Group, TokenStream as TokenStream2};
use proc_macro_error::abort_call_site;
use proc_macro_error::proc_macro_error;
use quote::quote;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr,
    Fields, FieldsNamed, Type,
};
use syn::{FieldsUnnamed, Token, Variant};

#[derive(Default)]
struct DialogueDefs {
    map: HashMap<String, Expr>,
}

struct DialogueDef {
    key: Ident,
    value: Expr,
}

#[derive(Default)]
struct DialogueDefsParenthesized {
    inner: DialogueDefs,
}

struct ExtraConfig {
    prompt: Option<Expr>,
    name: Option<Expr>,
}

impl Parse for DialogueDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value: Expr = input.parse()?;

        Ok(DialogueDef { key, value })
    }
}

impl Parse for DialogueDefs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let items: Punctuated<DialogueDef, Token![,]> = Punctuated::parse_terminated(input)?;

        let map = items
            .into_iter()
            .map(|dd| (dd.key.to_string(), dd.value))
            .collect();

        Ok(DialogueDefs { map })
    }
}

impl Parse for DialogueDefsParenthesized {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group: Group = input.parse()?;

        let defs: DialogueDefs = syn::parse2(group.stream())?;

        Ok(DialogueDefsParenthesized { inner: defs })
    }
}

#[proc_macro_derive(Dialogue, attributes(dialogue))]
#[proc_macro_error]
pub fn derive_dialogue(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Enum(DataEnum { variants, .. }) => derive_on_enum(variants.iter(), ident),
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => derive_on_struct(fields, ident),
        _ => {
            abort_call_site!("Dialogue must be either Enum or Struct")
        }
    }
    .into()
}

fn derive_on_enum<'a>(variants: impl Iterator<Item = &'a Variant>, ident: Ident) -> TokenStream2 {
    let variants = variants.map(|v| {
        (
            &v.ident,
            extract_type(&v.fields),
            extract_extra_config(&v.attrs),
        )
    });

    let mut opts = Vec::new();
    let mut names = Vec::new();

    for (i, (field, typ, extra_config)) in variants.enumerate() {
        let ExtraConfig { prompt, name } = extra_config;

        if let Some(typ) = typ {
            if prompt.is_none() {
                abort_call_site!("Missing promp attribute on an enum variant");
            }
            opts.push(quote! {
                #i => #ident::#field(<#typ as dialoguer_trait::Dialogue>::compose(#prompt)?),
            });
        } else {
            opts.push(quote! {
                #i => #ident::#field,
            })
        }

        if let Some(name) = name {
            names.push(quote! { #name });
        } else {
            names.push(quote! { stringify!(#field) });
        }
    }

    let opts: TokenStream2 = opts.into_iter().collect();

    quote! {
        impl dialoguer_trait::Dialogue for #ident {
            fn compose(prompt: &str) -> std::io::Result<Self> {
                use dialoguer_trait::dialoguer::Select;
                use dialoguer_trait::dialoguer::theme::ColorfulTheme;

                let selections = [#(#names),*];

                let idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .default(0)
                    .items(&selections[..])
                    .interact()?;

                Ok(match idx {
                    #opts
                    _ => unreachable!(),
                })
            }
        }
    }
}

fn extract_extra_config(attributes: &[Attribute]) -> ExtraConfig {
    let defs: DialogueDefsParenthesized = match attributes
        .iter()
        .find(|attr| attr.path == parse_quote!(dialogue))
    {
        Some(attr) => syn::parse(attr.tokens.clone().into()).expect("Expected Def"),
        None => DialogueDefsParenthesized::default(),
    };

    ExtraConfig {
        prompt: defs.inner.map.get("prompt").cloned(),
        name: defs.inner.map.get("name").cloned(),
    }
}

fn extract_type(fields: &Fields) -> Option<&Type> {
    match fields {
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => match unnamed.len() {
            1 => {
                let field = unnamed.first().unwrap();
                Some(&field.ty)
            }
            _ => abort_call_site!("Only enum variants with single field are supported at a time"),
        },
        Fields::Unit => None,
        _ => abort_call_site!("Only unnamed fields are supported for enums at the time"),
    }
}

fn derive_on_struct(fields: &FieldsNamed, ident: Ident) -> TokenStream2 {
    let mut acc = Vec::new();

    for field in &fields.named {
        let ExtraConfig { prompt, .. } = extract_extra_config(&field.attrs);
        let ident = if let Some(ident) = field.ident.as_ref() {
            ident
        } else {
            abort_call_site!("Missing ident of struct field")
        };
        let typ = &field.ty;

        acc.push(quote! {
            #ident: <#typ as dialoguer_trait::Dialogue>::compose(#prompt)?,
        })
    }

    let fields: TokenStream2 = acc.into_iter().collect();

    quote! {
        impl dialoguer_trait::Dialogue for #ident {
            fn compose(prompt: &str) -> std::io::Result<Self> {
                println!("{}", prompt);

                let result = #ident {
                    #fields
                };

                Ok(result)
            }
        }
    }
}
