use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Builder can only be derived for structs"),
    };
    let methods = match fields {
        syn::Fields::Named(f) => f
            .named
            .iter()
            .filter_map(|f| {
                // skip fields marked #[skip]
                if f.attrs.iter().any(|a| {
                    a.path().is_ident("md_rs")
                        && a.parse_args::<syn::Ident>()
                            .is_ok_and(|i| i == "skip_builder")
                }) {
                    return None;
                }

                let ident = &f.ident;
                let ty = &f.ty;
                // if Option<T>, unwrap to T for the setter
                let (method_ty, assignment) = if let Some(inner) = unwrap_option(ty) {
                    (inner, quote! { self.#ident = Some(val.into()); })
                } else {
                    (ty.clone(), quote! { self.#ident = val.into(); })
                };
                Some(quote! {
                    pub fn #ident(mut self, val: impl Into<#method_ty>) -> Self {
                        #assignment
                        self
                    }
                })
            })
            .collect::<Vec<_>>(),
        _ => panic!("Builder requires named fields"),
    };

    quote! {
        impl #name {
            #(#methods)*
        }
    }
    .into()
}

fn unwrap_option(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if seg.ident != "Option" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return Some(inner.clone());
        }
    }
    None
}

pub fn derive_text_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("SpanContainer can only be derived for structs"),
    };

    let span_field = match fields {
        syn::Fields::Named(f) => f.named.iter().find(|f| {
            f.ident.as_ref().is_some_and(|i| i == "spans")
                || f.attrs.iter().any(|a| {
                    a.path().is_ident("md_rs")
                        && a.parse_args::<syn::Ident>().is_ok_and(|i| i == "spans")
                })
        }),
        _ => panic!("SpanContainer requires named fields"),
    }
    .expect("No `spans` field found — add one or mark it with #[md_rs(spans)]");

    let field_name = &span_field.ident;

    quote! {
        impl ::md_rs::components::TextComponentExt for #name {
            fn spans_mut(&mut self) -> &mut Vec<::md_rs::components::span::Span> {
                &mut self.#field_name
            }
        }
    }
    .into()
}
