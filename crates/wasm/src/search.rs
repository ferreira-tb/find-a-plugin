use super::plugins;
use smallvec::SmallVec;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn search(query: &str) -> Result<Vec<String>, JsError> {
  let words = query
    .split_whitespace()
    .map(str::to_lowercase)
    .collect::<Vec<_>>();

  let mut matches = Vec::new();
  let mut against = SmallVec::<[&String; 7]>::new();

  for plugin in plugins()? {
    against.push(&plugin.name);
    against.extend(plugin.keywords.iter().take(5));
    plugin
      .description
      .as_ref()
      .inspect(|it| against.push(it));

    if words
      .iter()
      .all(|word| check_word(word, against.as_slice()))
    {
      matches.push(plugin.name.clone());
    }

    against.clear();
  }

  Ok(matches)
}

fn check_word(word: &str, against: &[&String]) -> bool {
  against.iter().any(|it| is_match(word, it))
}

fn is_match(word: &str, against: &str) -> bool {
  against
    .split_whitespace()
    .any(|it| it.to_lowercase().contains(word))
}
