use crate::event::Key;

macro_rules! generate_keybindings {
    ($($field:ident),+) => {
        pub struct KeyBindings {
            $(pub $field: KeyBinding),+
        }
    };
}

generate_keybindings! {
    quit
}

pub struct KeyBinding {
    key: Key,
    desc: &'static str
}

pub const DEFAULT_KEYBINDINGS: KeyBindings = KeyBindings {
    quit: KeyBinding {
        key: Key::Char('q'),
        desc: "Quit",
    }
};