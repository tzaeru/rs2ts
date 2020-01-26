extern crate proc_macro;
extern crate syn;
extern crate rs2ts;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, DataStruct, Fields, Ident, TypePath, Type};
use rs2ts::parse_to_ts;

#[proc_macro_derive(ParseToTS, attributes(ts_type))]
pub fn parse_to_ts_macro(input: TokenStream) -> TokenStream {    
    // Build a syntax tree from the input TokenStream
    let di: DeriveInput = parse_macro_input!(input);
    println!("{:?}", di.ident.to_string());
    parse_to_ts(di);
    "".parse().unwrap()
}
