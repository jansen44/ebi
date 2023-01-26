/// Right now both of these two implementations SHOULD NOT be used since the
/// current "ebi-abi protocol" uses only JSON via String (*const c_char).
///
/// This decision was made due to how complex things can be with Rust and ABI.
/// The first problem is how to deal with vecs, since in order to work with
/// *const to custom types (e.g., *const Manga) we need to (basically speaking)
/// leak the memory out with something like std::mem::forget. That's because
/// Rust handle the memory-cleaning for us automatically, but in this case
/// it does so even before getting the value out of the function.
///
/// Leaking memory like this without a proper way of cleaning afterwards is
/// not a good idea. Because of this, I adopted a WAY simpler but slower
/// approach: serializing my data structures to JSON using serde and passing
/// them as *const mut c_char. Although very simplistic, it became easier to
/// let Rust do his job without any manual leaking.
///
/// This "ebi-abi protocol" with JSON approach is subject to change in the
/// future. Until then, please, refrain from using this ABI implementation.
pub mod manga;
pub mod source;
