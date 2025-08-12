use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Derive macro for generating a `validate` method for a Theme struct.
/// The `validate` method ensures that all values with the `validate` attribute are not `None`.
/// Otherwise, an error message it output to both the log file and stdout and the program exits.
///
/// # Example
///
/// Valid themes pass through the program transitively without any messages being output.
///
/// ```
/// use validate_theme_derive::ValidateTheme;
///
/// #[derive(ValidateTheme, Default)]
/// struct Theme {
///     pub name: String,
///     #[validate]
///     pub good: Option<Style>,
///     #[validate]
///     pub bad: Option<Style>,
///     pub ugly: Option<Style>,
/// }
///
/// struct Style {
///     color: String,
/// }
///
/// let theme = Theme {
///     good: Some(Style { color: "Green".to_owned() }),
///     bad: Some(Style { color: "Red".to_owned() }),
///     ..Theme::default()
/// };
///
/// // Since only `good` and `bad` have the `validate` attribute, the `validate` method will only check those fields.
/// theme.validate();
/// // Since both `good` and `bad` have values, the program will not exit and no message is output.
/// ```
///
/// Invalid themes will output an error message to both the log file and stdout and the program will exit.
///
/// ```should_panic
/// use validate_theme_derive::ValidateTheme;
///
/// #[derive(ValidateTheme, Default)]
/// struct Theme {
///     pub name: String,
///     #[validate]
///     pub good: Option<Style>,
///     #[validate]
///     pub bad: Option<Style>,
///     pub ugly: Option<Style>,
/// }
///
/// struct Style {
///     color: String,
/// }
///
/// let theme = Theme {
///     bad: Some(Style { color: "Red".to_owned() }),
///     ..Theme::default()
/// };
///
/// // Since `good` has the `validate` attribute and since `good` is `None`, the `validate` method will output an error message and exit the program.
/// theme.validate();
/// ```
#[proc_macro_derive(ValidateTheme, attributes(validate))]
pub fn derive_validate_theme(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let struct_name = &input.ident;

  let mut validation_checks = Vec::new();

  if let Data::Struct(data_struct) = &input.data
    && let Fields::Named(fields) = &data_struct.fields
  {
    for field in &fields.named {
      let field_name = &field.ident;

      let has_validate_attr = field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("validate"));

      if has_validate_attr {
        validation_checks.push(quote! {
          if self.#field_name.is_none() {
            log::error!("{} is missing a color value.", stringify!(#field_name));
            eprintln!("{} is missing a color value.", stringify!(#field_name));
            std::process::exit(1);
          }
        })
      }
    }
  }

  quote! {
    impl #struct_name {
      pub fn validate(&self) {
        #(#validation_checks)*
      }
    }
  }
  .into()
}
