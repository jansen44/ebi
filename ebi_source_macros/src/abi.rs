use proc_macro::TokenStream;
use syn::{ItemFn, Signature};

enum AbiFunctions {
    Source,
    MangaList,
    ChapterList,
}

impl TryFrom<&proc_macro2::Ident> for AbiFunctions {
    type Error = syn::Error;

    fn try_from(name: &proc_macro2::Ident) -> Result<Self, Self::Error> {
        match name.to_string().as_str() {
            "source" => Ok(Self::Source),
            "manga_list" => Ok(Self::MangaList),
            "chapter_list" => Ok(Self::ChapterList),
            _ => Err(syn::Error::new(name.span(), "Invalid function name")),
        }
    }
}

impl AbiFunctions {
    pub fn arg_list(&self) -> proc_macro2::TokenStream {
        match self {
            AbiFunctions::MangaList | AbiFunctions::Source => String::new(),
            AbiFunctions::ChapterList => String::from("manga: ABIManga"),
        }
        .parse()
        .unwrap()
    }

    pub fn convert(&self) -> proc_macro2::TokenStream {
        match self {
            AbiFunctions::MangaList | AbiFunctions::Source => String::new(),
            AbiFunctions::ChapterList => {
                String::from("let manga: ebi_source::Manga = manga.into()")
            }
        }
        .parse()
        .unwrap()
    }

    pub fn call(&self, name: &str) -> proc_macro2::TokenStream {
        let name: proc_macro2::TokenStream = name.parse().unwrap();
        match self {
            AbiFunctions::ChapterList => quote::quote! { #name(manga) },
            _ => quote::quote! { #name() },
        }
    }

    pub fn convert_json(&self) -> proc_macro2::TokenStream {
        match self {
            AbiFunctions::Source => quote::quote! { serde_json::to_string(&resource).unwrap() },
            _ => quote::quote! {
                match resource {
                    Ok(resource) => serde_json::to_string(&resource).unwrap(),
                    Err(err) => {
                        let err = SourceErrorSerialized {
                            error: err,
                        };
                        serde_json::to_string(&err).unwrap()
                    },
                }
            },
        }
    }
}

pub fn gen_abi_function(signature: &Signature) -> Result<TokenStream, syn::Error> {
    let name = &signature.ident;

    let abi_fn_name = format!("abi_{}", name.to_string());
    let abi_fn_name: proc_macro2::TokenStream = abi_fn_name.parse().unwrap();

    let function = AbiFunctions::try_from(name)?;
    let arg_list = function.arg_list();
    let convert = function.convert();
    let convert_json = function.convert_json();
    let call = function.call(name.to_string().as_str());

    let abi_function = quote::quote! {
        #[no_mangle]
        pub extern "C" fn #abi_fn_name(#arg_list) -> *mut ffi::c_char {
            use ffi::{c_char, CString};

            #convert;
            let resource = #call;
            let resource = #convert_json;
            let resource = CString::new(resource).unwrap();

            resource.into_raw()
        }
    };

    Ok(abi_function.into())
}

pub fn gen_ebi_plugin(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = match syn::parse::<ItemFn>(input.clone()) {
        Ok(ast) => ast,
        Err(err) => return input_and_compile_error(input, err),
    };

    let signature = ast.sig;
    let abi_function = match gen_abi_function(&signature) {
        Ok(abi_fn) => abi_fn,
        Err(err) => return input_and_compile_error(input, err),
    };

    // TODO: Validate funcion name => allow only pre-defined functions

    let mut input = input.clone();
    input.extend(abi_function);
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
