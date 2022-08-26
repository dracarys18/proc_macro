extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{self, parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn context(_: TokenStream, input: TokenStream) -> TokenStream {
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
fn unwrap(f: syn::Item) -> Result<(), syn::Error> {
    if let syn::Item::Fn(f) = f {
        let stmts = f.block.stmts;
        for s in stmts.iter() {
            match s {
                syn::Stmt::Semi(t, _) => {
                    if let syn::Expr::MethodCall(m) = t {
                        let ident = &m.method;
                        if ident.eq(&Ident::new("unwrap", Span::call_site())) {
                            return Err(syn::Error::new(s.span(), "Gaand ke andhe unwrap hatha"));
                        }
                    }
                }
                _ => {}
            };
        }
    }
    Ok(())
}
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
#[proc_macro]
pub fn wtf(_: TokenStream) -> TokenStream {
    "pub fn lmao()->u32{2}".parse().unwrap()
}
