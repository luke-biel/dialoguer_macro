extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::{Group, TokenStream as TokenStream2};
use proc_macro_error::abort_call_site;
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
    let variants: Vec<_> = variants
        .map(|v| (&v.ident, extract_type(&v.fields), extract_prompt(&v.attrs)))
        .collect();

    let mut opts = Vec::new();
    let mut names = Vec::new();

    for (i, (field, typ, prompt)) in variants.into_iter().enumerate() {
        opts.push(quote! {
            #i => #ident::#field(<#typ as yaga::Dialogue>::compose(#prompt)?),
        });

        names.push(field);
    }

    let opts: TokenStream2 = opts.into_iter().collect();

    quote! {
        impl yaga::Dialogue for #ident {
            fn compose(prompt: &str) -> std::io::Result<Self> {
                use dialoguer::Select;
                use dialoguer::theme::ColorfulTheme;

                let selections = [#(stringify!(#names)),*];

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

fn extract_prompt(attributes: &Vec<Attribute>) -> Expr {
    let defs: DialogueDefsParenthesized = match attributes
        .iter()
        .find(|attr| attr.path == parse_quote!(dialogue))
    {
        Some(attr) => syn::parse(attr.tokens.clone().into()).expect("Expected Def"),
        None => DialogueDefsParenthesized::default(),
    };

    defs.inner.map.get("prompt").unwrap().clone()
}

fn extract_type(fields: &Fields) -> &Type {
    match fields {
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
            if unnamed.len() != 1 {
                abort_call_site!("Only enum variants with single field are supported at a time")
            } else {
                let field = unnamed.first().unwrap();
                &field.ty
            }
        }
        _ => abort_call_site!("Only unnamed fields are supported for enums at the time"),
    }
}

fn derive_on_struct(fields: &FieldsNamed, ident: Ident) -> TokenStream2 {
    let mut acc = Vec::new();

    for field in &fields.named {
        let prompt = extract_prompt(&field.attrs);
        let ident = field.ident.as_ref().unwrap();
        let typ = &field.ty;

        acc.push(quote! {
            #ident: <#typ as yaga::Dialogue>::compose(#prompt)?,
        })
    }

    let fields: TokenStream2 = acc.into_iter().collect();

    quote! {
        impl yaga::Dialogue for #ident {
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
