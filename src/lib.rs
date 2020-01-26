extern crate proc_macro;
extern crate syn;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, DataStruct, Fields, Ident, TypePath, Type};

enum TSType {
    String,
    Number,
    Boolean,
    Custom(String),
}

impl TSType {
    pub fn to_string(&self) -> String {
        match self {
            TSType::String => "string".to_string(),
            TSType::Number => "number".to_string(),
            TSType::Boolean => "boolean".to_string(),
            TSType::Custom(ts_type) => ts_type.to_string(),
        }
    }
}

struct TSField {
    pub name: String,
    pub ts_type: TSType,
}

struct TSInterface {
    pub name: String,
    pub fields: Vec<TSField>,
}

impl TSInterface {
    pub fn new(name: String) -> TSInterface {
        TSInterface {name: name, fields: Vec::new()}
    }
    
    pub fn to_string(&self) -> String {
        let interface_block_open = format!("interface {} {{\n", self.name);
        let interface_block_close = "}";
        let fields_def = self.fields.iter().fold("".to_owned(), |acc, field| format!("{}  {}: {};\n", acc, field.name, field.ts_type.to_string()));
        format!("{}{}{}", interface_block_open, &fields_def, interface_block_close)
    }
}

fn rust_type_to_ts_type(ts_type: &str) -> TSType
{
    match ts_type {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize" | "usize" | "f32" | "f64" => TSType::Number,
        "String" => TSType::String,
        "bool" => TSType::Boolean,
        _ => TSType::Custom(ts_type.to_string())
    }
}

pub fn parse_to_ts(syntax: DeriveInput)
{
    //println!("{:?}", syntax.data);
    match syntax.data {
        // for now, we only handle structs
        Data::Struct(struct_) => {
            let struct_name = syntax.ident.to_string();
            let mut ts_interface = TSInterface::new(struct_name.clone());
            let mut ts_fields: Vec<TSField> = Vec::new();
            match struct_.fields {
                Fields::Named(ref fields) => {
                    for field in fields.named.iter() {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        println!("{:?}", field_name);
                        match &field.ty {
                            Type::Path(type_path) => {
                                let field_type = type_path.path.segments.first().unwrap().ident.to_string();
                                println!("{:?}", field_type);
                                let ts_type = TSField {name: field_name, ts_type: rust_type_to_ts_type(&field_type)};
                                ts_fields.push(ts_type);
                            }
                            _ => {println!("{:?}", &field.ty);}
                        }
                    } 
                }
                _ => {}
            }
            ts_interface.fields = ts_fields;

            fs::create_dir_all("target/ts");

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(format!("target/ts/{}.ts", struct_name)).unwrap();

            file.write_all(ts_interface.to_string().as_bytes());
            println!("{}", ts_interface.to_string());
        }

        _ => panic!("Only structs are currently supported for deriving ParseToTS"),
    }
}
