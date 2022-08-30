extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};
/// Proc macro to create Optional struct
#[proc_macro_attribute]
pub fn optional(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let attrs = &input.attrs;
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
    let new_struct: TokenStream = quote! {
    #[automatically_derived]
    #(#attrs)*
    pub struct #struc {
        #(#build_fields,)*
    }
    }
    .into();
    output.extend(new_struct);
    output
}
