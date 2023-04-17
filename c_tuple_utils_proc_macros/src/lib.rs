extern crate quote;

use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut generated = TokenStream::new();
    generated.append_all(inner());
    generated.into()
}

fn inner() -> TokenStream {
    quote!(let _ = "hi")
}
