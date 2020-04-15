use magnesium::*;

type Tais = TagAttributeIterator<'static>;

#[test]
fn empty_string_or_whitespace_gives_none() {
  assert!(Tais::new("").next().is_none());

  assert!(Tais::new("   ").next().is_none());

  assert!(Tais::new("\r\n").next().is_none());

  assert!(Tais::new("\t").next().is_none());
}

#[test]
fn test_parsing() {
  let mut iter = Tais::new(r#"namespace="Graphics" group="Polygon""#);
  assert_eq!(
    iter.next(),
    Some(TagAttribute { key: "namespace", value: "Graphics" })
  );
  assert_eq!(
    iter.next(),
    Some(TagAttribute { key: "group", value: "Polygon" })
  );
  assert_eq!(iter.next(), None);

  // extra space around doesn't affect it
  let mut iter = Tais::new(r#" namespace="Graphics" group="Polygon" "#);
  assert_eq!(
    iter.next(),
    Some(TagAttribute { key: "namespace", value: "Graphics" })
  );
  assert_eq!(
    iter.next(),
    Some(TagAttribute { key: "group", value: "Polygon" })
  );
  assert_eq!(iter.next(), None);
}
