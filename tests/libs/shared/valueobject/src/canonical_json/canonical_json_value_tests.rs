use shared_valueobject::domain::canonical_json::canonical_json_value::{
    CanonicalJsonValue, MAX_SAFE_INTEGER,
};

#[test]
fn it_serialises_an_object_without_whitespace() {
    let value =
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string("89.19.209.154"))]);

    assert_eq!(value.to_canonical_string(), r#"{"value":"89.19.209.154"}"#);
}

#[test]
fn it_sorts_members_by_key() {
    let value = CanonicalJsonValue::object([
        ("subject", CanonicalJsonValue::string("Hello")),
        ("body", CanonicalJsonValue::string("World")),
    ]);

    assert_eq!(
        value.to_canonical_string(),
        r#"{"body":"World","subject":"Hello"}"#
    );
}

#[test]
fn it_sorts_members_by_utf16_code_units_as_rfc_8785_requires() {
    // RFC 8785 section 3.2.3: U+1F600 is encoded as the surrogate pair
    // D83D DE00, so it sorts before U+FB33 even though its UTF-8 bytes do not.
    let value = CanonicalJsonValue::object([
        ("\u{20ac}", CanonicalJsonValue::string("Euro Sign")),
        ("\r", CanonicalJsonValue::string("Carriage Return")),
        (
            "\u{fb33}",
            CanonicalJsonValue::string("Hebrew Letter Dalet With Dagesh"),
        ),
        ("1", CanonicalJsonValue::string("One")),
        (
            "\u{1f600}",
            CanonicalJsonValue::string("Emoji: Grinning Face"),
        ),
        ("\u{80}", CanonicalJsonValue::string("Control")),
        (
            "\u{f6}",
            CanonicalJsonValue::string("Latin Small Letter O With Diaeresis"),
        ),
    ]);

    let expected = "{\"\\r\":\"Carriage Return\",\"1\":\"One\",\"\u{80}\":\"Control\",\
                    \"\u{f6}\":\"Latin Small Letter O With Diaeresis\",\"\u{20ac}\":\"Euro Sign\",\
                    \"\u{1f600}\":\"Emoji: Grinning Face\",\
                    \"\u{fb33}\":\"Hebrew Letter Dalet With Dagesh\"}";
    assert_eq!(value.to_canonical_string(), expected);
}

#[test]
fn it_escapes_strings_like_ecmascript_json_stringify() {
    // RFC 8785 section 3.2.2.2 example: only quote, backslash and control
    // characters are escaped, short forms first, lowercase hex otherwise.
    let value = CanonicalJsonValue::string("\u{20ac}$\u{f}\nA'B\"\\\\\"/");

    assert_eq!(
        value.to_canonical_string(),
        "\"\u{20ac}$\\u000f\\nA'B\\\"\\\\\\\\\\\"/\""
    );
}

#[test]
fn it_uses_the_short_escapes_for_common_control_characters() {
    let value = CanonicalJsonValue::string("\u{8}\u{c}\n\r\t");

    assert_eq!(value.to_canonical_string(), r#""\b\f\n\r\t""#);
}

#[test]
fn it_keeps_non_ascii_characters_literal() {
    let value = CanonicalJsonValue::string("fa\u{df}.de");

    assert_eq!(value.to_canonical_string(), "\"fa\u{df}.de\"");
}

#[test]
fn it_keeps_the_order_of_array_items() {
    let value = CanonicalJsonValue::array([
        CanonicalJsonValue::string("tcp"),
        CanonicalJsonValue::string("http"),
    ]);

    assert_eq!(value.to_canonical_string(), r#"["tcp","http"]"#);
}

#[test]
fn it_serialises_nested_objects_and_arrays() {
    let value = CanonicalJsonValue::object([(
        "values",
        CanonicalJsonValue::array([
            CanonicalJsonValue::object([
                ("name", CanonicalJsonValue::string("Foo")),
                ("data", CanonicalJsonValue::string("qwerty")),
            ]),
            CanonicalJsonValue::object([
                ("name", CanonicalJsonValue::string("Bar")),
                ("data", CanonicalJsonValue::string("42")),
            ]),
        ]),
    )]);

    assert_eq!(
        value.to_canonical_string(),
        r#"{"values":[{"data":"qwerty","name":"Foo"},{"data":"42","name":"Bar"}]}"#
    );
}

#[test]
fn it_serialises_empty_containers() {
    let empty_object = CanonicalJsonValue::object(Vec::<(String, CanonicalJsonValue)>::new());
    let empty_array = CanonicalJsonValue::array([]);

    assert_eq!(empty_object.to_canonical_string(), "{}");
    assert_eq!(empty_array.to_canonical_string(), "[]");
}

#[test]
fn it_serialises_integers_and_booleans() {
    let value = CanonicalJsonValue::object([
        (
            "number",
            CanonicalJsonValue::integer(15139).expect("valid integer"),
        ),
        (
            "negative",
            CanonicalJsonValue::integer(-1).expect("valid integer"),
        ),
        ("is_active", CanonicalJsonValue::boolean(true)),
        ("is_blocking", CanonicalJsonValue::boolean(false)),
    ]);

    assert_eq!(
        value.to_canonical_string(),
        r#"{"is_active":true,"is_blocking":false,"negative":-1,"number":15139}"#
    );
}

#[test]
fn it_keeps_the_last_value_of_a_repeated_key() {
    let value = CanonicalJsonValue::object([
        ("value", CanonicalJsonValue::string("first")),
        ("value", CanonicalJsonValue::string("second")),
    ]);

    assert_eq!(value.to_canonical_string(), r#"{"value":"second"}"#);
}

#[test]
fn it_accepts_the_largest_safe_integers() {
    let largest = CanonicalJsonValue::integer(MAX_SAFE_INTEGER).expect("the limit is accepted");
    let smallest = CanonicalJsonValue::integer(-MAX_SAFE_INTEGER).expect("the limit is accepted");

    assert_eq!(largest.to_canonical_string(), "9007199254740991");
    assert_eq!(smallest.to_canonical_string(), "-9007199254740991");
}

#[test]
fn it_rejects_integers_beyond_the_safe_range() {
    let too_large = CanonicalJsonValue::integer(MAX_SAFE_INTEGER + 1)
        .expect_err("one over the limit must be rejected");
    let too_small =
        CanonicalJsonValue::integer(i64::MIN).expect_err("the minimum i64 must be rejected");

    assert!(too_large.message().contains("must be within"));
    assert!(too_small.message().contains("must be within"));
}

#[test]
fn it_compares_equal_for_the_same_content() {
    let left = CanonicalJsonValue::object([
        ("a", CanonicalJsonValue::string("1")),
        ("b", CanonicalJsonValue::string("2")),
    ]);
    let right = CanonicalJsonValue::object([
        ("b", CanonicalJsonValue::string("2")),
        ("a", CanonicalJsonValue::string("1")),
    ]);

    assert_eq!(left, right);
}
