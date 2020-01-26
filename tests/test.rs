extern crate syn;

use std::fs;

//use syn::{parse_macro_input, Data, DeriveInput, DataStruct, Fields, Ident, TypePath, Type};

#[test]
fn test_add() {
    let code = fs::read_to_string("rs2ts-proc/tests/test.rs").unwrap();
    let syntax = syn::parse_file(&code).unwrap();

    for item in syntax.items.iter() {
        match item {
            syn::Item::Struct(ref item) => {
                println!("{:?}", item);
            }
            _ => {}
        }
    }

    /*
    match struct_.fields {
                  Fields::Named(ref fields) => {
                      for field in fields.named.iter() {*/
}
