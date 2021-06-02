use proc_macro::TokenStream;

use darling::FromMeta;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, parse_macro_input, ReturnType};

#[proc_macro_attribute]
pub fn foo(args: TokenStream, func: TokenStream) -> TokenStream {
    expand(args, func)
}

#[derive(FromMeta)]
struct FooArgs {
    bar: String,
}

pub(crate) fn expand(args: TokenStream, func: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let args: FooArgs = match FooArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let func = parse_macro_input!(func as ItemFn);
    let fn_vis = &func.vis;
    let fn_block = &func.block;
    let fn_attrs = &func.attrs;
    let fn_sig = &func.sig;
    let fn_name = &fn_sig.ident;
    let fn_generics = &fn_sig.generics;
    let fn_args = &fn_sig.inputs;
    let fn_async = &fn_sig.asyncness.unwrap();
    let fn_output = match &fn_sig.output {
        ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
        ReturnType::Default => {
            quote! {()}
        }
    };
    let bar = args.bar;
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name #fn_generics(
            _req_: actix_web::HttpRequest,
            #fn_args
        ) -> #fn_output {
            _req_.extensions_mut().insert(actix_web_custom_macro_example::Foo::new(#bar));
            log::debug!("set foobar: {}", #bar);
            let f = || async move #fn_block;
            f().await
        }
    };
    expanded.into()
}