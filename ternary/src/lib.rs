use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, Expr, Token};

struct TernaryStruct {
    predicate: Expr,
    if_true: Expr,
    if_false: Expr,
}

impl Parse for TernaryStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let predicate = Expr::parse(input)?;
        <Token![?]>::parse(input)?;
        let if_true = Expr::parse(input)?;
        <Token![:]>::parse(input)?;
        let if_false = Expr::parse(input)?;
        Ok(Self {
            predicate,
            if_true,
            if_false,
        })
    }
}
impl Into<proc_macro2::TokenStream> for TernaryStruct {
    fn into(self) -> proc_macro2::TokenStream {
        let condition = self.predicate;
        let if_true = self.if_true;
        let if_false = self.if_false;
        quote! {
            if #condition {
                #if_true
            } else {
                #if_false
            }
        }
    }
}

#[proc_macro]
pub fn ternary(tokenstream: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = parse_macro_input!(tokenstream as TernaryStruct).into();
    input.into()
}
