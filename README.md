# Ebi Manga Reader

Ebi is a manga reader backend targeting desktop based on plugins inspired by [Tachiyomi](https://github.com/tachiyomiorg/tachiyomi) (which is incredible for mobile use).

## EARLY DEVELOPMENT

A lot has to be done. Right now there's only one UI, but it's being used merely for debugging. Feel free to build a UI if you want. Right now there's no way to download Sources: you need to build them by hand and manually move them to $HOME/.ebi/sources/{source_name}/{source_name}.(dll|lib.so|dylib);

### TODO:

- [x] Simple plugin system;
- [x] Simple download system;
- [x] Macro-based source development support;
- [ ] Improve filesystem interactions;
- [ ] Windows support;
- [ ] Async-ffi for sources;
- [ ] Better ffi error handling and unsafe-behavior prevention for sources;
- [ ] Simple CLI tool for downloading manga / managing extensions;
- [ ] Improve source loader;
- [ ] Source download support;
- [ ] Ebi FFI -- using ebi on another languages;
