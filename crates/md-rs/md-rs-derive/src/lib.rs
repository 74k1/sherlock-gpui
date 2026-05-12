use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

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
            fn render(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
                write!(out, #prefix)?;
                for span in &self.spans {
                    span.render(out)?;
                }
                writeln!(out)?;
                writeln!(out)
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
