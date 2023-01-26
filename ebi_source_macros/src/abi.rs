use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::ItemFn;

pub fn gen_ebi_plugin(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = match syn::parse::<ItemFn>(input.clone()) {
        Ok(ast) => ast,
        // on parse error, make IDEs happy; see fn docs
        Err(err) => return input_and_compile_error(input, err),
    };

    let name = ast.sig.ident.clone();

    // TODO: Handle more functions
    if name.to_string() != String::from("source") {
        return input_and_compile_error(
            input,
            syn::Error::new(Span::call_site(), r#"invalid function name"#),
        );
    }

    let func_name: proc_macro2::TokenStream = format!("abi_{}", name.to_string()).parse().unwrap();

    let gen = quote::quote! {
        use std::ffi::{c_char, CString};
        use ebi_source::{Deserialize, Serialize, serde_json};

        #[no_mangle]
        pub extern "C" fn #func_name() -> *mut c_char {
            let src = source();
            let src = serde_json::to_string(&src).unwrap();
            let src = CString::new(src).unwrap();

            src.into_raw()
        }
    };

    let gen: TokenStream = gen.into();
    let mut input = input.clone();
    input.extend(gen);
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