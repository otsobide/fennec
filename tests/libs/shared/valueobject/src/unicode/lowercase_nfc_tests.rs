use shared_valueobject::domain::unicode::lowercase_nfc::lowercase_nfc;

#[test]
fn it_lowercases_ascii() {
    assert_eq!(lowercase_nfc("John.Smith"), "john.smith");
}

#[test]
fn it_lowercases_non_ascii_letters() {
    assert_eq!(lowercase_nfc("ÉLODIE"), "élodie");
    assert_eq!(lowercase_nfc("ΣΟΦΙΑ"), "σοφια");
}

#[test]
fn it_composes_decomposed_sequences() {
    assert_eq!(lowercase_nfc("jose\u{301}"), "jos\u{e9}");
}

#[test]
fn it_gives_one_string_for_every_equivalent_spelling() {
    assert_eq!(lowercase_nfc("JOS\u{c9}"), lowercase_nfc("jose\u{301}"));
}

#[test]
fn it_keeps_a_lowercase_mapping_that_has_no_composed_form() {
    assert_eq!(lowercase_nfc("\u{130}"), "i\u{307}");
}

#[test]
fn it_leaves_digits_and_symbols_alone() {
    assert_eq!(lowercase_nfc("a+b_c-1!#$%"), "a+b_c-1!#$%");
}
