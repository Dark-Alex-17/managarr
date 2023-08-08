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
  settings,
  filter,
  sort,
  edit,
  logs,
  tasks,
  restrictions,
  refresh,
  update,
  events,
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
  settings: KeyBinding {
    key: Key::Char('s'),
    desc: "Settings",
  },
  filter: KeyBinding {
    key: Key::Char('f'),
    desc: "Filter",
  },
  sort: KeyBinding {
    key: Key::Char('o'),
    desc: "Sort",
  },
  edit: KeyBinding {
    key: Key::Char('e'),
    desc: "Edit",
  },
  events: KeyBinding {
    key: Key::Char('e'),
    desc: "Events",
  },
  logs: KeyBinding {
    key: Key::Char('l'),
    desc: "Logs",
  },
  tasks: KeyBinding {
    key: Key::Char('t'),
    desc: "Tasks",
  },
  restrictions: KeyBinding {
    key: Key::Char('t'),
    desc: "Restrictions",
  },
  refresh: KeyBinding {
    key: Key::Char('r'),
    desc: "Refresh",
  },
  update: KeyBinding {
    key: Key::Char('u'),
    desc: "Update",
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
