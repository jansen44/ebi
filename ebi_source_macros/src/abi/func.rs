use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

use super::args::{ChapterListFunction, ChapterPageListFunction, GenArgsExt, NonArgFunctions};

pub struct FnGenerator {
    name: TokenStream,
    gen: Box<dyn GenArgsExt>,
}

impl FnGenerator {
    pub fn args_list(&self) -> TokenStream {
        self.gen.args_list()
    }

    pub fn args_parsing(&self) -> TokenStream {
        self.gen.args_parsing()
    }

    pub fn call(&self) -> TokenStream {
        let name = &self.name;
        self.gen.call(name)
    }
}

pub enum AbiFns {
    Source,
    MangaList,
    ChapterList,
    ChapterPageList,
}

impl TryFrom<&Ident> for AbiFns {
    type Error = syn::Error;

    fn try_from(name: &Ident) -> Result<Self, Self::Error> {
        match name.to_string().as_str() {
            "source" => Ok(Self::Source),
            "manga_list" => Ok(Self::MangaList),
            "chapter_list" => Ok(Self::ChapterList),
            "chapter_page_list" => Ok(Self::ChapterPageList),
            _ => Err(syn::Error::new(name.span(), "Invalid function name")),
        }
    }
}

impl AbiFns {
    pub fn generator(name: &Ident) -> Result<FnGenerator, syn::Error> {
        let abi_func = Self::try_from(name)?;
        let name = name.to_token_stream();

        let generator: Box<dyn GenArgsExt> = match abi_func {
            Self::ChapterList => Box::new(ChapterListFunction {}),
            Self::ChapterPageList => Box::new(ChapterPageListFunction {}),
            _ => Box::new(NonArgFunctions {}),
        };

        Ok(FnGenerator {
            name,
            gen: generator,
        })
    }
}
