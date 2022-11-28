use proc::proc;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod proc;

#[proc_macro_derive(Nbt, attributes(lower, rename))]
pub fn derive_nbt(input: TokenStream) -> TokenStream {
    proc(parse_macro_input!(input as DeriveInput))
}
