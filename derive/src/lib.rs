//! derive
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

use proc::{
    DeriveInput, Path, Result, TokenStream,
    quote::{format_ident, quote},
    util::{
        EnumData, EnumMatcher, ForEachField, StructData, StructEnumDeriver,
        VariantExpander, compile_error,
    },
};

#[allow(missing_docs)]
#[proc::derive(name = Claim, host = "trivial")]
fn claim(crate_: Path, item: DeriveInput) -> Result<StructEnumDeriver<Path>> {
    StructEnumDeriver::new(
        "Claim",
        item,
        crate_,
        struct_impl("Claim", "claim"),
        enum_impl("Claim", "claim"),
    )
}

#[allow(missing_docs)]
#[proc::derive(name = Trivial, host = "trivial")]
fn trivial(crate_: Path, item: DeriveInput) -> Result<StructEnumDeriver<Path>> {
    StructEnumDeriver::new(
        "Trivial",
        item,
        crate_,
        struct_impl("Trivial", "dup"),
        enum_impl("Trivial", "dup"),
    )
}

fn struct_impl(
    trait_: &'static str,
    method: &'static str,
) -> impl Fn(&StructData, &Path, &mut TokenStream) {
    move |data, crate_, tokens| {
        let trait_ident = format_ident!("{trait_}");
        let method = format_ident!("{method}");
        let StructData {
            ident,
            generics,
            fields,
        } = data;
        if !generics.params.is_empty() {
            return tokens.extend(compile_error(
                generics,
                format_args!(
                    "#[derive({trait_})] doesn't support generic structs"
                ),
            ));
        }
        let fields = ForEachField(fields, |idx, field| {
            if let Some(ident) = &field.ident {
                quote!(#ident: self.#ident.#method())
            } else {
                quote!(self.#idx.#method())
            }
        });
        tokens.extend(quote! {
            impl #crate_::#trait_ident for #ident {
                fn #method(&self) -> Self {
                    Self #fields
                }
            }
        });
    }
}

fn enum_impl(
    trait_: &'static str,
    method: &'static str,
) -> impl Fn(&EnumData, &Path, &mut TokenStream) {
    move |data, crate_, tokens| {
        let trait_ident = format_ident!("{trait_}");
        let method = format_ident!("{method}");
        let EnumData {
            ident,
            generics,
            variants,
        } = data;
        if variants.is_empty() {
            return tokens.extend(compile_error(
                ident,
                format_args!("cannot #[derive({trait_})] on an empty enum"),
            ));
        }
        if !generics.params.is_empty() {
            return tokens.extend(compile_error(
                generics,
                format_args!(
                    "#[derive({trait_})] doesn't support generic enums"
                ),
            ));
        }
        let matchers = variants.iter().map(EnumMatcher);
        let bodies = variants
            .iter()
            .map(|v| VariantExpander(v, |ident, _| quote!(#ident.#method())));
        tokens.extend(quote! {
            impl #crate_::#trait_ident for #ident {
                fn #method(&self) -> Self {
                    match self {
                        #(Self::#matchers => Self::#bodies),*
                    }
                }
            }
        });
    }
}
