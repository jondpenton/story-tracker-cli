use branded::{get_branded_impls, get_branded_type};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

mod branded;

#[proc_macro_derive(Branded)]
pub fn derive_branded(item: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(item as DeriveInput);
  let ident = &input.ident;
  let type_ident = get_branded_type(&input);

  // Build the output, possibly using quasi-quotation
  let tokens = get_branded_impls(ident, type_ident);

  // Hand the output tokens back to the compiler
  TokenStream::from(tokens)
}

#[proc_macro_derive(BrandedInt)]
pub fn derive_branded_int(item: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(item as DeriveInput);
  let ident = &input.ident;
  let type_ident = get_branded_type(&input);
  let branded_impls = get_branded_impls(&ident, type_ident);

  // Build the output, possibly using quasi-quotation
  let tokens = quote! {
    #branded_impls

    impl std::fmt::Display for #ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
      }
    }
  };

  // Hand the output tokens back to the compiler
  TokenStream::from(tokens)
}
