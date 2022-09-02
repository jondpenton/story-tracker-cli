use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

extern crate proc_macro;

#[proc_macro_derive(BrandedInt)]
pub fn derive_branded(item: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(item as DeriveInput);
  let ident = input.ident;
  let data_struct = match input.data {
    Data::Struct(data_struct) => Some(data_struct),
    _ => None,
  }
  .unwrap();
  let struct_type = &data_struct.fields.iter().next().unwrap().ty;
  let type_path = match struct_type {
    Type::Path(type_path) => Some(type_path),
    _ => None,
  }
  .unwrap();
  let type_ident = &type_path.path.segments[0].ident;

  // Build the output, possibly using quasi-quotation
  let tokens = quote! {
    impl core::ops::Deref for #ident {
      type Target = #type_ident;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl std::fmt::Display for #ident {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
      }
    }
  };

  // Hand the output tokens back to the compiler
  TokenStream::from(tokens)
}
