use convert_case::{Case, Casing};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_quote, spanned::Spanned, Data, DeriveInput, GenericParam, Generics};

pub(crate) fn proc(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;

    let generics = add_trait_bounds(input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let lower = input
        .attrs
        .iter()
        .any(|attr| match attr.parse_meta().unwrap() {
            syn::Meta::Path(ref path) => *path.get_ident().unwrap() == "lower",
            _ => false,
        });
    let body = serializer_body(&input.data, lower);

    let expanded = quote! {
        impl #impl_generics fluxmc::Nbt for #name #ty_generics #where_clause {
            fn nbt(&self) -> fluxmc::nbt::Value {
                #![allow(unreachable_code)]
                let mut out = std::collections::HashMap::new();
                #body
                fluxmc::nbt::Value::Compound(out)
            }
        }
    };

    expanded.into()
}

fn serializer_body(data: &Data, lower: bool) -> TokenStream {
    match data {
        Data::Struct(str) => proc_fields(&str.fields, quote!(&self.), false),
        Data::Enum(en) => {
            let variants = en.variants.iter().map(|var| {
                let name = &var.ident;
                let field_names = if var.fields.is_empty() {
                    TokenStream::new()
                } else {
                    match &var.fields {
                        syn::Fields::Named(ref named) => {
                            let fields = named.named.iter().map(|it| &it.ident);
                            quote! { { #(#fields),* } }
                        }
                        syn::Fields::Unnamed(ref unnamed) => {
                            let fields = unnamed
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, it)| Ident::new(&format!("_{index}"), it.span()));
                            quote! { ( #(#fields),* )}
                        }
                        _ => quote! { compile_error!("Unit types are not supported") },
                    }
                };
                let body = if var.fields.is_empty() {
                    // providing an identifying string
                    let ident = if lower {
                        let ident = &var.ident.to_string();
                        ident.to_case(Case::Snake)
                    } else {
                        var.ident.to_string()
                    };

                    quote! {
                        return fluxmc::nbt::Value::String(#ident.to_owned())
                    }
                } else {
                    proc_fields(
                        &var.fields,
                        TokenStream::new(),
                        matches!(var.fields, syn::Fields::Unnamed(_)),
                    )
                };
                quote! {
                    Self::#name #field_names => {
                        #body
                    }
                }
            });

            quote! {
                match self {
                    #(#variants)*
                }
            }
        }
        Data::Union(_) => quote! {
            compile_error!("Unions are not supported for NBT serialization")
        },
    }
}

fn proc_fields(fields: &syn::Fields, prefix: TokenStream, u_prefix: bool) -> TokenStream {
    match fields {
        syn::Fields::Named(ref named) => {
            let names = named.named.iter().map(|f| {
                // checking for the rename attr
                let mut name: String = f.ident.clone().unwrap().to_string();
                for attr in &f.attrs {
                    if let Some(new_name) = match attr.parse_meta().unwrap() {
                        syn::Meta::Path(path) => {
                            if *path.get_ident().unwrap() == "rename" {
                                let renamed: Literal = attr.parse_args().unwrap();
                                Some(renamed.to_string())
                            } else {
                                None
                            }
                        }
                        syn::Meta::List(list) => {
                            let path = list.path;
                            if *path.get_ident().unwrap() == "rename" {
                                let renamed: Literal = attr.parse_args().unwrap();
                                let st = renamed.to_string();
                                Some(st[1..st.len() - 1].to_owned()) // strip off the "" parts
                            } else {
                                None
                            }
                        }
                        _ => None,
                    } {
                        name = new_name;
                        break;
                    } else {
                        continue;
                    }
                }
                quote_spanned! { f.span()=> #name.to_owned() }
            });
            let recurse = named.named.iter().map(|f| {
                let name = if u_prefix {
                    Some(Ident::new(
                        &format!("_{}", f.ident.clone().unwrap()),
                        f.ident.span(),
                    ))
                } else {
                    f.ident.clone()
                };
                quote_spanned! { f.span()=>
                    fluxmc::nbt::Nbt::nbt(#prefix #name)
                }
            });

            quote! {
                #(
                    out.insert(#names, #recurse);
                )*
            }
        }
        syn::Fields::Unnamed(unnamed) => {
            let count = unnamed.unnamed.iter().count() as u32;
            let names = (0..count).into_iter().map(|it| it.to_string());
            let recurse = unnamed.unnamed.iter().enumerate().map(|(pos, f)| {
                if u_prefix {
                    let name = Ident::new(&format!("_{pos}"), f.ident.span());
                    quote_spanned! { f.span()=>
                        nbt::Nbt::nbt(#prefix #name)
                    }
                } else {
                    let pos = syn::Index::from(pos);
                    quote_spanned! { f.span()=>
                        nbt::Nbt::nbt(#prefix #pos)
                    }
                }
            });
            quote! {
                #(
                    out.insert(#names.to_owned(), #recurse);
                )*
            }
        }
        syn::Fields::Unit => quote! {
            compile_error!("Unit types are not supported for nbt conversion")
        },
    }
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(fluxmc::nbt::Nbt));
        }
    }
    generics
}
