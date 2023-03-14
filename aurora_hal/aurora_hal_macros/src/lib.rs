extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};
use toml::Value;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(expression_parser);

/// # Panics
///
/// Will panic if IoTree.toml is unparseable
#[proc_macro_attribute]
pub fn add_fields(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    //let name = &ast.ident;

    //Import file that defines the required struct
    let toml = include_str!("IoTree.toml");
    let toml = toml.parse::<Value>().expect("Couldn't parse IoTree.toml");

    //Add the fields for Control and Process Variables to the Struct
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            if let syn::Fields::Named(fields) = &mut struct_data.fields {
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
        Value::String(var_type) => match var_type.as_str() {
            "u64" | "u32" | "u16" | "i64" | "i32" | "i16" | "f64" | "f32" | "bool" => {
                struct_def.push_str(&format!("{key}: Atomic{},\n", var_type.to_uppercase()));
                (Some(struct_def), None)
            }

            "str" => {
                struct_def.push_str(&format!("{key}: std::sync::RwLock<String>,\n"));
                (Some(struct_def), None)
            }
            _ => panic!("Variable type unknown"),
        },
        Value::Table(table) => {
            let mut type_name: Option<String> = None;
            let mut ringbuffer_size: Option<String> = None;
            for (k, v) in table.iter() {
                if k == "type" {
                    if let Value::String(t) = v {
                        type_name = Some(String::from(t));
                    } else {
                        panic!("The value of a 'type' field in the struct config must be a String");
                    }
                }
                if k == "size" {
                    if let Value::Integer(s) = v {
                        ringbuffer_size = Some(s.to_string());
                    } else {
                        panic!(
                            "The value of a 'size' field in the struct config must be an Integer"
                        );
                    }
                }
            }

            match (type_name, ringbuffer_size) {
                (Some(t), Some(s)) => {
                    if s == "0" {
                        let res = format!("{key}: Atomic{},\n", t.as_str().to_uppercase());
                        (Some(res), None)
                    } else {
                        let res = format!(
                            "{key}: std::sync::RwLock<RingBuffer<{}, {}>>,\n",
                            t.as_str(),
                            s.as_str()
                        );
                        (Some(res), None)
                    }
                }
                (Some(_), None) => {
                    panic!("Size of history not specified");
                }
                (None, Some(_)) => {
                    panic!("Type of history not specified");
                }
                _ => {
                    rest.push_str(&format!("#[derive(Init)]\nstruct {key} {{\n"));

                    for (k, v) in table.iter() {
                        let (struct_res, sub_structs) = build_struct(k, v);
                        if let Some(val) = struct_res {
                            rest.push_str(&val);
                        }
                        if let Some(defs) = sub_structs {
                            rest = format!("{defs}{rest}");
                        }
                    }
                    rest.push_str("}\n\n");
                    struct_def = format!("m_{key}: {key},\n");
                    (Some(struct_def), Some(rest))
                }
            }
        }
        _ => (None, None),
    }
}

/// # Panics
///
/// Will panic if Callbacks.toml is unparseable
#[proc_macro]
pub fn derive_callbacks(_input: TokenStream) -> TokenStream {
    let toml = include_str!("Callbacks.toml");
    let toml = toml
        .parse::<Value>()
        .expect("Couldn't parse Callbacks.toml");

    let mut callback_code = String::new();

    match toml {
        Value::Table(table) => {
            for v in table.values() {
                if let Value::Table(callback_def) = v {
                    let mut owning_variable = String::new();
                    let mut condition = String::new();
                    let mut callback = String::new();
                    for (key, value) in callback_def {
                        match key.as_str() {
                            "var" => {
                                if let Value::String(var) = value {
                                    owning_variable.push_str(build_var_path(var).as_str());
                                }
                            }
                            "condition" => {
                                if let Value::String(cond) = value {
                                    let mut res = expression_parser::AssignmentParser::new()
                                        .parse(cond.as_str())
                                        .expect("Couldn't parse condition");
                                    res.pop();
                                    condition.push_str(res.as_str());
                                }
                            }
                            "callback" => {
                                if let Value::String(cb) = value {
                                    let res = expression_parser::AssignmentParser::new()
                                        .parse(cb.as_str())
                                        .unwrap_or_else(|_| {
                                            panic!("Couldn't parse callback: {cb}")
                                        });
                                    callback.push_str(res.as_str());
                                }
                            }
                            _ => panic!("Unknown entry in callback definition"),
                        }
                    }
                    if (!owning_variable.is_empty())
                        && (!condition.is_empty())
                        && (!callback.is_empty())
                    {
                        callback_code.push_str(&format!("let mut v: Vec<(Box<dyn Fn() -> bool + Send + Sync>, Box<dyn Fn() + Send + Sync>)> = Vec::new();\n\
                                                                v.push((Box::new(||{{{condition}}}), Box::new(||{{{callback}}})));\n\
                                                                CALLBACKS.lock().unwrap().insert(\"{owning_variable}\".to_string(), v);\n"));
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

    quote! {
        pub fn init_callbacks() {
            #cb_tokens
        }
    }
    .into()
}

fn build_var_path(s: &str) -> String {
    let mut path = String::new();
    if s.contains("process") {
        path.push_str("CALLBACKS.lock().unwrap().process.");
    } else if s.contains("control") {
        path.push_str("CALLBACKS.lock().unwrap().control.");
    } else {
        panic!("Trying to access non-Process and non-Control variable")
    }
    let mut members = s.split('.').peekable();
    members.next();

    while let Some(member) = members.next() {
        if members.peek().is_some() {
            path.push_str("m_");
            path.push_str(member);
            path.push('.');
        } else {
            path.push_str(member);
        }
    }
    path
}

/// # Panics
///
/// Will panic if the `IoTree` is empty or contains only unnamed fields.
/// Also panics if the `IoTree` is built in an invalid way.
#[proc_macro_derive(Init)]
pub fn derive_init_fn(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident.to_string();

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
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
                    if seg.ident == *"RwLock" {
                        path.push_str("::new(");
                        path.push_str(get_type_in_rwlock(seg).as_str());
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
            _ => panic!("Invalid type in the struct, can't create init function."),
        };

        field_initialization.push_str(add_new_function(fname.as_str(), ftype.as_str()).as_str());
    }

    let name_token: proc_macro2::TokenStream = name.parse().unwrap();
    let field_init_token: proc_macro2::TokenStream = field_initialization.parse().unwrap();

    quote! {
        impl #name_token {
            pub fn new() -> #name_token {
                #name_token {
                    #field_init_token
                }
            }
        }
    }
    .into()
}

fn get_type_in_rwlock(rwlock: &syn::PathSegment) -> String {
    let mut inner_type_initialization = String::new();
    if let syn::PathArguments::AngleBracketed(brackets) = &rwlock.arguments {
        for arg in brackets.args.iter() {
            if let syn::GenericArgument::Type(syn::Type::Path(rwlock_args)) = arg {
                for type_in_rwlock in rwlock_args.path.segments.iter() {
                    inner_type_initialization.push_str(type_in_rwlock.ident.to_string().as_str());

                    if type_in_rwlock.ident == "RingBuffer" {
                        inner_type_initialization.push_str("::new(");
                        inner_type_initialization
                            .push_str(build_buffer_to_initialize(type_in_rwlock).as_str());
                        inner_type_initialization.push(')');
                    } else {
                        // The value in the RwLock is a String or something else with a simple new() function
                        inner_type_initialization.push_str("::new()");
                    }
                }
            }
        }
    }
    inner_type_initialization
}

fn build_buffer_to_initialize(ringbuffer: &syn::PathSegment) -> String {
    let mut buffer_to_initialize = String::new();
    match &ringbuffer.arguments {
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
            buffer_to_initialize.push('[');
            for _i in 0..buffer_length {
                buffer_to_initialize.push_str(format!("{}, ", initialize_type(&buffer_type)).as_str());
            }
            buffer_to_initialize.push(']');
        }
        _ => panic!("The arguments in the RingBuffer struct contain something aside from syn::PathArguments::AngleBracketed")
    }
    buffer_to_initialize
}

fn add_new_function(field_name: &str, field_type: &str) -> String {
    match field_type {
        "AtomicU64" | "AtomicU32" | "AtomicU16" | "AtomicI64" | "AtomicI32" | "AtomicI16" => {
            format!("{field_name}: {field_type}::new(0),\n")
        }
        "AtomicF64" | "AtomicF32" => format!("{field_name}: {field_type}::new(0.0),\n"),
        "AtomicBool" => format!("{field_name}: {field_type}::new(false),\n"),
        _ => {
            if field_type.contains("RwLock") || field_type.contains("RingBuffer") {
                format!("{field_name}: {field_type},\n")
            } else {
                format!("{field_name}: {field_type}::new(),\n")
            }
        }
    }
}

fn initialize_type(field_type: &String) -> String {
    match field_type.as_str() {
        "AtomicU64" | "AtomicU32" | "AtomicU16" | "AtomicI64" | "AtomicI32" | "AtomicI16" => {
            format!("{field_type}::new(0)")
        }
        "AtomicF64" | "AtomicF32" => {
            format!("{field_type}::new(0.0)")
        }
        "AtomicBool" => {
            format!("{field_type}")
        }
        "u16" | "u32" | "u64" | "i16" | "i32" | "i64" => "0".to_string(),
        "f16" | "f32" | "f64" => "0.0".to_string(),
        _ => {
            if field_type.contains("RwLock") || field_type.contains("RingBuffer") {
                format!("{field_type}")
            } else {
                format!("{field_type}::new()")
            }
        }
    }
}
