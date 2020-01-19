extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, DataStruct, Fields, Ident, TypePath, Type};

#[proc_macro_derive(UniformInterface, attributes(ts_type))]
pub fn derive_uniform_interface(input: TokenStream) -> TokenStream {    
  let di: DeriveInput = parse_macro_input!(input);

  match di.data {
    // for now, we only handle structs
    Data::Struct(struct_) => {
        match struct_.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    println!("{:?}", field.ident.as_ref().unwrap());
                    match &field.ty {
                        Type::Path(type_path) => {
                            println!("{:?}", type_path.path.segments.first().unwrap().ident.to_string());
                        }
                        _ => {}
                    }
                } 
            }
            _ => {}
        }
        "".parse().unwrap()
    }

    _ => panic!("only structs are currently supported for deriving UniformInterface"),
  }
}