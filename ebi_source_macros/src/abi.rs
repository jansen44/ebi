use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, Signature};

trait GenArgsExt {
    fn args_list(&self) -> proc_macro2::TokenStream;
    fn args_parsing(&self) -> proc_macro2::TokenStream;
    fn call(&self, name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream;
}

struct NonArgFunctions;
struct ChapterListFunction;

impl GenArgsExt for NonArgFunctions {
    fn args_list(&self) -> proc_macro2::TokenStream {
        quote::quote! {}
    }

    fn args_parsing(&self) -> proc_macro2::TokenStream {
        quote::quote! {}
    }

    fn call(&self, name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote::quote! { #name() }
    }
}

impl GenArgsExt for ChapterListFunction {
    fn args_list(&self) -> proc_macro2::TokenStream {
        quote::quote! { manga: ABIChapterListInput }
    }

    fn args_parsing(&self) -> proc_macro2::TokenStream {
        quote::quote! {
            let (manga_identifier, manga_url) = unsafe {
                (CString::from_raw(manga.manga_identifier), CString::from_raw(manga.manga_url))
            };

            let manga_identifier = manga_identifier.to_string_lossy().into_owned();
            let manga_url = manga_url.to_string_lossy().into_owned();
        }
    }

    fn call(&self, name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        quote::quote! { #name(manga_identifier, manga_url) }
    }
}

struct FuncGenerator {
    name: proc_macro2::TokenStream,
    gen: Box<dyn GenArgsExt>,
}

impl FuncGenerator {
    pub fn args_list(&self) -> proc_macro2::TokenStream {
        self.gen.args_list()
    }

    pub fn args_parsing(&self) -> proc_macro2::TokenStream {
        self.gen.args_parsing()
    }

    pub fn call(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        self.gen.call(name)
    }
}

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
    fn generator(self, name: proc_macro2::TokenStream) -> FuncGenerator {
        match self {
            Self::ChapterList => FuncGenerator {
                name,
                gen: Box::new(ChapterListFunction {}),
            },
            _ => FuncGenerator {
                name,
                gen: Box::new(NonArgFunctions {}),
            },
        }
    }
}

fn gen_abi_function(signature: &Signature) -> Result<TokenStream, syn::Error> {
    let name = &signature.ident;

    let abi_fn_name = format!("abi_{}", name.to_string());
    let abi_fn_name: proc_macro2::TokenStream = abi_fn_name.parse().unwrap();

    let function = AbiFunctions::try_from(name)?.generator(name.to_token_stream());
    let arg_list = function.args_list();
    let convert = function.args_parsing();
    let call = function.call();

    let abi_function = quote::quote! {
        #[no_mangle]
        pub extern "C" fn #abi_fn_name(#arg_list) -> *mut ffi::c_char {
            use ffi::{c_char, CString};

            #convert;
            let resource = #call;
            let resource = match resource {
                Ok(resource) => serde_json::to_string(&resource).unwrap(),
                Err(err) => {
                    let err = SourceErrorSerialized {
                        error: err,
                    };
                    serde_json::to_string(&err).unwrap()
                },
            };
            let resource = CString::new(resource).unwrap();

            resource.into_raw()
        }
    };

    Ok(abi_function.into())
}

pub fn gen_ebi_plugin(_: TokenStream, fn_body: TokenStream) -> TokenStream {
    let ast = match syn::parse::<ItemFn>(fn_body.clone()) {
        Ok(ast) => ast,
        Err(err) => return input_and_compile_error(fn_body, err),
    };

    let signature = ast.sig;
    let abi_function = match gen_abi_function(&signature) {
        Ok(abi_fn) => abi_fn,
        Err(err) => return input_and_compile_error(fn_body, err),
    };

    // TODO: Validate funcion name => allow only pre-defined functions

    let mut input = fn_body.clone();
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
