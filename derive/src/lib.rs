#![allow(clippy::derive_partial_eq_without_eq)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::*;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(IntoStruct)]
pub fn derive_into_struct(inp: TokenStream) -> TokenStream {
    let inp = parse_macro_input!(inp as DeriveInput);

    let old_name = inp.ident;
    let new_name = Ident::new(&format!("{}Struct", old_name), Span::call_site());

    if let Data::Enum(DataEnum { variants, .. }) = inp.data {
        let enum_fields = variants
            .iter()
            .flat_map(|v| {
                let n = Ident::new(&format!("{}", v.ident).to_lowercase(), Span::call_site());
                match &v.fields {
                    Fields::Unit => vec![quote!( #n: bool, )],
                    Fields::Named(_) => panic!("I hate names! Make them stop!"),
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let n_set = Ident::new(
                            &format!("{}_set", v.ident).to_lowercase(),
                            Span::call_site(),
                        );
                        vec![quote!( #n_set: bool, )]
                            .into_iter()
                            .chain(unnamed.iter().enumerate().map(|(i, u)| {
                                let ty = &u.ty;
                                let n = Ident::new(
                                    &format!("{}_{}", v.ident, i).to_lowercase(),
                                    Span::call_site(),
                                );

                                quote!(#n: #ty,)
                            }))
                            .collect::<Vec<_>>()
                    }
                }
            })
            .collect::<Vec<_>>();
        let derive_fields = variants
            .iter()
            .flat_map(|v| {
                let n = Ident::new(&format!("{}", v.ident).to_lowercase(), Span::call_site());
                let oldname = old_name.clone();
                let vi = &v.ident;

                match &v.fields {
                    Fields::Unit => vec![quote! { #n: matches!(self, #oldname::#v), }],
                    Fields::Named(_) => panic!("I hate names! Make them stop!"),
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let n_set = Ident::new(
                            &format!("{}_set", v.ident).to_lowercase(),
                            Span::call_site(),
                        );
                        assert!(
                            unnamed.len() < 52,
                            "We can only restructure touples up to 52 variants long."
                        );

                        let vars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMOPQRSTUVWXYZ"
                            .chars()
                            .map(|x| Ident::new(&format!("{}", x), Span::call_site()))
                            .take(unnamed.len())
                            .collect::<Vec<_>>();

                        let blanks = std::iter::repeat(quote!(_))
                            .take(unnamed.len())
                            .collect::<Vec<_>>();

                        vec![quote! { #n_set: matches!(self, #oldname::#vi(# (#blanks),*)), }]
                            .into_iter()
                            .chain(unnamed.iter().enumerate().map(|(i, u)| {
                                let ty = &u.ty;
                                let n = Ident::new(
                                    &format!("{}_{}", v.ident, i).to_lowercase(),
                                    Span::call_site(),
                                );
                                let x = &vars[i];
                                quote!(#n: match self {
                                    #oldname::#vi(#( #vars ),*) => {
                                        #x.to_owned()
                                    }
                                    _ => #ty::default(),
                                },)
                            }))
                            .collect::<Vec<_>>()
                    }
                }
            })
            .collect::<Vec<_>>();

        let final_code = quote! {
            impl IntoStruct<#new_name> for #old_name {
                fn as_struct(&self) -> #new_name {
                    #new_name {
                        #( #derive_fields )*
                    }
                }
            }
            #[derive(Debug, PartialEq, Clone)]
            pub struct #new_name {
                #( pub #enum_fields )*
            };
        };
        return proc_macro2::TokenStream::from(final_code).into();
    } else {
        panic!("Not an enum.")
    };
}
