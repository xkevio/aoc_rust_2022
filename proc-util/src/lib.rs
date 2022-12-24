use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(RotateEnum)]
pub fn derive_rotate_enum(item: TokenStream) -> TokenStream {
    let ts = parse_macro_input!(item as DeriveInput);
    let ident = ts.ident;

    let variants = if let Data::Enum(e) = ts.data {
        e.variants.into_iter().collect::<Vec<_>>()
    } else {
        panic!("RotateEnum can only be used on enums!")
    };

    let mut nexts = variants.clone();
    nexts.rotate_left(1);

    quote!(
        impl #ident {
            pub fn prev(&self) -> #ident {
                match &self {
                    #(Self::#nexts => Self::#variants),*
                }
            }

            pub fn next(&self) -> #ident {
                match &self {
                    #(Self::#variants => Self::#nexts),*
                }
            }
        }
    )
    .into()
}
