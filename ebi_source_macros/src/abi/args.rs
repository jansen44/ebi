use proc_macro2::TokenStream;

pub trait GenArgsExt {
    fn args_list(&self) -> TokenStream {
        quote::quote! {}
    }

    fn args_parsing(&self) -> TokenStream {
        quote::quote! {}
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! { #name() }
    }
}

pub struct NonArgFunctions;
impl GenArgsExt for NonArgFunctions {}

pub struct ChapterListFunction;

impl GenArgsExt for ChapterListFunction {
    fn args_list(&self) -> TokenStream {
        quote::quote! { manga: ABIChapterListInput }
    }

    fn args_parsing(&self) -> TokenStream {
        quote::quote! {
            let (manga_identifier, manga_url) = unsafe {
                (CString::from_raw(manga.manga_identifier), CString::from_raw(manga.manga_url))
            };

            let manga_identifier = manga_identifier.to_string_lossy();
            let manga_url = manga_url.to_string_lossy();
        }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! { #name(manga_identifier.into_owned(), manga_url.into_owned()) }
    }
}

pub struct ChapterPageListFunction;

impl GenArgsExt for ChapterPageListFunction {
    fn args_list(&self) -> TokenStream {
        quote::quote! { chapter: ABIChapterPageListInput }
    }

    fn args_parsing(&self) -> TokenStream {
        quote::quote! {
            let (chapter_url, manga_identifier) = unsafe {
                (CString::from_raw(chapter.chapter_url), CString::from_raw(chapter.manga_identifier))
            };

            let chapter_url = chapter_url.to_string_lossy();
            let manga_identifier = manga_identifier.to_string_lossy();
        }
    }

    fn call(&self, name: &TokenStream) -> TokenStream {
        quote::quote! { #name(chapter.chapter, chapter_url.into_owned(), manga_identifier.into_owned()) }
    }
}
