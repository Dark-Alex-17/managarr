mod macro_models;

use crate::macro_models::DisplayStyleArgs;
use darling::FromVariant;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Derive macro for the EnumDisplayStyle trait.
///
/// # Example
///
/// Using default values for the display style:
///
/// ```
/// use enum_display_style_derive::EnumDisplayStyle;
///
/// #[derive(EnumDisplayStyle)]
/// enum Weekend {
///   Saturday,
///   Sunday,
/// }
///
/// assert_eq!(Weekend::Saturday.to_display_str(), "Saturday");
/// assert_eq!(Weekend::Sunday.to_display_str(), "Sunday");
///
/// ```
///
/// Using custom values for the display style:
///
/// ```
/// use enum_display_style_derive::EnumDisplayStyle;
///
/// #[derive(EnumDisplayStyle)]
/// enum MonitorStatus {
///   #[display_style(name = "Monitor Transactions")]
///   Active,
///   #[display_style(name = "Don't Monitor Transactions")]
///   None,
/// }
///
/// assert_eq!(MonitorStatus::Active.to_display_str(), "Monitor Transactions");
/// assert_eq!(MonitorStatus::None.to_display_str(), "Don't Monitor Transactions");
/// ```
#[proc_macro_derive(EnumDisplayStyle, attributes(display_style))]
pub fn enum_display_style_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let enum_name = &input.ident;

  let mut match_arms = Vec::new();

  if let Data::Enum(data_enum) = &input.data {
    let variants = &data_enum.variants;

    for variant in variants {
      let variant_ident = &variant.ident;
      let variant_display_name = DisplayStyleArgs::from_variant(variant)
        .unwrap()
        .name
        .unwrap_or_else(|| variant_ident.to_string());

      match_arms.push(quote! {
        #enum_name::#variant_ident => #variant_display_name,
      });
    }
  }

  quote! {
    impl<'a> #enum_name {
      pub fn to_display_str(self) -> &'a str {
        match self {
          #(#match_arms)*
        }
      }
    }
  }
  .into()
}
