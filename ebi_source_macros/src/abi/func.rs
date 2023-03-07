use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

use super::args::GenArgsExt;
use super::args::{
    ChapterListFunction, ChapterPageListFunction, MangaListFunction, SourceFunction,
};

pub struct FnGenerator {
    name: TokenStream,
    gen: Box<dyn GenArgsExt>,
}

impl FnGenerator {
    pub fn args_list(&self) -> TokenStream {
        self.gen.args_list()
    }

    // pub fn args_parsing(&self) -> TokenStream {
    //     self.gen.args_parsing()
    // }

    pub fn return_type(&self) -> TokenStream {
        self.gen.return_type()
    }

    pub fn call(&self) -> TokenStream {
        let name = &self.name;
        self.gen.call(name)
    }
}

pub enum AbiFns {
    SourceInfo,
    MangaList,
    ChapterList,
    ChapterPageList,
}

impl TryFrom<&Ident> for AbiFns {
    type Error = syn::Error;

    fn try_from(name: &Ident) -> Result<Self, Self::Error> {
        match name.to_string().as_str() {
            "source_info" => Ok(Self::SourceInfo),
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
            Self::SourceInfo => Box::new(SourceFunction {}),
            Self::MangaList => Box::new(MangaListFunction {}),
        };

        Ok(FnGenerator {
            name,
            gen: generator,
        })
    }
}
