use proc_macro::TokenStream;

mod abi;

#[proc_macro_attribute]
pub fn ebi_plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    abi::gen_ebi_plugin(args, input)
}
