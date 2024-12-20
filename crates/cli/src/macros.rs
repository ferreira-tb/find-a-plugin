#[macro_export]
macro_rules! return_if_ne {
  ($ord:expr) => {
    let ord = $ord;
    if ord != std::cmp::Ordering::Equal {
      return ord;
    }
  };
}
