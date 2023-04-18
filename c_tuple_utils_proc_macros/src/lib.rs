extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::Index;

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
    for i in 0..3 {
        generated.append_all(generate_sealed(i));
    }
    generated.into()
}

fn generate_sealed(i: usize) -> TokenStream {
    let names: Vec<_> = (0..i).map(type_ident).collect();

    quote!(
        impl< #(#names, )* > Sealed for ( #(#names, )* ) {}
    )
}

fn generate_pluck(i: usize) -> TokenStream {
    if i == 0 {
        quote!(
            impl Pluck for () {
                type Head = ();
                type Tail = ();
                fn pluck(self) -> (Self::Head, Self::Tail) {
                    ()
                }
            }
        )
    } else {
        let head = type_ident(0);
        let tail: Vec<_> = (1..i).map(type_ident).collect();
        let fields: Vec<_> = (1..i).map(Index::from).collect();
        quote!(
            impl< #head, #(#tail, )* > Pluck for ( #head, #(#tail, )* ) {
                type Head = #head;
                type Tail = ( #(#tail, )* );
                fn pluck(self) -> (Self::Head, Self::Tail) {
                    (
                        self.0,
                        ( #(#fields,)* )
                    )
                }
            }
        )
    }
}

fn type_ident(n: usize) -> Ident {
    Ident::new(&format!("T{}", n), Span::call_site())
}
