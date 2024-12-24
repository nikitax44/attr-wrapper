#![forbid(unsafe_code)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_me(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    if !attr.is_empty() {
        return quote! {
            compiler_error!("`time_me` must be used without arguments");
            #input
        }
        .into();
    }

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let statements = block.stmts;

    let function_identifier = sig.ident.clone();

    quote!(
        #(#attrs)*
        #vis #sig {
            struct PerfGuard(crate::Instant);
            impl ::core::ops::Drop for PerfGuard {
                fn drop(&mut self) {
                    ::log::debug!("perf: {} took {:?}", stringify!(#function_identifier), self.0.elapsed());
                }
            }
            let _guard = PerfGuard(crate::Instant::now());

            #(#statements)*
        }
    )
    .into()
}
