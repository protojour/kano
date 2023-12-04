use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn from_property(input: syn::ItemEnum) -> syn::Result<proc_macro2::TokenStream> {
    let enum_ident = input.ident;
    let mut impls = vec![];

    {
        let span = enum_ident.span();
        impls.push(quote_spanned! {span=>
            impl ::kano::FromProperty<#enum_ident> for #enum_ident {
                fn from_property(property: #enum_ident) -> Option<Self> {
                    Some(property)
                }
            }
        });
    }

    for variant in input.variants {
        let variant_span = variant.span();
        let variant_ident = variant.ident;

        let unnamed_fields = match variant.fields {
            syn::Fields::Unnamed(unnamed) => unnamed.unnamed,
            _ => {
                return Err(syn::Error::new(
                    variant_span,
                    "Expected tuple-like enum variant",
                ));
            }
        };

        if unnamed_fields.len() != 1 {
            return Err(syn::Error::new(
                unnamed_fields.span(),
                "Expected single enum field",
            ));
        }

        let field = unnamed_fields.into_iter().next().unwrap();
        let span = field.span();
        let ty = field.ty;

        impls.push(quote_spanned! {span=>
            impl ::kano::FromProperty<#ty> for #enum_ident {
                fn from_property(property: #ty) -> Option<Self> {
                    Some(Self::#variant_ident(property))
                }
            }
        });
    }

    Ok(quote! {
        #(#impls)*
    })
}
