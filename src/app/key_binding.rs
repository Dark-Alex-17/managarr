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
  next_servarr,
  previous_servarr,
  clear,
  search,
  auto_search,
  settings,
  filter,
  sort,
  edit,
  logs,
  tasks,
  test,
  test_all,
  toggle_monitoring,
  refresh,
  update,
  events,
  home,
  end,
  delete,
  submit,
  confirm,
  quit,
  esc
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct KeyBinding {
  pub key: Key,
  pub desc: &'static str,
}

pub const DEFAULT_KEYBINDINGS: KeyBindings = KeyBindings {
  add: KeyBinding {
    key: Key::Char('a'),
    desc: "add",
  },
  up: KeyBinding {
    key: Key::Up,
    desc: "up",
  },
  down: KeyBinding {
    key: Key::Down,
    desc: "down",
  },
  left: KeyBinding {
    key: Key::Left,
    desc: "left",
  },
  right: KeyBinding {
    key: Key::Right,
    desc: "right",
  },
  backspace: KeyBinding {
    key: Key::Backspace,
    desc: "backspace",
  },
  next_servarr: KeyBinding {
    key: Key::Tab,
    desc: "next servarr",
  },
  previous_servarr: KeyBinding {
    key: Key::BackTab,
    desc: "previous servarr",
  },
  clear: KeyBinding {
    key: Key::Char('c'),
    desc: "clear",
  },
  auto_search: KeyBinding {
    key: Key::Char('S'),
    desc: "auto search",
  },
  search: KeyBinding {
    key: Key::Char('s'),
    desc: "search",
  },
  settings: KeyBinding {
    key: Key::Char('S'),
    desc: "settings",
  },
  filter: KeyBinding {
    key: Key::Char('f'),
    desc: "filter",
  },
  sort: KeyBinding {
    key: Key::Char('o'),
    desc: "sort",
  },
  edit: KeyBinding {
    key: Key::Char('e'),
    desc: "edit",
  },
  events: KeyBinding {
    key: Key::Char('e'),
    desc: "events",
  },
  logs: KeyBinding {
    key: Key::Char('l'),
    desc: "logs",
  },
  tasks: KeyBinding {
    key: Key::Char('t'),
    desc: "tasks",
  },
  test: KeyBinding {
    key: Key::Char('t'),
    desc: "test",
  },
  test_all: KeyBinding {
    key: Key::Char('T'),
    desc: "test all",
  },
  toggle_monitoring: KeyBinding {
    key: Key::Char('m'),
    desc: "toggle monitoring",
  },
  refresh: KeyBinding {
    key: Key::Ctrl('r'),
    desc: "refresh",
  },
  update: KeyBinding {
    key: Key::Char('u'),
    desc: "update",
  },
  home: KeyBinding {
    key: Key::Home,
    desc: "home",
  },
  end: KeyBinding {
    key: Key::End,
    desc: "end",
  },
  delete: KeyBinding {
    key: Key::Delete,
    desc: "delete",
  },
  submit: KeyBinding {
    key: Key::Enter,
    desc: "submit",
  },
  confirm: KeyBinding {
    key: Key::Ctrl('s'),
    desc: "submit",
  },
  quit: KeyBinding {
    key: Key::Char('q'),
    desc: "quit",
  },
  esc: KeyBinding {
    key: Key::Esc,
    desc: "close",
  },
};
