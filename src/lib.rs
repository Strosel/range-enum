use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Error, Result};

fn expr_to_usize(expr: Box<syn::Expr>) -> Result<usize> {
    let err = "range-enum requires a usize range";
    if let syn::Expr::Lit(lit) = *expr {
        if let syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        } = lit
        {
            int.base10_parse()
        } else {
            Err(Error::new(lit.span(), err))
        }
    } else {
        Err(Error::new(expr.span(), err))
    }
}

/// An attribute macro that generates enum variants from a range.
/// # Example
/// ```
/// use range_enum::range_enum;
///
/// #[range_enum(1..=2)]
/// enum Range {}
///
/// let _: Range = Range::Range1;
/// ```
/// # Note
/// Any existing variants or generics are erased by this attribute
#[proc_macro_attribute]
pub fn range_enum(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let range: syn::ExprRange = match syn::parse(input) {
        Ok(range) => range,
        Err(_) => {
            return Error::new(
                proc_macro2::Span::call_site(),
                "range-enum requires a range",
            )
            .into_compile_error()
            .into();
        }
    };
    let range_span = range.limits.span();

    let from = match range.from.map(expr_to_usize).transpose() {
        Ok(from) => from.unwrap_or(0),
        Err(e) => {
            return e.into_compile_error().into();
        }
    };
    let to = match range.to.map(expr_to_usize).transpose() {
        Ok(Some(to)) => to,
        Ok(None) => {
            return Error::new(range_span, "range-enum requires an explicit end of range")
                .into_compile_error()
                .into();
        }
        Err(e) => {
            return e.into_compile_error().into();
        }
    };

    let mut ty: syn::ItemEnum = match syn::parse(annotated_item) {
        Ok(ty) => ty,
        Err(_) => {
            return Error::new(
                proc_macro2::Span::call_site(),
                "range-enum requires an enum",
            )
            .into_compile_error()
            .into();
        }
    };
    ty.generics = Default::default();

    let ident = ty.ident.clone();

    ty.variants = dbg!(match range.limits {
        syn::RangeLimits::HalfOpen(_) => from..=(to - 1),
        syn::RangeLimits::Closed(_) => from..=to,
    })
    .map(|n| -> syn::Variant {
        let ident = proc_macro2::Ident::new(&format!("{ident}{n}"), ident.span());
        syn::parse(quote!(#ident).into()).unwrap()
    })
    .inspect(|_| println!("hey"))
    .collect();

    quote! {
        #ty
    }
    .into()
}
