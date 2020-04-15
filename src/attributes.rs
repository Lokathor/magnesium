use super::*;

/// The output of a [`TagAttributeIterator`].
///
/// Attributes within an XML tag are key-value pairs. Only `Start` and `Empty`
/// tags have attributes.
///
/// Each key is expected to only appear once in a given tag. The order of the
/// keys is not usually significant.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct TagAttribute<'s> {
  pub key: &'s str,
  pub value: &'s str,
}

/// Iterator to walk through a `Start` or `Empty` tag's attribute string.
///
/// Supports both `'` and `"` quoting around the attribute values.
///
/// The parsing is a little simplistic, and if the iterator gets confused by bad
/// input it will just end the iteration.
#[derive(Debug, Clone, Default)]
pub struct TagAttributeIterator<'s> {
  attrs: &'s str,
}
impl<'s> TagAttributeIterator<'s> {
  /// Makes a new iterator over the attribute string.
  pub fn new(attrs: &'s str) -> Self {
    Self { attrs: attrs.trim() }
  }
}
impl<'s> Iterator for TagAttributeIterator<'s> {
  type Item = TagAttribute<'s>;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    debug_assert_eq!(self.attrs, self.attrs.trim());
    if self.attrs.is_empty() {
      return None;
    }
    'clear_and_return_none: loop {
      // break on `=`
      let (key, rest) = match break_on_first_char(self.attrs, '=') {
        Some((key, rest)) => (key, rest),
        None => break 'clear_and_return_none,
      };
      self.attrs = rest;
      // support both `"` and `'` since it's easy to do
      let quote_marker = match self.attrs.chars().nth(0) {
        Some(q) if q == '\'' || q == '\"' => {
          self.attrs = &self.attrs[1..];
          q
        }
        _ => break 'clear_and_return_none,
      };
      // break on the end of the quote
      let (value, rest) = match break_on_first_char(self.attrs, quote_marker) {
        Some((key, rest)) => (key, rest),
        None => break 'clear_and_return_none,
      };
      self.attrs = rest.trim_start();
      return Some(TagAttribute { key, value });
    }
    self.attrs = "";
    None
  }
}
impl<'s> core::iter::FusedIterator for TagAttributeIterator<'s> {}
