use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

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
    f.attrs
        .iter()
        .find(|&i| i.path.segments.len() == 1 && i.path.segments[0].ident == "def")
}

/// Parses the attribute value
fn get_default(a: &syn::Attribute) -> Option<syn::MetaNameValue> {
    if let Ok(syn::Meta::NameValue(v)) = a.parse_meta() {
        return Some(v);
    }
    None
}
