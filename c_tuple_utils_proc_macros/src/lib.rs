extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};

#[proc_macro]
pub fn tuple_ext_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut generated = TokenStream::new();
    generated.append_all(quote!(
        pub trait Pluck: Sealed {
            type Head;
            type Tail;
            fn pluck(self) -> (Self::Head, Self::Tail);
        }
        pub trait TupleExt: Sealed {}

        mod sealed {
            pub trait Sealed {}
        }
        use sealed::Sealed;
    ));
    for i in 0..12 {
        generated.append_all(generate_for_size(i));
    }
    generated.into()
}

fn generate_for_size(i: usize) -> TokenStream {
    let names: Vec<_> = (0..=i).into_iter().map(type_ident).collect();
    quote!(
        impl< #(#names, )* > Sealed for ( #(#names, )* ) {}
        impl< #(#names, )* > TupleExt for ( #(#names, )* ) {}
    )
}

fn type_ident(n: usize) -> Ident {
    Ident::new(&format!("T{}", n), Span::call_site())
}

fn number_ident(n: usize) -> Ident {
    Ident::new(&n.to_string(), Span::call_site())
}

// impl< #(#names, )* > Pluck for ( #(#names, )* ) {
//     type Head = T0;
//     type Tail = (T1, T2);
//     fn pluck(self) -> (Self::Head, Self::Tail) {
//         (
//             self.0,
//             (
//                 self.1,
//                 self.2,
//             )
//         )
//     }
// }
