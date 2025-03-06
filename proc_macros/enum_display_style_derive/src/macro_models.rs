use darling::FromVariant;

#[derive(Debug, FromVariant)]
#[darling(attributes(display_style))]
pub struct DisplayStyleArgs {
	pub name: Option<String>
}