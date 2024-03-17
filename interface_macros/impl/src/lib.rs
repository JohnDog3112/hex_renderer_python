
use proc_macro2::Ident;
use syn::{parse_macro_input, Result, Error, parse::Parse, Token, Type};


type ProcStream = proc_macro::TokenStream;


#[derive(Debug)]
struct Arguments {
    bridge: Option<Ident>
}

impl Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self { bridge: None });
        }
        let ident: Ident = input.parse()?;
        if ident.to_string() != "bridge" {
            return Err(Error::new(ident.span(), "expected bridge"));
        }

        input.parse::<Token![=]>()?;
        
        let bridge: Ident = input.parse()?;

        Ok(Self {
            bridge: Some(bridge)
        })
    }
}

mod py_gen;

#[proc_macro_attribute]
pub fn py_gen(args: ProcStream, input: ProcStream) -> ProcStream {
    
    let item = parse_macro_input!(input as syn::Item);

    let args = parse_macro_input!(args as Arguments);


    py_gen::py_gen_impl(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}


#[derive(Debug)]
struct TypeArgs {
    nested: Option<Type>,
}

impl Parse for TypeArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self {nested: None})
        }
        let ident: Ident = input.parse()?;
        if ident.to_string() != "nested" {
            return Err(Error::new(ident.span(), "expected nested"));
        }

        input.parse::<Token![=]>()?;
        
        let nested: Type = input.parse()?;
        Ok(Self {
            nested: Some(nested)
        })
    }
}

mod py_type_gen;
#[proc_macro_attribute]
pub fn py_type_gen(args: ProcStream, input: ProcStream) -> ProcStream {

    let item = parse_macro_input!(input as syn::Item);

    let args = parse_macro_input!(args as TypeArgs);

    py_type_gen::py_type_gen_impl(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}