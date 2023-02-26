extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};
use toml::Value;
use std::sync::{atomic, RwLock};
use atomic_float::{AtomicF32, AtomicF64};

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(expression_parser);


#[proc_macro_attribute]
pub fn add_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    //let name = &ast.ident;

    //Import file that defines the required struct
    let toml = include_str!("CentralDataStruct.toml");
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
            match var_type.as_str() {
                "u64" | "u32" | "u16" | "i64" | "i32" | "i16" | "f64" | "f32" | "bool" => {
                    struct_def.push_str(&format!("{}: Value<Atomic{}>,\n", key, var_type.to_uppercase()));
                    return (Some(struct_def), None);
                }

                "String" | "string" => {
                    struct_def.push_str(&format!("{}: Value<std::sync::RwLock<String>>,\n", key));
                    return (Some(struct_def), None);
                }
                _ => panic!("Variable type unknown"),
            }
        }
        Value::Table(table) => {
            let mut Type: Option<String> = None;
            let mut Size: Option<String> = None;
            for (k, v) in table.iter() {
                if k == "type" {
                    if let Value::String(t) = v {
                        Type = Some(String::from(t));
                    } else {
                        panic!("The value of a 'type' field in the struct config must be a String");
                    }
                }
                if k == "size" {
                    if let Value::Integer(s) = v {
                        Size = Some(s.to_string());
                    } else {
                        panic!("The value of a 'size' field in the struct config must be an Integer");
                    }
                }
            }

            if Type != None && Size != None {
                let t = Type.unwrap();
                let s = Size.unwrap();
                if s != "0" {
                    let res = String::from(format!("{}: Value<std::sync::RwLock<[{}; {}]>>,\n", key, t.as_str(), s.as_str()));
                    println!("Member with history: {}", res);
                    return (Some(res), None);
                } else {
                    let res = String::from(format!("{}: Value<Atomic{}>,\n", key, t.as_str().to_uppercase()));
                    return (Some(res), None);
                }
            } else if Type != None || Size != None {
                panic!("For values with a history, both the type and the size of the history need to be specified");
            } else {
                rest.push_str(&format!("struct {} {{\n", key));

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
                            rest = format!("{}{}", defs, rest);
                        }
                        None => {}
                    }
                }
                rest.push_str("}\n\n");
                struct_def = format!("m_{}: {},\n", key, key);
                return (Some(struct_def), Some(rest));
            }
        }
        _ => (None, None),
    }
}




#[proc_macro_derive(Callbacks)]
pub fn derive_callbacks(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let toml = include_str!("Callbacks.toml");
    let toml = toml.parse::<Value>().unwrap();

    let name = ast.ident.to_string();

    let mut callback_code = String::new();

    match toml {
        Value::Table(table) => {
            for (_callback_name, v) in table.iter() {
                if let Value::Table(callback_def) = v {
                    let mut owning_variable = String::new();
                    let mut condition = String::new();
                    let mut callback = String::new();
                    for (key, value) in callback_def {
                        match key.as_str() {
                            "var" => {
                                if let Value::String(var) = value {
                                    owning_variable.push_str(
                                        build_var_path(var).as_str()
                                    );
                                }
                            }
                            "condition" => {
                                if let Value::String(cond) = value {
                                    let mut res = expression_parser::AssignmentParser::new().parse(cond.as_str()).expect("Couldn't parse condition");
                                    res.pop();
                                    condition.push_str(res.as_str());
                                }
                            }
                            "callback" => {
                                if let Value::String(cb) = value {
                                    let res = expression_parser::AssignmentParser::new().parse(cb.as_str()).expect(&format!("Couldn't parse callback: {}", cb));
                                    callback.push_str(res.as_str());
                                }
                            }
                            _ => panic!("Unknown entry in callback definition"),
                        }
                    }
                    if (owning_variable.len() > 0) && (condition.len() > 0) && (callback.len() > 0) {
                        callback_code.push_str(&format!("{}.register_callback(Box::new(||{{{}}}), Condition{{ eval: Box::new(||{{{}}})}});\n",
                                                        owning_variable, callback, condition));
                        // println!("Callback Code: {}", callback_code);
                    } else {
                        panic!("Callback is missing either an owning variable, a condition or a callback function");
                    }
                } else {
                    panic!("Syntax error in Callback.toml");
                }
            }
        }
        _ => panic!("Callbacks.toml parsed incorrectly"),
    }

    let struct_name: proc_macro2::TokenStream = name.parse().unwrap();
    let cb_tokens: proc_macro2::TokenStream = callback_code.parse().unwrap();


    return quote!{
        impl #struct_name {
            pub fn init(&'static self) {
                #cb_tokens
            }
        }
    }.into();
}


fn build_var_path(s: &String) -> String {
    let mut path = String::new();
    if s.as_str().contains("Process") {
        path.push_str("self.Process.");
    } else if s.as_str().contains("Control") {
        path.push_str("self.Control.");
    } else {
        panic!("Trying to access non-Process and non-Control variable")
    }
    let mut members = s.split(".").peekable();
    members.next();

    while let Some(member) = members.next() {
        if !(members.peek().is_none()) {
            path.push_str("m_");
            path.push_str(member);
            path.push('.');
        } else {
            path.push_str(member);
        }
    }
    path
}

