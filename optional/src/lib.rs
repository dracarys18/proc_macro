extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};
use syn::{parse_quote, Attribute};
/// Proc macro to create Optional struct
#[proc_macro_attribute]
pub fn optional(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut attrs = input.attrs.clone();
    let build_struct = format!("Optional{}", ident);
    // name of the builder struct
    let struc = syn::Ident::new(&build_struct, ident.span());
    // All the fields in the parent struct
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("You can't use this proc-macro on structs without fields");
    };

    // For declaring fields in the struct
    let build_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! {
            #name: std::option::Option<#ty>
        }
    });
    // If the parent struct didn't have an default implementation
    // then explicitly add Default implementation for Optional Struct
    input.attrs.iter().for_each(|f| {
        if !f.tokens.to_string().contains("Default") {
            let default: Attribute = parse_quote! {
                #[derive(Default)]
            };
            attrs.push(default);
            return;
        }
    });
    let new_struct: TokenStream = quote! {
    #(#attrs)*
    pub struct #struc {
        #(#build_fields,)*
    }
    impl #ident {
        pub fn optional()->#struc{
            #struc::default()
        }
    }
    }
    .into();
    output.extend(new_struct);
    output
}
