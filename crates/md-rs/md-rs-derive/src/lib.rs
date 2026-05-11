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
            fn render(&self, out: &mut String) {
                out.push_str(#prefix);
                for span in &self.spans {
                    span.render(out);
                }
                out.push('\n');
                out.push('\n');
            }
        }
    }
    .into()
}
