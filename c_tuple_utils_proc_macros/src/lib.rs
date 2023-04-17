extern crate quote;

use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut generated = TokenStream::new();
    for i in 0..12 {
        generated.append_all(generate_for_size(i));
    }
    generated.into()
}

fn generate_for_size(i: usize) -> TokenStream {
    quote!(let _ = "hi")
}
