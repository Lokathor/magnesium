#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! This crate provides very a simplistic iterator for stepping over XML data.
//!
//! Only requires `core`, doesn't allocate, doesn't use `unsafe`.
//!
//! The processing is quite simplistic, and the iterator will simply fail and
//! end the iteration if there's a problem. This doesn't do any special
//! character replacement for you.
//!
//! The crate is intended for when you have a fairly basic XML file that is
//! assumed to be "non-hostile", and you just need to walk through and scrape
//! the data. For example, when parsing
//! [`gl.xml`](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml)
//! or
//! [`vk.xml`](https://github.com/KhronosGroup/Vulkan-Docs/blob/master/xml/vk.xml).
//!
//! ## Example Usage
//!
//! ```
//! use magnesium::*;
//!
//! let xml_string = r#"
//!   <?xml version="1.0" encoding="UTF-8"?>
//!   <!-- just imagine we had a whole file here -->
//!   <registry>
//!     <enums namespace="Graphics" group="Polygon">
//!       <enum value="0" name="GRAPHICS_POINTS"/>
//!       <enum value="1" name="GRAPHICS_LINES"/>
//!     </enums>
//!   </registry>
//! "#;
//!
//! for element in ElementIterator::new(xml_string) {
//!   println!("{:?}", element);
//! }
//! ```

mod elements;
pub use elements::*;

mod attributes;
pub use attributes::*;

#[cfg(feature="alloc")]
extern crate alloc;
#[cfg(feature="alloc")]
use alloc::string::String;

/// Converts an escaped string to the intended text.
///
/// ```rust
/// # use magnesium::revert_xml_encoding;
/// assert_eq!("abc<", &revert_xml_encoding("abc&lt;"));
/// assert_eq!("1>2", &revert_xml_encoding("1&gt;2"));
/// assert_eq!("a&b", &revert_xml_encoding("a&amp;b"));
/// ```
/// ## Panics
/// If an illegal '&' sequence is present.
#[cfg(feature="alloc")]
pub fn revert_xml_encoding(text: &str) -> String {
  let mut out = String::with_capacity(text.as_bytes().len());
  let mut chars = text.chars();
  while let Some(c) = chars.next() {
    if c != '&' {
      out.push(c);
    } else {
      match chars.next().unwrap() {
        'l' => {
          assert_eq!(chars.next().unwrap(), 't');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('<');
        }
        'g' => {
          assert_eq!(chars.next().unwrap(), 't');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('>');
        }
        'a' => {
          assert_eq!(chars.next().unwrap(), 'm');
          assert_eq!(chars.next().unwrap(), 'p');
          assert_eq!(chars.next().unwrap(), ';');
          out.push('&');
        }
        other => panic!("unknown '&' char: {}", other),
      }
    }
  }
  out
}

/// Break the input around the first `c` found.
///
/// Returns `(before, after)`.
///
/// The `c` value isn't in _either_ of the return slices, it's discarded.
fn break_on_first_char(input: &str, c: char) -> Option<(&str, &str)> {
  input.find(c).map(|b| {
    let mut buf = [0_u8; 4];
    let utf8_bytes_this_char = c.encode_utf8(&mut buf).len();
    let (head, tail) = input.split_at(b);
    let tail = &tail[utf8_bytes_this_char..];
    (head, tail)
  })
}

#[test]
fn test_break_on_first_char() {
  assert_eq!(break_on_first_char("", '='), None);
  assert_eq!(break_on_first_char("a", '='), None);
  assert_eq!(break_on_first_char("a=", '='), Some(("a", "")));
  assert_eq!(break_on_first_char("a=b", '='), Some(("a", "b")));
}

/// Break the input around the first `needle` found.
///
/// Returns `(before, after)`.
///
/// The `needle` value isn't in _either_ of the return slices, it's discarded.
fn break_on_first_str<'i>(
  input: &'i str,
  needle: &str,
) -> Option<(&'i str, &'i str)> {
  input.find(needle).map(|b| {
    let (head, tail) = input.split_at(b);
    let tail = &tail[needle.len()..];
    (head, tail)
  })
}

#[test]
fn test_break_on_first_str() {
  assert_eq!(break_on_first_str("", "=="), None);
  assert_eq!(break_on_first_str("a", "=="), None);
  assert_eq!(break_on_first_str("a=", "=="), None);
  assert_eq!(break_on_first_str("a==", "=="), Some(("a", "")));
  assert_eq!(break_on_first_str("a==b", "=="), Some(("a", "b")));
}
