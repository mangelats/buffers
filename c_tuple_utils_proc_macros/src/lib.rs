extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut generated = TokenStream::new();
    for i in 0..12 {
        generated.append_all(generate_for_size(i));
    }
    generated.into()
}

fn generate_for_size(i: usize) -> TokenStream {
    let names: Vec<_> = (0..=i).into_iter().map(|n| (n, type_ident(n))).collect();
    quote!()
}

fn type_ident(n: usize) -> Ident {
    Ident::new(&format!("T{}", n), Span::call_site())
}
