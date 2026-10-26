use kernel::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

fn canonical(value: &str) -> String {
    MacAddressValue::new(value)
        .expect("valid address")
        .to_string()
}

fn rejection(value: &str) -> String {
    MacAddressValue::new(value)
        .expect_err("the address must be rejected")
        .message()
        .to_string()
}

#[test]
fn it_keeps_a_canonical_address() {
    assert_eq!(canonical("d2:fb:49:24:37:18"), "d2:fb:49:24:37:18");
}

#[test]
fn it_lowercases_the_digits() {
    assert_eq!(canonical("D2:FB:49:24:37:18"), "d2:fb:49:24:37:18");
}

#[test]
fn it_accepts_hyphens() {
    assert_eq!(canonical("D2-FB-49-24-37-18"), "d2:fb:49:24:37:18");
}

#[test]
fn it_restores_dropped_leading_zeros() {
    assert_eq!(canonical("0:1b:63:84:45:e6"), "00:1b:63:84:45:e6");
    assert_eq!(canonical("2-0-5E-10-0-0"), "02:00:5e:10:00:00");
}

#[test]
fn it_accepts_the_dotted_notation() {
    assert_eq!(canonical("001B.6384.45E6"), "00:1b:63:84:45:e6");
}

#[test]
fn it_accepts_twelve_digits_without_separators() {
    assert_eq!(canonical("02005E100000"), "02:00:5e:10:00:00");
}

#[test]
fn it_trims_surrounding_whitespace() {
    assert_eq!(canonical("  d2:fb:49:24:37:18 \n"), "d2:fb:49:24:37:18");
}

#[test]
fn it_compares_equal_for_every_spelling_of_the_same_address() {
    let spellings = [
        "d2:fb:49:24:37:18",
        "D2-FB-49-24-37-18",
        "d2fb.4924.3718",
        "D2FB49243718",
    ];
    for spelling in spellings {
        assert_eq!(
            MacAddressValue::new(spelling).expect("valid address"),
            MacAddressValue::new(" d2:fb:49:24:37:18 ").expect("valid address"),
            "{spelling}"
        );
    }
}

#[test]
fn it_accepts_the_lowest_and_highest_addresses() {
    assert_eq!(canonical("00:00:00:00:00:00"), "00:00:00:00:00:00");
    assert_eq!(canonical("FF:FF:FF:FF:FF:FF"), "ff:ff:ff:ff:ff:ff");
}

#[test]
fn it_rejects_too_few_or_too_many_octets() {
    assert!(rejection("d2:fb:49:24:37").contains("six octets"));
    assert!(rejection("d2:fb:49:24:37:18:00").contains("six octets"));
}

#[test]
fn it_rejects_an_eui64_identifier() {
    assert!(rejection("d2:fb:49:ff:fe:24:37:18").contains("EUI-64"));
}

#[test]
fn it_rejects_octets_of_more_than_two_digits() {
    assert!(rejection("d2:fb:049:24:37:18").contains("one or two hexadecimal digits"));
}

#[test]
fn it_rejects_empty_octets() {
    assert!(rejection("d2::49:24:37:18").contains("one or two hexadecimal digits"));
}

#[test]
fn it_rejects_non_hexadecimal_digits() {
    assert!(rejection("d2:fb:49:24:37:1g").contains("one or two hexadecimal digits"));
}

#[test]
fn it_rejects_mixed_separators() {
    assert!(rejection("d2:fb-49:24-37:18").contains("six octets"));
    assert!(rejection("d2:fb:49:24:37-18").contains("six octets"));
}

#[test]
fn it_rejects_a_malformed_dotted_notation() {
    assert!(rejection("d2fb.4924").contains("three groups"));
    assert!(rejection("d2f.b4924.3718").contains("four hexadecimal digits"));
}

#[test]
fn it_rejects_a_wrong_number_of_digits_without_separators() {
    assert!(rejection("d2fb4924371").contains("twelve hexadecimal digits"));
    assert!(rejection("d2fb492437180").contains("twelve hexadecimal digits"));
    assert!(rejection("d2fb4924371g").contains("twelve hexadecimal digits"));
}

#[test]
fn it_rejects_non_ascii_digits() {
    assert!(rejection("d2:fb:49:24:37:１８").contains("one or two hexadecimal digits"));
}

#[test]
fn it_rejects_an_ipv4_address() {
    assert!(rejection("192.0.2.1").contains("three groups"));
}

#[test]
fn it_rejects_an_empty_value() {
    assert!(rejection("  ").contains("twelve hexadecimal digits"));
}

#[test]
fn it_exposes_its_stix_contributing_properties() {
    let value = MacAddressValue::new("D2-FB-49-24-37-18").expect("valid address");

    assert_eq!(
        value.contributing_properties().to_canonical_string(),
        r#"{"value":"d2:fb:49:24:37:18"}"#
    );
}

#[test]
fn it_exposes_its_octets_and_oui() {
    let value = MacAddressValue::new("00:1b:63:84:45:e6").expect("valid address");

    assert_eq!(value.octets(), [0x00, 0x1b, 0x63, 0x84, 0x45, 0xe6]);
    assert_eq!(value.oui(), "00:1b:63");
}

#[test]
fn it_reads_the_group_and_local_bits() {
    let address = |value: &str| MacAddressValue::new(value).expect("valid address");

    assert!(address("01:00:5e:00:00:01").is_multicast());
    assert!(!address("00:1b:63:84:45:e6").is_multicast());
    assert!(address("02:00:5e:10:00:00").is_locally_administered());
    assert!(!address("00:1b:63:84:45:e6").is_locally_administered());
    assert!(address("ff:ff:ff:ff:ff:ff").is_broadcast());
    assert!(address("ff:ff:ff:ff:ff:ff").is_multicast());
    assert!(!address("ff:ff:ff:ff:ff:fe").is_broadcast());
}
