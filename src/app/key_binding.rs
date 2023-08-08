use crate::event::Key;

macro_rules! generate_keybindings {
    ($($field:ident),+) => {
        pub struct KeyBindings {
            $(pub $field: KeyBinding),+
        }
    };
}

generate_keybindings! {
  add,
  up,
  down,
  left,
  right,
  backspace,
  search,
  filter,
  home,
  end,
  delete,
  submit,
  quit,
  esc
}

pub struct KeyBinding {
  pub key: Key,
  pub desc: &'static str,
}

pub const DEFAULT_KEYBINDINGS: KeyBindings = KeyBindings {
  add: KeyBinding {
    key: Key::Char('a'),
    desc: "Add",
  },
  up: KeyBinding {
    key: Key::Up,
    desc: "Scroll up",
  },
  down: KeyBinding {
    key: Key::Down,
    desc: "Scroll down",
  },
  left: KeyBinding {
    key: Key::Left,
    desc: "Move left",
  },
  right: KeyBinding {
    key: Key::Right,
    desc: "Move right",
  },
  backspace: KeyBinding {
    key: Key::Backspace,
    desc: "Backspace",
  },
  search: KeyBinding {
    key: Key::Char('s'),
    desc: "Search",
  },
  filter: KeyBinding {
    key: Key::Char('f'),
    desc: "Filter",
  },
  home: KeyBinding {
    key: Key::Home,
    desc: "Home",
  },
  end: KeyBinding {
    key: Key::End,
    desc: "End",
  },
  delete: KeyBinding {
    key: Key::Delete,
    desc: "Delete selected item",
  },
  submit: KeyBinding {
    key: Key::Enter,
    desc: "Select",
  },
  quit: KeyBinding {
    key: Key::Char('q'),
    desc: "Quit",
  },
  esc: KeyBinding {
    key: Key::Esc,
    desc: "Exit current menu",
  },
};
