use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl ::yarge::Resource for #name {}
    }
    .into()
}

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl ::yarge::Component for #name {}
    }
    .into()
}

#[proc_macro_attribute]
pub fn system(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // Extract parameter patterns and types
    let mut param_pats = Vec::new();
    let mut param_types = Vec::new();

    for arg in &input.sig.inputs {
        match arg {
            syn::FnArg::Typed(pat_ty) => {
                param_types.push(&pat_ty.ty);
                match &*pat_ty.pat {
                    syn::Pat::Ident(ident) => param_pats.push(ident.ident.clone()),
                    _ => panic!("system parameters must be simple identifiers"),
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("system functions must not take self");
            }
        }
    }

    let param_count = param_types.len();

    // Param tuple type
    let param_tuple = match param_count {
        0 => quote! { () },
        1 => quote! { #(#param_types)* },
        _ => quote! { ( #(#param_types),* ) },
    };

    // Create a unique name for the actual function implementation
    let impl_fn_name = syn::Ident::new(&format!("__{}_impl", fn_name), fn_name.span());

    // How we call the user function
    let call_user_fn = match param_count {
        0 => quote! {
            #impl_fn_name()
        },
        1 => quote! {
            #impl_fn_name(args)
        },
        _ => quote! {
            let ( #(#param_pats),* ) = args;
            #impl_fn_name( #(#param_pats),* )
        },
    };

    // Create the impl function signature
    let mut impl_sig = sig.clone();
    impl_sig.ident = impl_fn_name.clone();

    // Create a unique wrapper type name based on the function name
    let wrapper_name = syn::Ident::new(&format!("__SystemWrapper_{}", fn_name), fn_name.span());

    // Create the intput type for function overloading
    let new_fn_input = match param_count {
        0 => quote! { () },
        _ => quote! { ( #(#param_types),* , ) },
    };
    let new_call_user_fn = match param_count {
        0 => quote! {
            #impl_fn_name()
        },
        1 => quote! {
            #impl_fn_name( args.0 )
        },
        _ => quote! {
            let ( #(#param_pats),* ) = args;
            #impl_fn_name( #(#param_pats),* )
        },
    };
    // Create the output type for function overloading
    let new_fn_output = match &sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    quote! {
        // Original function with renamed implementation name
        #vis #impl_sig #block

        // Create a zero-sized wrapper type for this specific function
        #[allow(non_camel_case_types)]
        #vis struct #wrapper_name;

        impl crate::IntoSystem for #wrapper_name {
            fn into_system(&self) -> Box<dyn crate::SystemTrait> {
                Box::new(
                    crate::SystemFuncWrapper::<_, #param_tuple> {
                        function: move |args| {
                            #call_user_fn
                        },
                        state: None,
                        _marker: std::marker::PhantomData,
                    }
                )
            }
        }

        // Overload caller so that the original function call still works using the final constant
        impl FnOnce<#new_fn_input> for #wrapper_name {
            type Output = #new_fn_output;
            extern "rust-call" fn call_once(self, args: #new_fn_input) -> Self::Output {
                #new_call_user_fn
            }
        }

        impl FnMut<#new_fn_input> for #wrapper_name {
            extern "rust-call" fn call_mut(&mut self, args: #new_fn_input) -> Self::Output {
                #new_call_user_fn
            }
        }

        impl Fn<#new_fn_input> for #wrapper_name {
            extern "rust-call" fn call(&self, args: #new_fn_input) -> Self::Output {
                #new_call_user_fn
            }
        }

        // Create a constant with the same name as the original function
        #[allow(non_upper_case_globals)]
        #vis const #fn_name: #wrapper_name = #wrapper_name;
    }
    .into()
}
