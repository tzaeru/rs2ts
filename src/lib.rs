extern crate proc_macro;
extern crate syn;

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
        "u32" | "f32" => TSType::Number,
        "String" => TSType::String,
        _ => TSType::Custom(ts_type.to_string())
    }
}

#[proc_macro_derive(ParseToTS, attributes(ts_type))]
pub fn parse_to_ts(input: TokenStream) -> TokenStream {    

    // Build a syntax tree from the input TokenStream
    let di: DeriveInput = parse_macro_input!(input);
    println!("{:?}", di.ident.to_string());
    match di.data {
        // for now, we only handle structs
        Data::Struct(struct_) => {
            let mut ts_interface = TSInterface::new(di.ident.to_string());
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
                            _ => {}
                        }
                    } 
                }
                _ => {}
            }
            ts_interface.fields = ts_fields;
            println!("{}", ts_interface.to_string());
            "".parse().unwrap()
        }

        _ => panic!("Oly structs are currently supported for deriving ParseToTS"),
    }
}
