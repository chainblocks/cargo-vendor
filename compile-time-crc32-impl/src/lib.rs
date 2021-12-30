extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Lit, spanned::Spanned};

fn lit_to_bytes(lit: &Lit) -> Option<Vec<u8>> {
    match lit {
        Lit::Str(lit_str) => {
            Some(lit_str.value().into_bytes())
        }
        Lit::ByteStr(lit_str) => {
            Some(lit_str.value())
        }
        _ => {
            None
        }
    }
}

#[proc_macro_hack]
pub fn crc32(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Lit);

    match lit_to_bytes(&expr) {
        Some(bytes) => {
            let crc = crc::crc32::checksum_ieee(&bytes);
            
            TokenStream::from(quote! {
                (#crc)
            })
        }
        None => {
            let span = expr.span();
            TokenStream::from(quote_spanned!{span =>
                compile_error!("Invalid literal");
            })
        }
    }
    
}
