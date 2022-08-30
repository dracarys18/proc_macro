extern crate proc_macro;

use proc_macro2::{Ident, Span};

use proc_macro::TokenStream;
use syn::{self, parse_macro_input};

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
