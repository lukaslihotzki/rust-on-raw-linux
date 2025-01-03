extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, ImplItemFn, Type};

fn ensure_uninhabited_type(ty: &syn::ReturnType) -> proc_macro2::TokenStream {
    let ty = match ty {
        syn::ReturnType::Type(_, ty) => &**ty,
        syn::ReturnType::Default => &syn::parse_quote!(()),
    };
    if matches!(ty, Type::Never(_)) {
        quote! {}
    } else {
        quote! { const _: fn(#ty) -> () = |x| match x {}; }
    }
}

#[proc_macro_attribute]
pub fn start_linux(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ImplItemFn);
    let ident = &input.sig.ident;
    let ensure_uninhabited_type = ensure_uninhabited_type(&input.sig.output);
    quote! {
        #ensure_uninhabited_type
        start_linux::wrap_start!(#ident);
        #input
    }
    .into()
}
