use proc_macro2::TokenStream;

pub trait GenArgsExt {
    fn args_list(&self) -> TokenStream {
        quote::quote! {}
    }

    fn return_type(&self) -> TokenStream {
        quote::quote! { ebi_source::abi::primitives::ABIResultArray }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! { #name() }
    }
}

pub struct SourceFunction;

impl GenArgsExt for SourceFunction {
    fn return_type(&self) -> TokenStream {
        quote::quote! { ebi_source::abi::source::source_info::ABISourceInfoOutput }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! {
            #name().into()
        }
    }
}

pub struct MangaListFunction;

impl GenArgsExt for MangaListFunction {
    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! {
            #name().into()
        }
    }
}

pub struct ChapterListFunction;

impl GenArgsExt for ChapterListFunction {
    fn args_list(&self) -> TokenStream {
        quote::quote! { manga: ebi_source::abi::chapter::chapter_list::ABIChapterListInput }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! {
            // TODO: remove unwrap
            let identifier = manga.identifier.try_into().unwrap();
            let url = manga.url.try_into().unwrap();

            #name(identifier, url).into()
        }
    }
}

pub struct ChapterPageListFunction;

impl GenArgsExt for ChapterPageListFunction {
    fn args_list(&self) -> TokenStream {
        quote::quote! { chapter: ebi_source::abi::chapter::chapter_page_list::ABIChapterPageListInput }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! {
            // TODO: remove unwrap
            let url = chapter.chapter_url.try_into().unwrap();
            let manga = chapter.manga.try_into().unwrap();

            let chapter = chapter.chapter;

            #name(chapter, url, manga).into()
        }
    }
}
