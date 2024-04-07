use std::fmt::{Debug, Formatter};

static VEC_DEBUG_LIMIT: usize = 10;

enum OrMore<T> {
  Value(T),
  More,
}

impl<T: Debug> Debug for OrMore<T> {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      OrMore::Value(val) => Debug::fmt(val, fmt),
      OrMore::More => write!(fmt, "..."),
    }
  }
}

pub fn debug_vec<V: Debug>(val: &Vec<V>, fmt: &mut Formatter<'_>) -> std::fmt::Result {
  if val.len() <= VEC_DEBUG_LIMIT {
    fmt
      .debug_list()
      .entries(val.iter().take(VEC_DEBUG_LIMIT))
      .finish()
  } else {
    fmt
      .debug_list()
      .entries(
        val
          .iter()
          .take(VEC_DEBUG_LIMIT)
          .map(OrMore::Value)
          .chain(vec![OrMore::More]),
      )
      .finish()
  }
}
