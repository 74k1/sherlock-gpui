use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod builder;

#[proc_macro_derive(ComponentBuilder, attributes(md_rs))]
pub fn derive_component_builder(input: TokenStream) -> TokenStream {
    builder::derive_builder(input)
}

#[proc_macro_derive(TextCompomponent, attributes(md_rs))]
pub fn derive_text_component(input: TokenStream) -> TokenStream {
    builder::derive_text_component(input)
}

#[proc_macro_derive(SpanNode, attributes(span_node))]
pub fn derive_span_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let prefix = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("span_node"))
        .and_then(|a| {
            a.parse_args::<syn::MetaNameValue>().ok().and_then(|mnv| {
                if mnv.path.is_ident("prefix")
                    && let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = mnv.value
                {
                    return Some(s.value());
                }
                None
            })
        })
        .unwrap_or_default();

    let write_prefix = if prefix.is_empty() {
        quote! {}
    } else {
        quote! { write!(out, #prefix)?; }
    };

    quote! {
        impl ::md_rs::components::Component for #name {
            fn render_inline(&self, out: &mut dyn ::std::fmt::Write) -> ::std::fmt::Result {
                #write_prefix
                for (i, span) in self.spans.iter().enumerate() {
                    if i > 0 {
                        let prev = &self.spans[i - 1];
                        let needs_space = span.needs_space_before()
                            && !matches!(prev, ::md_rs::components::span::Span::LineBreak);
                        if needs_space {
                            write!(out, " ")?;
                        }
                    }
                    span.render(out)?;
                }
                Ok(())
            }
        }
    }
    .into()
}

#[proc_macro_derive(ComponentConstructor, attributes(md_rs))]
pub fn derive_component_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fn_name = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("md_rs"))
        .and_then(|a| {
            a.parse_args::<syn::MetaNameValue>().ok().and_then(|mnv| {
                if mnv.path.is_ident("rename")
                    && let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = mnv.value
                {
                    return Some(syn::Ident::new(&s.value(), name.span()));
                }
                None
            })
        })
        .unwrap_or_else(|| syn::Ident::new(&name.to_string().to_lowercase(), name.span()));

    quote! {
        pub fn #fn_name() -> #name {
            #name::default()
        }
    }
    .into()
}

#[proc_macro_derive(HeadingConstructors)]
pub fn iderive_heading_constructors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let constructors = (1u8..=6).map(|level| {
        let fn_name = syn::Ident::new(&format!("h{level}"), proc_macro2::Span::call_site());
        quote! {
            pub fn #fn_name(heading: impl Into<::md_rs::components::span_nodes::Paragraph>) -> #name {
                #name::from(heading).level(#level)
            }
        }
    });

    quote! { #(#constructors)* }.into()
}

#[proc_macro_derive(ParentComponent, attributes(children))]
pub fn derive_parent_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("ParentComponent can only be derived for structs."),
    };

    let children_field = match fields {
        syn::Fields::Named(f) => f.named.iter().find(|f| {
            f.ident.as_ref().is_some_and(|i| i == "children")
                || f.attrs.iter().any(|a| a.path().is_ident("children"))
        }),
        _ => panic!("ParentComponent requires named fields"),
    }
    .expect("No `children` field found - add one or mark it with #[children]");

    let field_name = &children_field.ident;

    quote! {
        impl ::md_rs::components::ParentComponentExt for #name {
            fn child(mut self, child: impl ::md_rs::components::IntoComponent + 'static) -> Self {
                self.#field_name.push(Box::new(child.into_component()));
                self
            }
            fn children(
                mut self,
                children: impl IntoIterator<Item = impl ::md_rs::components::IntoComponent + 'static>,
            ) -> Self {
                self.#field_name.extend(
                    children
                        .into_iter()
                        .map(|c|
                            Box::new(c.into_component()) as Box<dyn ::md_rs::components::Component>
                        ),
                );
                self
            }
        }
    }.into()
}
