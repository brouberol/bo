use crate::{Help, Section};
use std::collections::HashMap;

#[test]
fn test_help_section_format() {
    let help_section = Section {
        title: String::from("Test section title"),
        entries: HashMap::from([("x", "x doc"), ("yy", "yy doc")]),
    };
    let expected_output = r#"[1mTest section title[m
  x  => x doc
  yy => yy doc"#;
    assert_eq!(help_section.format(), expected_output);
}

#[test]
fn test_help_format() {
    let help_section_1 = Section {
        title: String::from("Test section title"),
        entries: HashMap::from([("x", "x doc"), ("yy", "yy doc")]),
    };
    let help_section_2 = Section {
        title: String::from("Other test section title"),
        entries: HashMap::from([("blah", "blah doc"), ("derp", "derp doc")]),
    };
    let help = Help {
        sections: vec![help_section_1, help_section_2],
    };
    let expected_output = r#"[1mTest section title[m
  x  => x doc
  yy => yy doc

[1mOther test section title[m
  blah => blah doc
  derp => derp doc"#;
    assert_eq!(help.format(), expected_output);
}
