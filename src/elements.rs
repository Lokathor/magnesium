use super::*;

/// An element within an XML structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XmlElement<'s> {
  /// An opening tag with a name and some attributes.
  ///
  /// If the XML is well formed, then there will be an EndTag with a matching
  /// name later on. In between there can be any number of sub-entries.
  StartTag {
    /// Name of this tag.
    name: &'s str,
    /// Attribute string.
    ///
    /// Parse this with a
    /// [`TagAttributeIterator`](crate::TagAttributeIterator).
    attrs: &'s str,
  },

  /// Closes the `StartTag` of the same name.
  EndTag {
    /// Name of the tag being closed.
    name: &'s str,
  },

  /// An "empty" tag has no inner content, it opens and immediate closes.
  EmptyTag {
    /// The tag's name.
    name: &'s str,
    /// The tag's attribute string.
    ///
    /// Parse this with a
    /// [`TagAttributeIterator`](crate::TagAttributeIterator).
    attrs: &'s str,
  },

  /// Text between tags, including Cdata entries.
  Text(&'s str),

  /// Comments between tags in `<!--` and `-->`.
  Comment(&'s str),
}

/// An iterator to walk the elements of some XML data.
///
/// This gives you basically all the element entries. It's highly likely that
/// you want to chain a call to [`filter_map`](core::iter::Iterator::filter_map)
/// onto this iterator for simplified processing. The crate provides the
/// [`skip_empty_text_elements`] and [`skip_comments`] filters for you.
///
/// The parsing is a little simplistic, and if the iterator gets confused by bad
/// input it will just end the iteration.
#[derive(Debug, Clone, Default)]
pub struct ElementIterator<'s> {
  // Note: this should *initially* be trimmed to the start of the top level XML
  // tag. From there, any leading whitespace we see is part of a Text element.
  text: &'s str,
}
impl<'s> ElementIterator<'s> {
  /// Makes a new iterator over the elements.
  ///
  /// This works both with and without the initial XML declaration in the
  /// string. The declaration won't be in the iteration either way.
  ///
  /// ## Panics
  /// * This calls [`try_new`](ElementIterator::try_new) and then unwraps.
  #[inline]
  #[must_use]
  pub fn new(words: &'s str) -> Self {
    Self::try_new(words).unwrap()
  }

  /// Makes a new iterator over the elements.
  ///
  /// This works both with and without the initial XML declaration in the
  /// string. The declaration won't be in the iteration either way.
  ///
  /// ## Failure
  /// * If the text has an XML declaration that opens but doesn't close, this
  ///   fails.
  #[inline]
  #[must_use]
  pub fn try_new(text: &'s str) -> Option<Self> {
    let text = trim_xml_declaration(text)?;
    Some(Self { text })
  }
}
impl<'s> Iterator for ElementIterator<'s> {
  type Item = XmlElement<'s>;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<Self::Item> {
    'clear_and_return_none: loop {
      if self.text.is_empty() {
        return None;
      } else if self.text.starts_with("<!CDATA[") {
        let (cdata, rest) = match break_on_first_str(self.text, "]]>") {
          Some((cdata, rest)) => (&cdata[8..], rest),
          None => break 'clear_and_return_none,
        };
        self.text = rest;
        return Some(XmlElement::Text(cdata));
      } else if self.text.starts_with("<!--") {
        let (comment, rest) = match break_on_first_str(self.text, "-->") {
          Some((comment, rest)) => (&comment[4..], rest),
          None => break 'clear_and_return_none,
        };
        self.text = rest;
        return Some(XmlElement::Comment(comment));
      } else if self.text.starts_with('<') {
        let (tag_text, rest) = match break_on_first_char(self.text, '>') {
          Some((tag_text, rest)) => (&tag_text[1..], rest),
          None => break 'clear_and_return_none,
        };
        self.text = rest;
        if tag_text.ends_with('/') {
          let (name, attrs) =
            break_on_first_char(tag_text, ' ').unwrap_or((tag_text, "/"));
          let attrs = &attrs[..attrs.len() - 1];
          return Some(XmlElement::EmptyTag { name, attrs });
        } else if tag_text.starts_with('/') {
          return Some(XmlElement::EndTag { name: &tag_text[1..] });
        } else {
          let (name, attrs) =
            break_on_first_char(tag_text, ' ').unwrap_or((tag_text, ""));
          return Some(XmlElement::StartTag { name, attrs });
        }
      } else {
        let text_end_byte = self.text.find('<').unwrap_or(self.text.len());
        let (here, rest) = self.text.split_at(text_end_byte);
        self.text = rest;
        return Some(XmlElement::Text(here));
      }
    }
    self.text = "";
    None
  }
}
impl<'s> core::iter::FusedIterator for ElementIterator<'s> {}

/// Filters out `XmlElement::Text(t)` when `t` is only whitespace.
///
/// If `t` is more than just whitespace it is unaffected.
///
/// For use with [`filter_map`](core::iter::Iterator::filter_map) calls on
/// an [`ElementIterator`].
///
/// ## Failure
/// * If the input is `XmlElement::Text` and the contained text becomes an empty
///   string after calling [`trim`](str::trim).
#[inline]
#[must_use]
pub fn skip_empty_text_elements<'s>(
  el: XmlElement<'s>,
) -> Option<XmlElement<'s>> {
  match el {
    XmlElement::Text(t) => {
      if t.trim().is_empty() {
        None
      } else {
        Some(XmlElement::Text(t))
      }
    }
    other => Some(other),
  }
}

/// Filters out `XmlElement::Comment(_)`.
///
/// For use with [`filter_map`](core::iter::Iterator::filter_map) calls on
/// an [`ElementIterator`].
///
/// ## Failure
/// * If the input is `XmlElement::Comment`.
#[inline]
#[must_use]
pub fn skip_comments<'s>(el: XmlElement<'s>) -> Option<XmlElement<'s>> {
  match el {
    XmlElement::Comment(_) => None,
    other => Some(other),
  }
}

/// Remove the XML declaration (and leading whitespace), if any.
/// ## Failure
/// * If the declaration opens but _doesn't_ close, this fails.
fn trim_xml_declaration(mut text: &str) -> Option<&str> {
  text = text.trim();
  if text.starts_with("<?xml") {
    break_on_first_str(text.trim_start(), "?>")
      .map(|(_decl, rest)| rest.trim_start())
  } else {
    Some(text)
  }
}

#[test]
fn test_trim_xml_declaration() {
  assert_eq!(trim_xml_declaration(""), Some(""));

  assert_eq!(trim_xml_declaration(" "), Some(""));

  assert_eq!(trim_xml_declaration("<?xml"), None);

  let a = r#"<?xml ?>"#;
  assert_eq!(trim_xml_declaration(a), Some(""));

  let b = r#"<?xml
    version="version_number" ?>"#;
  assert_eq!(trim_xml_declaration(b), Some(""));

  let c = r#"<?xml
    version="version_number"
    encoding="encoding_declaration" ?>"#;
  assert_eq!(trim_xml_declaration(c), Some(""));

  let d = r#"<?xml
    version="version_number"
    encoding="encoding_declaration"
    standalone="standalone_status" ?>"#;
  assert_eq!(trim_xml_declaration(d), Some(""));

  let graphics = r#"<?xml version="1.0" encoding="UTF-8"?>
    <registry>"#;
  assert_eq!(trim_xml_declaration(graphics), Some("<registry>"));
}
