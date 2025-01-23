#![forbid(unsafe_code)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{parse_macro_input, ExprLit, ItemFn, Lit};

#[proc_macro_attribute]
pub fn time_me(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let thresh = if attr.is_empty() {
        core::time::Duration::from_millis(50)
    } else {
        parse_macro_input!(attr as Dur).0
    };
    let thr_sec = thresh.as_secs();
    let thr_nan = thresh.subsec_nanos();

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
                    let elapsed = self.0.elapsed();
                    if (elapsed > ::core::time::Duration::new(#thr_sec, #thr_nan)) {
                        ::log::debug!("perf: {} took {:?}", stringify!(#function_identifier), elapsed);
                    }
                }
            }
            let _guard = PerfGuard(crate::Instant::now());

            #(#statements)*
        }
    )
    .into()
}

struct Dur(core::time::Duration);

impl Parse for Dur {
    fn parse(input: ParseStream) -> Result<Self> {
        let ExprLit { attrs, lit } = ExprLit::parse(input)?;
        if !attrs.is_empty() {
            return Err(Error::new(lit.span(), "unexpected attributes"));
        }
        let Lit::Int(lit) = lit else {
            return Err(Error::new(lit.span(), "unexpected literal: expected Int"));
        };
        if lit.suffix() != "ms" {
            return Err(Error::new(
                lit.span(),
                format_args!("unexpected literal suffix: {}, expected `ms`", lit.suffix()),
            ));
        }
        let ms: u64 = lit.base10_parse()?;
        Ok(Self(core::time::Duration::from_millis(ms)))
    }
}
