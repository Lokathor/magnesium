use magnesium::*;

#[test]
fn test_parsing() {
  let xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <registry>
      <!-- We're gonna pretend that there's a whole file here -->
      <types>
        <type>typedef unsigned int <name>GraphicsEnum</name>;</type>
      </types>
      <enums group="GraphicPolygons">
        <enum name="GRAPHIC_POINTS" value="0x0000" />
        <enum name="GRAPHIC_LINES" value="0x0001" />
      </enums>
    </registry>
  "#;

  let mut iter = ElementIterator::new(xml);

  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "registry", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n      ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::Comment(
      " We're gonna pretend that there's a whole file here "
    ))
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n      ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "types", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n        ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "type", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("typedef unsigned int ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "name", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("GraphicsEnum")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "name" }));
  assert_eq!(iter.next(), Some(XmlElement::Text(";")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "type" }));
  assert_eq!(iter.next(), Some(XmlElement::Text("\n      ")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "types" }));
  assert_eq!(iter.next(), Some(XmlElement::Text("\n      ")));
  //
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag {
      name: "enums",
      attrs: r#"group="GraphicPolygons""#
    })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n        ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_POINTS" value="0x0000" "#
    })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n        ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_LINES" value="0x0001" "#
    })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("\n      ")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "enums" }));
  assert_eq!(iter.next(), Some(XmlElement::Text("\n    ")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "registry" }));
  assert_eq!(iter.next(), None);
}

#[test]
fn test_whitespace_skipping_parsing() {
  let xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <registry>
      <!-- We're gonna pretend that there's a whole file here -->
      <types>
        <type>typedef unsigned int <name>GraphicsEnum</name>;</type>
      </types>
      <enums group="GraphicPolygons">
        <enum name="GRAPHIC_POINTS" value="0x0000" />
        <enum name="GRAPHIC_LINES" value="0x0001" />
      </enums>
    </registry>
  "#;

  let mut iter = ElementIterator::new(xml).filter_map(skip_empty_text_elements);

  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "registry", attrs: "" })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::Comment(
      " We're gonna pretend that there's a whole file here "
    ))
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "types", attrs: "" })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "type", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("typedef unsigned int ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "name", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("GraphicsEnum")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "name" }));
  assert_eq!(iter.next(), Some(XmlElement::Text(";")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "type" }));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "types" }));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag {
      name: "enums",
      attrs: r#"group="GraphicPolygons""#
    })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_POINTS" value="0x0000" "#
    })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_LINES" value="0x0001" "#
    })
  );
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "enums" }));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "registry" }));
  assert_eq!(iter.next(), None);
}

#[test]
fn test_skipping_comments_parsing() {
  let xml = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <registry>
      <!-- We're gonna pretend that there's a whole file here -->
      <types>
        <type>typedef unsigned int <name>GraphicsEnum</name>;</type>
      </types>
      <enums group="GraphicPolygons">
        <enum name="GRAPHIC_POINTS" value="0x0000" />
        <enum name="GRAPHIC_LINES" value="0x0001" />
      </enums>
    </registry>
  "#;

  let mut iter = ElementIterator::new(xml)
    .filter_map(skip_empty_text_elements)
    .filter_map(skip_comments);

  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "registry", attrs: "" })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "types", attrs: "" })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "type", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("typedef unsigned int ")));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag { name: "name", attrs: "" })
  );
  assert_eq!(iter.next(), Some(XmlElement::Text("GraphicsEnum")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "name" }));
  assert_eq!(iter.next(), Some(XmlElement::Text(";")));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "type" }));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "types" }));
  assert_eq!(
    iter.next(),
    Some(XmlElement::StartTag {
      name: "enums",
      attrs: r#"group="GraphicPolygons""#
    })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_POINTS" value="0x0000" "#
    })
  );
  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag {
      name: "enum",
      attrs: r#"name="GRAPHIC_LINES" value="0x0001" "#
    })
  );
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "enums" }));
  assert_eq!(iter.next(), Some(XmlElement::EndTag { name: "registry" }));
  assert_eq!(iter.next(), None);
}

#[test]
fn test_empty_tag_no_attrs() {
  let xml = "<apientry/>";

  let mut iter = ElementIterator::new(xml);

  assert_eq!(
    iter.next(),
    Some(XmlElement::EmptyTag { name: "apientry", attrs: "" })
  );
}
