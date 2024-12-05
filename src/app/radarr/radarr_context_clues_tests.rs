#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::radarr_context_clues::{
    ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES, COLLECTIONS_CONTEXT_CLUES,
    COLLECTION_DETAILS_CONTEXT_CLUES, LIBRARY_CONTEXT_CLUES,
    MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES, MANUAL_MOVIE_SEARCH_CONTEXT_CLUES,
    MOVIE_DETAILS_CONTEXT_CLUES, SYSTEM_TASKS_CONTEXT_CLUES,
  };

  #[test]
  fn test_library_context_clues() {
    let mut library_context_clues_iter = LIBRARY_CONTEXT_CLUES.iter();

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.add);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.add.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.delete);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.delete.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update all");

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = library_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(library_context_clues_iter.next(), None);
  }

  #[test]
  fn test_collections_context_clues() {
    let mut collections_context_clues = COLLECTIONS_CONTEXT_CLUES.iter();

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.search.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.filter);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.filter.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, "update all");

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = collections_context_clues.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "cancel filter");
    assert_eq!(collections_context_clues.next(), None);
  }

  #[test]
  fn test_movie_details_context_clues() {
    let mut movie_details_context_clues_iter = MOVIE_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = movie_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(movie_details_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_movie_search_context_clues() {
    let mut manual_movie_search_context_clues_iter = MANUAL_MOVIE_SEARCH_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.refresh);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.refresh.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.update);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.update.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.edit);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.edit.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.sort);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.sort.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.auto_search);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.auto_search.desc);

    let (key_binding, description) = manual_movie_search_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(manual_movie_search_context_clues_iter.next(), None);
  }

  #[test]
  fn test_manual_movie_search_contextual_context_clues() {
    let mut manual_movie_search_contextual_context_clues_iter =
      MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES.iter();

    let (key_binding, description) = manual_movie_search_contextual_context_clues_iter
      .next()
      .unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");
    assert_eq!(
      manual_movie_search_contextual_context_clues_iter.next(),
      None
    );
  }

  #[test]
  fn test_add_movie_search_results_context_clues() {
    let mut add_movie_search_results_context_clues_iter =
      ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES.iter();

    let (key_binding, description) = add_movie_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "details");

    let (key_binding, description) = add_movie_search_results_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, "edit search");
    assert_eq!(add_movie_search_results_context_clues_iter.next(), None);
  }

  #[test]
  fn test_system_tasks_context_clues() {
    let mut system_tasks_context_clues_iter = SYSTEM_TASKS_CONTEXT_CLUES.iter();

    let (key_binding, description) = system_tasks_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "start task");

    let (key_binding, description) = system_tasks_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(system_tasks_context_clues_iter.next(), None);
  }

  #[test]
  fn test_collection_details_context_clues() {
    let mut collection_details_context_clues_iter = COLLECTION_DETAILS_CONTEXT_CLUES.iter();

    let (key_binding, description) = collection_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.submit);
    assert_str_eq!(*description, "show overview/add movie");

    let (key_binding, description) = collection_details_context_clues_iter.next().unwrap();

    assert_eq!(*key_binding, DEFAULT_KEYBINDINGS.esc);
    assert_str_eq!(*description, DEFAULT_KEYBINDINGS.esc.desc);
    assert_eq!(collection_details_context_clues_iter.next(), None);
  }
}
