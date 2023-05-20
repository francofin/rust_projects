extern crate proc_macro; //imports proc macro crate, provides traits to implement proc macros. 

use proc_macro::TokenStream; //proc macro provides types and func for proc macros, gives access to token stream type.
//Represents a sequence of tokens, takes some nput code, manipulates and returns a token stream representing modified code

use quote::quote; //generate tokenstream from a syntax stream, geenrate rust code using rust syntax
use syn::{parse_macro_input, ItemFn}; //provides a parser for rust syntax, into a syntax tree, mani[ulated by a proc macro. 
//quote takes from syntax tree and turn into a tokenstream. 

#[proc_macro_attribute] //creating own custom macro
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream{
    //attr we ignore, item is actual rust code, modified by macro
    //we pass in a func for attr micro. 
    let mut item_fn = parse_macro_input!(item as ItemFn);

    //sig attr returns a ref to func sig, contains meta data about func. ident returns a ref to func name.
    let ident = &item_fn.sig.ident;
    //insert debug print at begi of func body
    item_fn.block.stmts.insert(0, syn::parse_quote!(println!("Entering Function: {}", stringify!(#ident));), );//first arg is index, second what to insert

    //Return token streamTokenS
    TokenStream::from(quote!{#item_fn})
}