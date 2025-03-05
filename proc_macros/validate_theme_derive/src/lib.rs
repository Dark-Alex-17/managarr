use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ValidateTheme, attributes(validate))]
pub fn derive_validate_theme(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;

	let mut validation_checks = Vec::new();

	if let Data::Struct(data_struct) = &input.data {
		if let Fields::Named(fields) = &data_struct.fields {
			for field in &fields.named {
				let field_name = &field.ident;

				let has_validate_attr = field.attrs.iter().any(|attr| {
					attr.path().is_ident("validate")
				});

				if has_validate_attr {
					validation_checks.push(quote! {
						if self.#field_name.is_none() {
							log::error!("{} is missing a color value.", stringify!(#field_name));
								eprintln!("{} is missing a color value.", stringify!(#field_name));
								process::exit(1);
						}
					})
				}
			}
		}
	}

	quote! {
		impl #struct_name {
			pub fn validate(&self) {
				#(#validation_checks)*
			}
		}
	}.into()
}
