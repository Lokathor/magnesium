#![no_std]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! This crate provides very a simplistic iterator for stepping over XML data.
//!
//! The processing is quite simplistic. The full XML spec probably isn't
//! followed. It doesn't decode any of the special character replacements for
//! you.
//!
//! The crate is intended for when you have a fairly basic XML file and you just
//! need to walk through and scrape the data. For example, when parsing
//! [`gl.xml`](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml)
//! or
//! [`vk.xml`](https://github.com/KhronosGroup/Vulkan-Docs/blob/master/xml/vk.xml).
//!
//! Only requires `core`, doesn't allocate.
//!
//! ## Example Usage
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
