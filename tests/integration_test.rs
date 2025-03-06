use enum_display_style_derive::EnumDisplayStyle;
use pretty_assertions::assert_str_eq;

#[test]
fn test_derive_enum_display_style() {
	assert_str_eq!(TestEnum::Test.to_display_str(), "Testing 123");
	assert_str_eq!(TestEnum::Ignored.to_display_str(), "Ignored");
}

#[derive(EnumDisplayStyle)]
pub enum TestEnum {
	#[display_style(name = "Testing 123")]
	Test,
	Ignored,
}