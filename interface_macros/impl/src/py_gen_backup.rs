use proc_macro2::{Span, TokenStream, Literal};
use syn::{Item, Error, Result, Ident, Type, parse::Parse, Token, LitStr, LitBool, Attribute, parse2, ItemStruct, ItemEnum, Expr, Lit};
use quote::{quote, ToTokens, TokenStreamExt};

use crate::Arguments;

pub fn py_gen_impl(args: Arguments, input: Item) -> Result<TokenStream> {

    match input {
        Item::Enum(enu) => py_gen_enum(args, enu),
        Item::Struct(struc) => py_gen_struct(args, struc),
        _ => Err(Error::new(Span::call_site(), "expected enum or struct")),
    }
}

struct FieldArgs {
    name: Option<Ident>,
    remote: Option<Type>,
    parent: bool,
}
impl FieldArgs {
    fn new() -> Self {
        Self {
            name: None,
            remote: None,
            parent: false,
        }
    }
}
impl Parse for FieldArgs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut fields = Self::new();

        while !input.is_empty() {
            let next: Ident = input.parse()?;

            match &next.to_string()[..] {
                "name" => {
                    input.parse::<Token![=]>()?;
                    let name: LitStr = input.parse()?;

                    if fields.name.is_some() {
                        return Err(Error::new(next.span(), "can't set name more than once"));
                    }
                    fields.name = Some(Ident::new(&name.value(), name.span()));
                },
                "remote" => {
                    input.parse::<Token![=]>()?;
                    let remote: Type = input.parse()?;

                    if fields.remote.is_some() {
                        return Err(Error::new(next.span(), "can't set remote more than once"));
                    }
                    fields.remote = Some(remote);
                },
                "parent" => {
                    input.parse::<Token![=]>()?;
                    let parent: LitBool = input.parse()?;

                    fields.parent = parent.value();
                }
                _ => return Err(Error::new(next.span(), "unknown attribute"))
            };

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(fields)
    }
}

impl FromIterator<FieldArgs> for Result<FieldArgs>   {
    fn from_iter<T: IntoIterator<Item = FieldArgs>>(iter: T) -> Result<FieldArgs> {
        iter.into_iter().fold(Ok(FieldArgs::new()), |acc, e| {
            let mut acc = acc?;

            if acc.name.is_none() && e.name.is_some() {
                acc.name = e.name;
            } else if acc.name.is_some() && e.name.is_some() {
                return Err(Error::new(e.name.unwrap().span(), "can't set name for than once"))
            }

            if acc.remote.is_none() && e.remote.is_some() {
                acc.remote = e.remote;
            } else if acc.remote.is_some() && e.remote.is_some() {
                return Err(Error::new_spanned(e.remote.unwrap().clone(), "can't set remote more than once"))
            }

            acc.parent = e.parent;

            Ok(acc)
        })
    }
}

fn parse_field_attrs(inp: &Vec<Attribute>) -> Result<FieldArgs> {
    inp.iter().filter_map(|attr| {
        match &attr.meta {
            syn::Meta::List(list) => {
                Some(parse2::<FieldArgs>(list.tokens.clone()))
            },
            _ => None,
        }
    }).collect::<Result<Vec<FieldArgs>>>()?
    .into_iter()
    .collect()
}

fn gen_construction_args(named: bool, fields: &Vec<(FieldArgs, &Type)>) -> TokenStream {

    let creation_args: TokenStream = fields.iter().map(|(args, ty)| {
        let name = args.name.clone().unwrap();

        match &args.remote {
            Some(remote) => {
                let into = quote!(
                    <#ty as ::interface_macros::PyBridge<#remote>>::from_py(#name, py)?
                );
                if named {
                    quote!(#name: #into,)
                } else {
                    quote!(#into,)
                }
            },
            None => quote!(#name,),
        }
    }).collect();
    
    if named {
        quote!{
            {
                #creation_args
            }
        }
    } else {
        quote!{
            (
                #creation_args
            )
        }
    }

}
fn get_doc_comments(attrs: &Vec<Attribute>) -> TokenStream {
    attrs.iter().filter(|attr| {
        match &attr.meta {
            syn::Meta::NameValue(name_val) => {
                //eprintln!("name_val: {:?}", name_val.path.get_ident());

                let Some(ident) = name_val.path.get_ident() else {
                    return false;
                };
                if ident.to_string() != "doc" {
                    return false;
                };
                let Expr::Lit(expr) = &name_val.value else {
                    return false;
                };

                let Lit::Str(str) = &expr.lit else {
                    return false;
                };

                eprintln!("{:?}", attr.meta);
                return true;
               // name_val.\
            },
            _ => return false,
        }
    }).map(|a| {
            quote!(#a)
        }).fold(TokenStream::new(), |mut acc, a| {
            acc.append_all(a);
            acc
        })
}

fn py_gen_struct(struct_args: Arguments, input: ItemStruct) -> Result<TokenStream> {

    let fields = input.fields.iter().map(|field| -> Result<(FieldArgs, &Type)> {
        let mut args = parse_field_attrs(&field.attrs)?;
        let ty = &field.ty;

        args.name = if let Some(name) = args.name {
            Some(name)
        } else if let Some(name) = field.ident.clone() {
            Some(name)
        } else {
            return Err(Error::new_spanned(field.clone(), "no name specified, expected `#[py_gen(name = \"<name\"]`"))
        };

        Ok((args, ty))
    }).collect::<Result<Vec<_>>>()?;

    let field_comments = input.fields.iter().map(|field| {
        get_doc_comments(&field.attrs)
    }).collect::<Vec<TokenStream>>();

    
    let new_args: TokenStream = fields.iter().map(|(args, ty)| {
        let name = args.name.clone().unwrap();
        
        match &args.remote {
            Some(val) => quote!(#name: #val,),
            None => quote!(#name: #ty,),
        }
    }).collect();

    let named = match &input.fields {
        syn::Fields::Named(_) => true,
        syn::Fields::Unnamed(_) => false,
        syn::Fields::Unit => return Err(Error::new_spanned(input.clone(), "unit not supported")),
    };

    let constructor = gen_construction_args(named, &fields);

    let getters: TokenStream = fields.iter().enumerate().zip(field_comments).map(|((i,(args, ty)), comments)| {
        let ident = args.name.clone().unwrap();
        let get_ident = Ident::new(&format!("get_{ident}"), ident.span());
        
        let item = if named {
            quote!(self.0.#ident.clone())
        } else {
            let i = Literal::usize_unsuffixed(i);
            quote!(self.0.#i.clone())
        };

        eprintln!("comments: \n{}", comments);

        match &args.remote {
            Some(remote) => {
                quote!{
                    #[getter]
                    #comments
                    fn #get_ident(&self, py: ::pyo3::Python) -> ::pyo3::PyResult<<#ty as ::interface_macros::PyBridge<#remote>>::PyOut> {
                        <#ty as ::interface_macros::PyBridge<#remote>>::into_py(#item, py)
                    }
                }
            },
            None => {
                quote!{
                    #[getter]
                    #comments
                    fn #get_ident(&self) -> #ty {
                        #item
                    }
                }
            }
        }
    }).collect();

    let withs: TokenStream = fields.iter().enumerate().map(|(i, (args, ty))| {

        let ident = args.name.clone().unwrap();

        let with_ident = Ident::new(&format!("with_{ident}"), ident.span());

        let set_index = if named {
            quote!(val.#ident)
        } else {
            let i = Literal::usize_unsuffixed(i);
            quote!(val.#i)
        };


        let (val, ty) = match &args.remote {
            Some(remote) => (
                quote!{
                    //#ident.into()
                    <#ty as ::interface_macros::PyBridge<#remote>>::from_py(#ident, py)?
                }, 
                remote
            ),
            None => (quote!(#ident), *ty),
        };

        quote! {
            fn #with_ident(&self, #ident: #ty, py: ::pyo3::Python) -> ::pyo3::PyResult<Self> {
                let mut val = self.0.clone();
                #set_index = #val;

                Ok(Self(val))
            }
        }
    }).collect();

    
    
    
    let remote = struct_args.remote;

    let vis = &input.vis;
    let ident = &input.ident;

    let remote_str = LitStr::new(&remote.to_string(), remote.span());

    let attrs: TokenStream = input.attrs.iter().map(|attr| attr.into_token_stream()).collect();


    Ok(quote!{
        #[::interface_macros::py_type_gen]
        #[::pyo3::pyclass(name=#remote_str)]
        #attrs
        #vis struct #ident(#vis #remote);

        #[::interface_macros::py_type_gen]
        #[::pyo3::pymethods]
        impl #ident {
            #[new]
            #vis fn new(py: ::pyo3::Python, #new_args) -> ::pyo3::PyResult<Self> {
                Ok(Self(
                    #remote #constructor
                ))
                
            }

            #getters

            #withs

            fn __repr__(&self) -> String {
                ::std::format!("{:?}", self.0)
            }
        }

        impl #ident {
            pub fn add_class(py: ::pyo3::Python, m: &::pyo3::types::PyModule) -> ::pyo3::PyResult<()> {
                m.add_class::<#ident>()?;

                Ok(())
            }
        }

        impl ::interface_macros::PyBridge<#ident> for #remote {
            type PyOut = #ident;

            fn into_py(self, _py: ::pyo3::Python) -> ::pyo3::PyResult<Self::PyOut> {
                Ok(#ident(self))
            }
            fn from_py(val: #ident, _py: ::pyo3::Python) -> ::pyo3::PyResult<Self> {
                Ok(val.0)
            }
        }
    })
}



fn py_gen_enum(args: Arguments, input: ItemEnum) -> Result<TokenStream> {

    let vis = &input.vis;
    let ident = &input.ident;
    let remote = &args.remote;

    let attrs: TokenStream = input.attrs.iter().map(|attr| attr.into_token_stream()).collect();


    let variant_parts: (Vec<TokenStream>, Vec<TokenStream>) = input.variants.iter().map(|variant| {

        let raw_variant_ident = &variant.ident;
        let variant_ident = Ident::new(&format!("{ident}{}", variant.ident), variant.ident.span());
        let variant_ident_str = {
            let st = &raw_variant_ident.to_string();
            let st = if st == "None" {
                "None_"
            } else {
                &st
            };
            LitStr::new(&st, raw_variant_ident.span())
        };


        let fields = variant.fields.iter().map(|field| -> Result<(FieldArgs, &Type)> {
            let mut args = parse_field_attrs(&field.attrs)?;
            let ty = &field.ty;
    
            args.name = if let Some(name) = args.name {
                Some(name)
            } else if let Some(name) = field.ident.clone() {
                Some(name)
            } else {
                return Err(Error::new_spanned(field.clone(), "no name specified, expected `#[py_gen(name = \"<name\"]`"))
            };
    
            Ok((args, ty))
        }).collect::<Result<Vec<_>>>()?;

        let field_comments = variant.fields.iter().map(|field| {
            get_doc_comments(&field.attrs)
        }).collect::<Vec<TokenStream>>();

        let new_fields: TokenStream = fields.iter().map(|(args, ty)| {
            let ident = args.name.clone().unwrap();
            match &args.remote {
                Some(val) => quote!(#ident: #val,),
                None => quote!(#ident: #ty,),
            }
        }).collect();

        let constructor = match &variant.fields {
            syn::Fields::Named(_) => gen_construction_args(true, &fields),
            syn::Fields::Unnamed(_) => gen_construction_args(false, &fields),
            syn::Fields::Unit => quote!(),
        };

        

        let deconstruct_fields: TokenStream = match &variant.fields {
            syn::Fields::Named(_) => {
                let field_idents: TokenStream = fields.iter().map(|(field, _)| {
                    let ident = field.name.clone().unwrap();
                    let ident_new = Ident::new(&format!("_{ident}"), ident.span());
                    quote!(#ident: #ident_new,)
                }).collect();

                quote!({#field_idents})
            },
            syn::Fields::Unnamed(_) => {
                let field_idents: TokenStream = fields.iter().map(|(field, _)| {
                    let ident = field.name.clone().unwrap();
                    let ident = Ident::new(&format!("_{ident}"), ident.span());
                    quote!(#ident,)
                }).collect();

                quote!((#field_idents))
            },
            syn::Fields::Unit => quote!(),
        };

        let getters: TokenStream = fields.iter().zip(field_comments).map(|((args, ty),comments)| {
            let raw_arg_ident = args.name.clone().unwrap();
            let get_ident = Ident::new(&format!("get_{raw_arg_ident}"), raw_arg_ident.span());
            let arg_ident = Ident::new(&format!("_{raw_arg_ident}"), raw_arg_ident.span());
            
            let (ret, ty) = match &args.remote {
                Some(remote) => (
                    quote!(<#ty as ::interface_macros::PyBridge<#remote>>::into_py(#arg_ident.clone(), py)), 
                    quote!(<#ty as ::interface_macros::PyBridge<#remote>>::PyOut)
                ),
                None => (quote!(Ok(#arg_ident.clone())), quote!(#ty)),
            };

            quote!{
                #[getter]
                #comments
                fn #get_ident(self_: ::pyo3::PyRef<'_, Self>, py: ::pyo3::Python) -> ::pyo3::PyResult<#ty> {
                    let super_ = self_.as_ref();

                    if let #remote::#raw_variant_ident #deconstruct_fields = &super_.0 {
                        #ret
                    } else {
                        Err(::pyo3::exceptions::PyTypeError::new_err("Malformed class? (How does this happen?)"))
                    }
                }
            }

        }).collect();

        let withs: TokenStream = fields.iter().map(|(args, ty)| {
            let raw_arg_ident = args.name.clone().unwrap();
            let with_ident = Ident::new(&format!("with_{raw_arg_ident}"), raw_arg_ident.span());
            let arg_ident = Ident::new(&format!("_{raw_arg_ident}"), raw_arg_ident.span());



            let into = match &args.remote {
                Some(remote) => quote!(
                    <#ty as ::interface_macros::PyBridge<#remote>>::from_py(#raw_arg_ident, py)?
                ),
                None => quote!(#raw_arg_ident.into())
            };

            let inner_ty = match &args.remote {
                Some(val) => val,
                None => *ty,
            };

            quote!{
                fn #with_ident(self_: ::pyo3::PyRef<'_, Self>, py: ::pyo3::Python, #raw_arg_ident: #inner_ty) -> ::pyo3::PyResult<::pyo3::Py<Self>> {
                    let super_ = self_.as_ref();
                    let mut val = super_.0.clone();

                    if let #remote::#raw_variant_ident #deconstruct_fields = &mut val {
                        *#arg_ident = #into;
                    } else {
                        return Err(::pyo3::exceptions::PyTypeError::new_err("Malformed class? (How does this happen?)"));
                    }

                    let init = ::pyo3::PyClassInitializer::from(#ident(val)).add_subclass(Self);

                    ::pyo3::Py::new(py, init)
                }
            }
        }).collect();

        let doc_comments = get_doc_comments(&variant.attrs);

        Ok((quote!{
            #[::interface_macros::py_type_gen(nested = #ident)]
            #[::pyo3::pyclass(extends=#ident, name = #variant_ident_str)]
            #doc_comments
            struct #variant_ident;

            #[::interface_macros::py_type_gen]
            #[::pyo3::pymethods]
            impl #variant_ident {
                #[new]
                fn new(py: ::pyo3::Python, #new_fields) -> ::pyo3::PyResult<(Self, #ident)> {
                    Ok((Self, #ident(#remote::#raw_variant_ident #constructor)))
                }
                #getters

                #withs
            }
        }, deconstruct_fields))
    }).collect::<std::result::Result<Vec<(TokenStream, TokenStream)>, Error>>()?
    .into_iter()
    .unzip();

    let variant_defs: TokenStream = variant_parts.0.into_iter().collect();
    let variant_deconstructors = variant_parts.1;

    
    let new_py: TokenStream = input.variants.iter().zip(variant_deconstructors).map(|(variant, deconstructor)| {
        let raw_variant_ident = &variant.ident;
        let variant_ident = Ident::new(&format!("{ident}{}", variant.ident), variant.ident.span());

        quote! {
            #remote::#raw_variant_ident #deconstructor => {
                let init = ::pyo3::PyClassInitializer::from(#ident(self)).add_subclass(#variant_ident);

                ::pyo3::Py::new(py, init)
                    .map(|a| ::pyo3::IntoPy::into_py(a, py))?
                    .extract(py)
            },
        }
    }).collect();

    let class_adders: TokenStream = input.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let ident = Ident::new(&format!("{ident}{variant_ident}"), variant_ident.span());

        quote!(namespace.add_class::<#ident>()?;)
    }).collect();
    
    let remote_str = LitStr::new(&remote.to_string(), ident.span());


    Ok(quote! {
        #[::interface_macros::py_type_gen]
        #[::pyo3::pyclass(subclass, name = #remote_str)]
        #attrs
        #vis struct #ident (#remote);

        #[::pyo3::pymethods]
        #[::interface_macros::py_type_gen]
        impl #ident {
            fn __repr__(&self) -> String {
                ::std::format!("{:?}", self.0)
            }
        }
        impl #ident {
            pub fn add_class(py: ::pyo3::Python, m: &::pyo3::types::PyModule) -> ::pyo3::PyResult<()> {
                let namespace = ::pyo3::types::PyModule::new(py, #remote_str)?;
                namespace.add_class::<#ident>()?;
                #class_adders
                m.add_submodule(namespace)?;

                Ok(())
            }
        }

        impl ::interface_macros::PyBridge<#ident> for #remote {
            type PyOut = ::pyo3::Py<#ident>;

            fn into_py(self, py: ::pyo3::Python) -> ::pyo3::PyResult<Self::PyOut> {
                match &self {
                    #new_py
                }
            }
            fn from_py(val: #ident, _py: ::pyo3::Python) -> ::pyo3::PyResult<Self> {
                Ok(val.0)
            }
        }

        #variant_defs
    })
}