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
    let return_type = gen.return_type();
    let arg_list = gen.args_list();
    let call = gen.call();

    quote::quote! {
        #[no_mangle]
        pub extern "C" fn #abi_fn_ident(#arg_list) -> #return_type {
            #call
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
