mod args;
mod func;

use self::func::{AbiFns, FnGenerator};
use syn::{Ident, Signature};

fn abi_ident(ident: &Ident) -> proc_macro2::TokenStream {
    let abi_fn_name = format!("abi_{}", ident.to_string());
    abi_fn_name.parse().unwrap()
}

fn abi_fn_from_generator(
    gen: FnGenerator,
    abi_fn_ident: proc_macro2::TokenStream,
) -> proc_macro::TokenStream {
    let arg_list = gen.args_list();
    let convert = gen.args_parsing();
    let call = gen.call();

    // TODO: Better generated code
    quote::quote! {
        #[no_mangle]
        pub extern "C" fn #abi_fn_ident(#arg_list) -> *mut ffi::c_char {
            use ffi::{c_char, CString};

            #convert;
            let resource = #call;

            let default_error = CString::new("{ \"error\": \"Serialize\" }")
                .unwrap()
                .into_raw();

            // Ok -> serialize to JSON string
            // Err -> return error as JSON string
            let resource = match resource {
                Ok(resource) => serde_json::to_string(&resource),
                Err(err) => {
                    let err = SourceErrorSerialized {
                        error: err,
                    };

                    let err = serde_json::to_string(&err);
                    if let Err(e) = err {
                        return default_error;
                    }

                    let err = CString::new(err.unwrap());
                    if let Err(e) = err {
                        return default_error;
                    }

                    return err.unwrap().into_raw();
                },
            };

            // Ok -> resource JSON string
            // Err -> SourceError::Serialize error as JSON string
            let resource = match resource {
                Ok(resource) => resource,
                Err(_) => {
                    let err = SourceErrorSerialized {
                        error: SourceError::Serialize,
                    };
                    let err = serde_json::to_string(&err);
                    if let Err(e) = err {
                        return default_error;
                    }
                    err.unwrap()
                },
            };

            let resource = CString::new(resource);
            if let Err(e) = resource {
                return default_error;
            }

            resource.unwrap().into_raw()
        }
    }
    .into()
}

pub fn gen_abi_fn(signature: &Signature) -> Result<proc_macro::TokenStream, syn::Error> {
    let name = &signature.ident;
    let abi_fn_ident = abi_ident(name);
    let fn_gen = AbiFns::generator(name)?;
    Ok(abi_fn_from_generator(fn_gen, abi_fn_ident))
}
