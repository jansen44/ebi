use proc_macro::TokenStream;
use syn::ItemFn;

mod abi;

#[proc_macro_attribute]
pub fn ebi_plugin(_: TokenStream, fn_body: TokenStream) -> TokenStream {
    let ast = match syn::parse::<ItemFn>(fn_body.clone()) {
        Ok(ast) => ast,
        Err(err) => return input_and_compile_error(fn_body, err),
    };

    let abi_fn = match abi::gen_abi_fn(&ast.sig) {
        Ok(abi_fn) => abi_fn,
        Err(err) => return input_and_compile_error(fn_body, err),
    };

    let mut input = fn_body;
    input.extend(abi_fn);
    input
}

/// (Original at: https://github.com/actix/actix-web/blob/6f0a6bd1bb7ffdd98fa5ce825b24a73c4d71d9a7/actix-web-codegen/src/route.rs#L428)
/// Converts the error to a token stream and appends it to the original input.
///
/// Returning the original input in addition to the error is good for IDEs which can gracefully
/// recover and show more precise errors within the macro body.
///
/// See <https://github.com/rust-analyzer/rust-analyzer/issues/10468> for more info.
pub(crate) fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
