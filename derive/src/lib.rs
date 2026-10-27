use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::{LitStr, Type, ext::IdentExt, parse_macro_input, spanned::Spanned, token};

fn type_attrs_into_code(attrs: &Vec<syn::Attribute>) -> Result<Option<impl ToTokens>, syn::Error> {
    let mut enum_code = None;
    for attr in attrs {
        if attr.path().is_ident("ecode") {
            //println!("DERIVE_ATTR: {:?}", attr,);
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("code") {
                    let _: token::Eq = meta.input.parse()?;
                    let lit: LitStr = meta.input.parse()?;
                    let value = lit.parse_with(syn::Ident::parse_any)?;
                    enum_code = Some(quote! {#value});

                    Ok(())
                } else {
                    Err(meta.error("unrecognized attribute `ecode`"))
                }
            })?;
        }
    }
    Ok(enum_code)
}

enum VariantType {
    None,
    Recursive,
    Flatten,
}

fn variant_attrs_into_props(attrs: &Vec<syn::Attribute>) -> Result<VariantType, syn::Error> {
    let mut tp = VariantType::None;
    for attr in attrs {
        if attr.path().is_ident("ecode") {
            //println!("DERIVE_ATTR: {:?}", attr,);
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("recursive") {
                    tp = match &tp {
                        VariantType::None | VariantType::Recursive => VariantType::Recursive,
                        VariantType::Flatten => return Err(meta.error(
                            "invalid attribute `ecode`: `recursive` and `flatten` at the same time",
                        )),
                    };
                    Ok(())
                } else if meta.path.is_ident("flatten") {
                    tp = match &tp {
                        VariantType::None | VariantType::Flatten => VariantType::Flatten,
                        VariantType::Recursive => return Err(meta.error(
                            "invalid attribute `ecode`: `recursive` and `flatten` at the same time",
                        )),
                    };
                    Ok(())
                } else {
                    Err(meta.error("unrecognized attribute `ecode`"))
                }
            })?;
        }
    }
    Ok(tp)
}

fn process_enum(input: syn::ItemEnum) -> Result<TokenStream, syn::Error> {
    // enum
    //println!("enum: {:?}", input);

    let code = type_attrs_into_code(&input.attrs)?;

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut var_values = Vec::new();
    let mut var_enum = Vec::new();
    for variant in input.variants {
        let span = variant.span();
        let var_name = variant.ident;
        let v_type = variant_attrs_into_props(&variant.attrs)?;

        let (params, i18n_code, enumerator) = match v_type {
            VariantType::None => {
                let i18n_code = match &code {
                    Some(ecode) => quote! { stringify!(#ecode.#var_name) },
                    None => quote! { stringify!(#var_name) },
                };
                let i18n_code = quote! { std::borrow::Cow::Borrowed(#i18n_code) };
                let params = match variant.fields {
                    syn::Fields::Unnamed(..) => quote! { (..) },
                    syn::Fields::Named(..) => quote! { {..} },
                    syn::Fields::Unit => quote! {},
                };
                let enumerator = None;
                (params, i18n_code, enumerator)
            }
            VariantType::Recursive | VariantType::Flatten => {
                let i18n_code = match v_type {
                    VariantType::Recursive => match &code {
                        Some(ecode) => quote! { #ecode.#var_name. },
                        None => quote! { #var_name. },
                    },
                    VariantType::Flatten => match &code {
                        Some(ecode) => quote! { #ecode. },
                        None => quote! {},
                    },
                    VariantType::None => unreachable!(),
                };

                match variant.fields {
                    syn::Fields::Unnamed(flds) if flds.unnamed.len() == 1 => {
                        let fld = flds.unnamed.first().unwrap(); // safe: len checked
                        let tpr = match &fld.ty {
                            Type::Path(tp) => &tp.path,
                            tp @ _ => {
                                return Err(syn::parse::Error::new(
                                    tp.span(),
                                    "Expecting type name",
                                ));
                            }
                        };
                        let i18n_enum_code = quote! { stringify!(#i18n_code) };
                        let i18n_code = quote! { std::borrow::Cow::Owned(format!(stringify!(#i18n_code{}), <#tpr as EnumCode>::code(fld))) };
                        let params = quote! { (fld) };
                        let enumerator = Some((
                            quote! {  #i18n_enum_code },
                            quote! { <#tpr as EnumCode>::enumerate() },
                        ));
                        (params, i18n_code, enumerator)
                    }
                    _ => {
                        return Err(syn::parse::Error::new(
                            span,
                            "for `recursive` or `flatten` variant expecting only tuple variant with exactly 1 field, struct and unit variants are forbidden for recursive EnumCode propagating",
                        ));
                    }
                }
            }
        };

        var_enum.push(match enumerator {
            None => quote! {
                enumer.push( #i18n_code );
            },
            Some((prefix, iter_call)) => quote! {
                for code in #iter_call {
                    enumer.push(std::borrow::Cow::Owned(format!("{}{}", #prefix, code)));
                }
            },
        });
        var_values.push(quote! {
            #name::#var_name #params => #i18n_code
        });
    }

    let prefix = match &code {
        Some(ecode) => quote! { Some(stringify!(#ecode)) },
        None => quote! { None },
    };

    Ok(quote! {
        impl #impl_generics EnumCode for #name #ty_generics #where_clause {
            fn code(&self) -> std::borrow::Cow<'static, str> {
                match self {
                    #( #var_values ),*
                }
            }
            fn prefix() -> Option<&'static str> {
                #prefix
            }
            fn enumerate() -> enumerator::CodeEnumerator {
                let mut enumer = Vec::new();
                #( #var_enum );*
                enumer.into()
            }
        }
    }
    .into())
}

fn process_struct(input: syn::ItemStruct) -> Result<TokenStream, syn::Error> {
    // struct
    // println!("struct: {:?}", input);

    let code = type_attrs_into_code(&input.attrs)?;
    let name = input.ident;

    let i18n_code = match &code {
        Some(ecode) => quote! { stringify!(#ecode) },
        None => quote! { stringify!(#name) },
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics EnumCode for #name #ty_generics #where_clause {
            fn code(&self) -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(#i18n_code)
            }
            fn prefix() -> Option<&'static str> {
                Some(#i18n_code)
            }
            fn enumerate() -> enumerator::CodeEnumerator {
                let mut enumer = Vec::new();
                enumer.push(std::borrow::Cow::Borrowed(#i18n_code));
                enumer.into()
            }
        }
    }
    .into())
}

#[proc_macro_derive(EnumCode, attributes(ecode))]
pub fn derive_parse_error_fn(item: TokenStream) -> TokenStream {
    // derive macro keep all code written and can add more

    match parse_macro_input!(item as syn::Item) {
        syn::Item::Enum(input) => match process_enum(input) {
            Ok(ts) => ts,
            Err(e) => e.into_compile_error().into(),
        },
        syn::Item::Struct(input) => match process_struct(input) {
            Ok(ts) => ts,
            Err(e) => e.into_compile_error().into(),
        },
        item @ _ => syn::parse::Error::new(
            item.span(),
            "expecting `enum` or `struct`, only `enum` or `struct` can automatically derive EnumCode trait",
        )
        .into_compile_error()
        .into(),
    }
}
