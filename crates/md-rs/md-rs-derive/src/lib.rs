use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod builder;

#[proc_macro_derive(ComponentBuilder, attributes(md_rs))]
pub fn derive_component_builder(input: TokenStream) -> TokenStream {
    builder::derive_builder(input)
}

#[proc_macro_derive(SpanNode, attributes(span_node))]
pub fn derive_span_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let prefix = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("span_node"))
        .and_then(|a| a.parse_args::<syn::LitStr>().ok())
        .map(|l| l.value())
        .unwrap_or_default();

    quote! {
        impl #name {
            pub fn new() -> Self {
                Self { spans: Vec::new() }
            }
            pub fn span(mut self, span: Span) -> Self {
                self.spans.push(span);
                self
            }
        }

        impl Component for #name {
            fn render_inline(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
                write!(out, #prefix)?;
                for span in &self.spans {
                    span.render(out)?;
                }
                Ok(())
            }
        }
    }
    .into()
}

#[proc_macro_derive(ComponentConstructor)]
pub fn derive_component_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fn_name = syn::Ident::new(&name.to_string().to_lowercase(), name.span());

    quote! {
        pub fn #fn_name() -> #name {
            #name::default()
        }
    }
    .into()
}

#[proc_macro_derive(HeadingConstructors)]
pub fn derive_heading_constructors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let constructors = (1u8..=6).map(|level| {
        let fn_name = syn::Ident::new(&format!("h{level}"), proc_macro2::Span::call_site());
        quote! {
            pub fn #fn_name() -> #name {
                #name::default().level(#level)
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
