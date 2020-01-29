extern crate syn;
extern crate rs2ts;

use std::fs;
use rs2ts::parse_from_filepath;


//use syn::{parse_macro_input, Data, DeriveInput, DataStruct, Fields, Ident, TypePath, Type};

#[test]
fn test_add() {

    parse_from_filepath("rs2ts-proc/tests/test.rs");
    /*let code = fs::read_to_string("rs2ts-proc/tests/test.rs").unwrap();
    let syntax = syn::parse_file(&code).unwrap();

    for item in syntax.items.iter() {
        match item {
            syn::Item::Struct(ref item) => {
                println!("{:?}", item);
            }
            _ => {}
        }
    }*/

    /*
    match struct_.fields {
                  Fields::Named(ref fields) => {
                      for field in fields.named.iter() {*/
}
