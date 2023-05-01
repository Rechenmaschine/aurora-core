#![allow(unused_imports)]
#![allow(non_snake_case)]

extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, parse::Parser, parse::ParseStream, parse::Parse, Result, };
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
    let toml = include_str!("IoTree.toml");
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
                    struct_def.push_str(&format!("{}: Atomic{},\n", key, var_type.to_uppercase()));
                    return (Some(struct_def), None);
                }

                "str" => {
                    struct_def.push_str(&format!("{}: std::sync::RwLock<String>,\n", key));
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
                    let res = String::from(format!("{}: std::sync::RwLock<RingBuffer<{}, {}>>,\n", key, t.as_str(), s.as_str()));
                    return (Some(res), None);
                } else {
                    let res = String::from(format!("{}: Atomic{},\n", key, t.as_str().to_uppercase()));
                    return (Some(res), None);
                }
            } else if Type != None || Size != None {
                panic!("For values with a history, both the type and the size of the history need to be specified");
            } else {
                rest.push_str(&format!("#[derive(Init)]\nstruct {} {{\n", key));

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




#[proc_macro]
pub fn derive_callbacks(_input: TokenStream) -> TokenStream {

    let toml = include_str!("Callbacks.toml");
    let toml = toml.parse::<Value>().unwrap();

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
                        callback_code.push_str(&format!("let mut v: Vec<(Box<dyn Fn() -> bool + Send + Sync>, Box<dyn Fn() + Send + Sync>)> = Vec::new();\n\
                                                                v.push((Box::new(||{{{}}}), Box::new(||{{{}}})));\n\
                                                                CALLBACKS.lock().unwrap().insert(\"{}\".to_string(), v);\n",
                                                        condition, callback, owning_variable));
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



    let cb_tokens: proc_macro2::TokenStream = callback_code.parse().unwrap();


    return quote!{
        pub fn init_callbacks() {
            #cb_tokens
        }
    }.into();
}


fn build_var_path(s: &String) -> String {
    let mut path = String::new();
    if s.as_str().contains("process") {
        path.push_str("CALLBACKS.lock().unwrap().process.");
    } else if s.as_str().contains("control") {
        path.push_str("CALLBACKS.lock().unwrap().control.");
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


#[proc_macro_derive(Init)]
pub fn derive_init_fn(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident.to_string();

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_iter = fields.iter();
    let mut field_initialization = String::new();

    for field in field_iter {
        let fname = field.ident.clone().unwrap().to_string();
        let ftype = match &field.ty {
            syn::Type::Path(t) => {
                let seg_it = t.path.segments.iter();
                let mut path = String::new();
                for seg in seg_it {
                    path.push_str(seg.ident.to_string().as_str());
                    if seg.ident.to_string() == "RwLock".to_string() {
                        path.push_str("::new(");
                        match &seg.arguments {
                            syn::PathArguments::AngleBracketed(brackets) => {
                                for arg in brackets.args.iter() {
                                    match arg {
                                        syn::GenericArgument::Type(syn::Type::Path(rwlock_args)) => {
                                            for type_in_rwlock in rwlock_args.path.segments.iter() {
                                                path.push_str(type_in_rwlock.ident.to_string().as_str());

                                                if type_in_rwlock.ident.to_string() == "RingBuffer" {
                                                    path.push_str("::new(");
                                                    match &type_in_rwlock.arguments {
                                                        syn::PathArguments::AngleBracketed(ringbuf_configs) => {
                                                            let mut buffer_length: u32 = 0;
                                                            let mut buffer_type = String::new();
                                                            for ringbuf_config in ringbuf_configs.args.iter() {
                                                                match ringbuf_config {
                                                                    // Find type contained in RingBuffer
                                                                    syn::GenericArgument::Type(syn::Type::Path(ringbuffer_args)) => {
                                                                        for ringbuffer_type in ringbuffer_args.path.segments.iter() {
                                                                            buffer_type.push_str(ringbuffer_type.ident.to_string().as_str());
                                                                        }
                                                                    }
                                                                    // Find length of RingBuffer
                                                                    syn::GenericArgument::Const(syn::Expr::Lit(syn::ExprLit{ attrs: _, lit})) => {
                                                                        match lit {
                                                                            syn::Lit::Int(buffer_length_syn) => {
                                                                                match buffer_length_syn.base10_parse::<u32>() {
                                                                                    Ok(l) => buffer_length = l,
                                                                                    _ => panic!("RingBuffer length parsing failed"),
                                                                                }
                                                                            },
                                                                            _ => panic!("RingBuffer length is not an integer value"),
                                                                        }

                                                                    }
                                                                    _ => panic!("The arguments in AngleBracketed of the RingBuffer contain something aside from Type and Size")
                                                                }
                                                            }
                                                            // Build an array to initialize the RingBuffer with, e.g.: [0, 0, 0, ...], values in the array depend
                                                            // on the type in the RingBuffer
                                                            path.push('[');
                                                            for _i in 0..buffer_length {
                                                                path.push_str(format!("{}, ", initialize_type(&buffer_type)).as_str());
                                                            }
                                                            path.push_str("]");
                                                        }
                                                        _ => panic!("The arguments in the RingBuffer struct contain something aside from syn::PathArguments::AngleBracketed")
                                                    }
                                                    path.push(')');
                                                } else {
                                                    // The value in the RwLock is a String or something else with a simple new() function
                                                    path.push_str("::new()");
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                        // Close the RwLock::new() function
                        path.push(')');
                    }
                    path.push_str("::");
                }
                // Remove trailing colons (Needed to be able to build a path
                path.pop();
                path.pop();
                path
            }
            _ => panic!("Invalid type in the struct, can't create init function.")
        };

        field_initialization.push_str(add_new_function(fname, ftype).as_str());

    }

    let name_token: proc_macro2::TokenStream = name.parse().unwrap();
    let field_init_token: proc_macro2::TokenStream = field_initialization.parse().unwrap();

    return quote! {
        impl #name_token {
            pub fn new() -> #name_token {
                #name_token {
                    #field_init_token
                }
            }
        }
    }.into();
}


fn add_new_function(field_name: String, field_type: String) -> String {
    match field_type.as_str() {
        "AtomicU64" | "AtomicU32" | "AtomicU16" | "AtomicI64" | "AtomicI32" | "AtomicI16" => format!("{}: {}::new(0),\n", field_name, field_type),
        "AtomicF64" | "AtomicF32" => format!("{}: {}::new(0.0),\n", field_name, field_type),
        "AtomicBool" => format!("{}: {}::new(false),\n", field_name, field_type),
        _ => {
            if field_type.contains("RwLock") || field_type.contains("RingBuffer") {
                format!("{}: {},\n", field_name, field_type)
            } else {
                format!("{}: {}::new(),\n", field_name, field_type)
            }
        }
    }
}

fn initialize_type (field_type: &String) -> String {
    match field_type.as_str() {
        "AtomicU64" | "AtomicU32" | "AtomicU16" | "AtomicI64" | "AtomicI32" | "AtomicI16" => { format!("{}::new(0)", field_type) }
        "AtomicF64" | "AtomicF32" => { format!("{}::new(0.0)", field_type) }
        "AtomicBool" => { format!("{}::new(false)", field_type) }
        "u16" | "u32" | "u64" | "i16" | "i32" | "i64" => { format!("0") }
        "f16" | "f32" | "f64" => { format!("0.0") }
        _ => {
            if field_type.contains("RwLock") || field_type.contains("RingBuffer") {
                format!("{}", field_type)
            } else {
                format!("{}::new()", field_type)
            }
        }
    }
}