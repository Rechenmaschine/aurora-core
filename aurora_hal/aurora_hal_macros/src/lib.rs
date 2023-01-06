extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};
use toml::Value;


#[proc_macro_attribute]
pub fn add_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    //let name = &ast.ident;

    //Import file that defines the required struct
    let toml = include_str!("struct.toml");
    let toml = toml.parse::<Value>().unwrap();

    //Add the fields for Control and Process Variables to the Struct
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => match &mut struct_data.fields {
            syn::Fields::Named(fields) => {
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { process: ProcessVar })
                        .unwrap(),
                );
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { control: ControlVar })
                        .unwrap(),
                );
            }
            _ => (),
        },
        _ => {
            panic!("Add field has to be called with structs");
        }
    }

    let (_process_rest, process_def_opt) = build_struct("ProcessVar", &toml["Process"]);
    let (_control_rest, control_def_opt) = build_struct("ControlVar", &toml["Control"]);

    let process_def: proc_macro2::TokenStream = match process_def_opt {
        Some(x) => x,
        None => String::new(),
    }
    .parse()
    .unwrap();

    let control_def: proc_macro2::TokenStream = match control_def_opt {
        Some(x) => x,
        None => String::new(),
    }
    .parse()
    .unwrap();

    quote! {
        #process_def
        #control_def
        #ast
    }
    .into()
}

fn build_struct(key: &str, value: &Value) -> (Option<String>, Option<String>) {
    let mut struct_def = String::new();
    let mut rest = String::new();

    match value {
        Value::String(var_type) => {
            struct_def.push_str(&format!("{key}: {var_type},\n"));
            (Some(struct_def), None)
        }
        Value::Table(table) => {
            rest.push_str(&format!("struct {key} {{\n"));

            for (k, v) in table.iter() {
                let (struct_res, sub_structs) = build_struct(k, v);
                match struct_res {
                    Some(val) => {
                        rest.push_str(&val);
                    }
                    None => {}
                }
                match sub_structs {
                    Some(defs) => {
                        rest = format!("{defs}{rest}");
                    }
                    None => {}
                }
            }
            rest.push_str("}\n\n");
            struct_def = format!("m_{key}: {key},\n");
            (Some(struct_def), Some(rest))
        }
        _ => (None, None),
    }
}
