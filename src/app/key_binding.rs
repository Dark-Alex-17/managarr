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
  pg_down,
  pg_up,
  backspace,
  next_servarr,
  previous_servarr,
  clear,
  search,
  auto_search,
  settings,
  help,
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
  pub alt: Option<Key>,
  pub desc: &'static str,
}

pub const DEFAULT_KEYBINDINGS: KeyBindings = KeyBindings {
  add: KeyBinding {
    key: Key::Char('a'),
    alt: None,
    desc: "add",
  },
  up: KeyBinding {
    key: Key::Up,
    alt: Some(Key::Char('k')),
    desc: "up",
  },
  down: KeyBinding {
    key: Key::Down,
    alt: Some(Key::Char('j')),
    desc: "down",
  },
  left: KeyBinding {
    key: Key::Left,
    alt: Some(Key::Char('h')),
    desc: "left",
  },
  right: KeyBinding {
    key: Key::Right,
    alt: Some(Key::Char('l')),
    desc: "right",
  },
  pg_down: KeyBinding {
    key: Key::PgDown,
    alt: Some(Key::Ctrl('d')),
    desc: "page down",
  },
  pg_up: KeyBinding {
    key: Key::PgUp,
    alt: Some(Key::Ctrl('u')),
    desc: "page up",
  },
  backspace: KeyBinding {
    key: Key::Backspace,
    alt: Some(Key::Ctrl('h')),
    desc: "backspace",
  },
  next_servarr: KeyBinding {
    key: Key::Tab,
    alt: None,
    desc: "next servarr",
  },
  previous_servarr: KeyBinding {
    key: Key::BackTab,
    alt: None,
    desc: "previous servarr",
  },
  clear: KeyBinding {
    key: Key::Char('c'),
    alt: None,
    desc: "clear",
  },
  auto_search: KeyBinding {
    key: Key::Char('S'),
    alt: None,
    desc: "auto search",
  },
  search: KeyBinding {
    key: Key::Char('s'),
    alt: None,
    desc: "search",
  },
  settings: KeyBinding {
    key: Key::Char('S'),
    alt: None,
    desc: "settings",
  },
  help: KeyBinding {
    key: Key::Char('?'),
    alt: None,
    desc: "show/hide keybindings",
  },
  filter: KeyBinding {
    key: Key::Char('f'),
    alt: None,
    desc: "filter",
  },
  sort: KeyBinding {
    key: Key::Char('o'),
    alt: None,
    desc: "sort",
  },
  edit: KeyBinding {
    key: Key::Char('e'),
    alt: None,
    desc: "edit",
  },
  events: KeyBinding {
    key: Key::Char('e'),
    alt: None,
    desc: "events",
  },
  logs: KeyBinding {
    key: Key::Char('L'),
    alt: None,
    desc: "logs",
  },
  tasks: KeyBinding {
    key: Key::Char('t'),
    alt: None,
    desc: "tasks",
  },
  test: KeyBinding {
    key: Key::Char('t'),
    alt: None,
    desc: "test",
  },
  test_all: KeyBinding {
    key: Key::Char('T'),
    alt: None,
    desc: "test all",
  },
  toggle_monitoring: KeyBinding {
    key: Key::Char('m'),
    alt: None,
    desc: "toggle monitoring",
  },
  refresh: KeyBinding {
    key: Key::Ctrl('r'),
    alt: None,
    desc: "refresh",
  },
  update: KeyBinding {
    key: Key::Char('u'),
    alt: None,
    desc: "update",
  },
  home: KeyBinding {
    key: Key::Home,
    alt: None,
    desc: "home",
  },
  end: KeyBinding {
    key: Key::End,
    alt: None,
    desc: "end",
  },
  delete: KeyBinding {
    key: Key::Delete,
    alt: None,
    desc: "delete",
  },
  submit: KeyBinding {
    key: Key::Enter,
    alt: None,
    desc: "submit",
  },
  confirm: KeyBinding {
    key: Key::Ctrl('s'),
    alt: None,
    desc: "submit",
  },
  quit: KeyBinding {
    key: Key::Char('q'),
    alt: None,
    desc: "quit",
  },
  esc: KeyBinding {
    key: Key::Esc,
    alt: None,
    desc: "close",
  },
};

#[macro_export]
macro_rules! matches_key {
  ($binding:ident, $key:expr) => {
    $crate::app::key_binding::DEFAULT_KEYBINDINGS.$binding.key == $key
      || ($crate::app::key_binding::DEFAULT_KEYBINDINGS
        .$binding
        .alt
        .is_some()
        && $crate::app::key_binding::DEFAULT_KEYBINDINGS
          .$binding
          .alt
          .unwrap()
          == $key)
  };
  ($binding:ident, $key:expr, $ignore_special_keys:expr) => {
    $crate::app::key_binding::DEFAULT_KEYBINDINGS.$binding.key == $key
      || !$ignore_special_keys
        && ($crate::app::key_binding::DEFAULT_KEYBINDINGS
          .$binding
          .alt
          .is_some()
          && $crate::app::key_binding::DEFAULT_KEYBINDINGS
            .$binding
            .alt
            .unwrap()
            == $key)
  };
}
