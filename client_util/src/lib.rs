extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let hex = input.value();
    let hex = hex.trim_start_matches('#');
    let len = hex.len();

    let result = match len {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid red component");
            let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid green component");
            let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid blue component");
            quote! {
                iced::Color::from_rgb8(#r, #g, #b)
            }
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid red component");
            let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid green component");
            let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid blue component");
            let a = u8::from_str_radix(&hex[6..8], 16).expect("Invalid alpha component");
            quote! {
                iced::Color::from_rgba8(#r, #g, #b, #a)
            }
        }
        _ => panic!("Hex color must be 6 or 8 characters (e.g. #ffaa00 or #ffaa00ff)"),
    };

    result.into()
}


#[proc_macro]
pub fn svg_handle(input: TokenStream) -> TokenStream {
    // Parse input as a string literal
    let lit = parse_macro_input!(input as LitStr);
    let filename = lit.value();

    // Generate path relative to project root using CARGO_MANIFEST_DIR
    let expanded = quote! {
        iced::widget::svg::Handle::from_memory(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", #filename, ".svg"))
                .as_bytes()
        )
    };

    TokenStream::from(expanded)
}

