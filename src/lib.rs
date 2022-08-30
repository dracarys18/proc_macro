extern crate proc_macro;

use proc_macro2::{Ident, Span};
use syn::spanned::Spanned;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

/// Detects if there's a reference in a parameter returns error if there is
#[proc_macro_attribute]
pub fn reference(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut out = input.clone();
    let t = parse_macro_input!(input as syn::Item);
    if let Err(e) = param(t) {
        out.extend(TokenStream::from(e.to_compile_error()));
    }
    out
}

/// Detects if there's an unwrap in the function and returns error if there is one
#[proc_macro_attribute]
pub fn no_unwrap(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut out = input.clone();
    let t = parse_macro_input!(input as syn::Item);
    if let Err(e) = unwrap(t) {
        out.extend(TokenStream::from(e.to_compile_error()));
    }
    out
}

/// A derive macro which generates the builder struct for any parent struct
#[proc_macro_derive(TestBuilder, attributes(def))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let build_struct = format!("{}Builder", ident);
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

    //For setting the fields from build struct to parent struct
    let build_f = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let d = default_val(f);
        if let Some(v) = d {
            // TODO: Remove this unwrap
            let t = get_default(v).unwrap();
            quote! {
            #name: self.#name.clone().unwrap_or(#t.parse().unwrap())
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(format!("{} is not set bruh. Set it with {}::with_{}",stringify!(#name),stringify!(#struc),stringify!(#name)))?
            }
        }
    });

    // Methods in the build struct like if the struct is
    // Struct i {n: u32}
    // this will be
    // pub fn set_n(&mut self,n: u32)
    let build_methods = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let method_name = format!("with_{}", name);
        let method_ident = syn::Ident::new(&method_name, name.span());
        let ty = &f.ty;
        quote! {
            pub fn #method_ident(&mut self,val:#ty)->&mut Self{
                self.#name = std::option::Option::Some(val);
                self
            }
        }
    });
    // Function that returns parent struct when you do Builder::build
    let build_function = quote! {
        pub fn build(&self)-> std::result::Result<#ident,std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#ident {
                     #(#build_f,)*
                })
            }
    };
    let output = quote! {
    #[derive(Default)]
    struct #struc {
        #(#build_fields,)*
    }
    impl #struc {
        #(#build_methods)*
        #build_function
    }
    impl #ident {
            pub fn builder()->#struc {
                Default::default()
            }
        }

    };
    output.into()
}

/// Get the default val from attrs
fn default_val(f: &syn::Field) -> Option<&syn::Attribute> {
    for i in &f.attrs {
        if i.path.segments.len() == 1 && i.path.segments[0].ident == "def" {
            return Some(i);
        }
    }
    None
}

/// Parses the attribute value
fn get_default(a: &syn::Attribute) -> Option<syn::MetaNameValue> {
    if let Ok(syn::Meta::NameValue(v)) = a.parse_meta() {
        return Some(v);
    }
    None
}
/// Helper function which detects unwrap and returns error if detected
fn unwrap(f: syn::Item) -> Result<(), syn::Error> {
    if let syn::Item::Fn(f) = f {
        let stmts = f.block.stmts;
        for s in stmts.iter() {
            if let syn::Stmt::Semi(syn::Expr::MethodCall(m), _) = s {
                let ident = &m.method;
                if ident.eq(&Ident::new("unwrap", Span::call_site())) {
                    return Err(syn::Error::new(ident.span(), "Gaand ke andhe unwrap hatha"));
                }
            }
        }
    }
    Ok(())
}

/// Helper function which detects reference in the parameters
fn param(f: syn::Item) -> Result<(), syn::Error> {
    if let syn::Item::Fn(f) = f {
        for e in f.sig.inputs.iter() {
            if let syn::FnArg::Typed(t) = e {
                if matches!(*t.ty, syn::Type::Reference(_)) {
                    return Err(syn::Error::new(e.span(), "Gaand ke andhe".to_string()));
                }
            }
        }
    }
    Ok(())
}
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

#[proc_macro]
pub fn wtf(_: TokenStream) -> TokenStream {
    "pub fn lmao()->u32{2}".parse().unwrap()
}
