use crate::event::Key;

macro_rules! generate_keybindings {
    ($($field:ident),+) => {
        pub struct KeyBindings {
            $(pub $field: KeyBinding),+
        }
    };
}

generate_keybindings! {
    quit,
    up,
    down,
    submit,
    esc
}

pub struct KeyBinding {
    pub key: Key,
    pub desc: &'static str
}

pub const DEFAULT_KEYBINDINGS: KeyBindings = KeyBindings {
    quit: KeyBinding {
        key: Key::Char('q'),
        desc: "Quit",
    },
    up: KeyBinding {
        key: Key::Up,
        desc: "Scroll up"
    },
    down: KeyBinding {
        key: Key::Down,
        desc: "Scroll down"
    },
    submit: KeyBinding {
        key: Key::Enter,
        desc: "Select"
    },
    esc: KeyBinding {
        key: Key::Esc,
        desc: "Exit menu"
    }
};