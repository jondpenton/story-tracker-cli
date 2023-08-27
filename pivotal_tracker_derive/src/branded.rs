use quote::{__private::TokenStream, quote};
use syn::{Data, DeriveInput, Ident, Type};

pub fn get_branded_impls(ident: &Ident, type_ident: &Ident) -> TokenStream {
  let impls = quote! {
    impl core::ops::Deref for #ident {
      type Target = #type_ident;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
  };

  impls
}

pub fn get_branded_type(input: &DeriveInput) -> &Ident {
  let data_struct = match &input.data {
    Data::Struct(data_struct) => Some(data_struct),
    _ => None,
  };
  let struct_type = &data_struct
    .expect(&format!("Failed to get data structure of {}", input.ident))
    .fields
    .iter()
    .next()
    .expect(&format!("Failed to get first field of {}", input.ident))
    .ty;
  let type_path = match struct_type {
    Type::Path(type_path) => Some(type_path),
    _ => None,
  };

  &type_path
    .expect(&format!(
      "Expected first field type of {} to be Type::Path",
      input.ident
    ))
    .path
    .segments[0]
    .ident
}
