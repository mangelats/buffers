#[macro_use]
extern crate quote;

use quote::quote;

#[proc_macro]
pub fn tuple_ext_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}
