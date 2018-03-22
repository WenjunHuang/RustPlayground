#![feature(proc_macro)]

extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn not_the_bees(_metadata:TokenStream,input:TokenStream) -> TokenStream{
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    let output = quote!{#item};
    input
}