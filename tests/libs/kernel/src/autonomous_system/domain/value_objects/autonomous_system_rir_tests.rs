use kernel::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;

fn rir(value: &str) -> AutonomousSystemRir {
    AutonomousSystemRir::new(value).expect("valid registry")
}

#[test]
fn it_accepts_the_five_registries() {
    assert_eq!(rir("AFRINIC"), AutonomousSystemRir::Afrinic);
    assert_eq!(rir("APNIC"), AutonomousSystemRir::Apnic);
    assert_eq!(rir("ARIN"), AutonomousSystemRir::Arin);
    assert_eq!(rir("LACNIC"), AutonomousSystemRir::Lacnic);
    assert_eq!(rir("RIPE NCC"), AutonomousSystemRir::RipeNcc);
}

#[test]
fn it_ignores_case_whitespace_and_separators() {
    for spelling in [
        "ripe ncc",
        "RIPE-NCC",
        "ripe_ncc",
        "RIPENCC",
        "RIPE",
        " Ripe NCC ",
    ] {
        assert_eq!(rir(spelling), AutonomousSystemRir::RipeNcc, "{spelling}");
    }
    assert_eq!(rir("arin"), AutonomousSystemRir::Arin);
}

#[test]
fn it_writes_each_registry_as_the_registry_does() {
    assert_eq!(AutonomousSystemRir::RipeNcc.to_string(), "RIPE NCC");
    assert_eq!(AutonomousSystemRir::Afrinic.as_str(), "AFRINIC");
}

#[test]
fn it_rejects_anything_else() {
    for value in ["", "IANA", "JPNIC", "ARIN2", "RIPE NCC Amsterdam"] {
        let error = AutonomousSystemRir::new(value).expect_err("not a registry");
        assert!(error.message().contains("must be one of"), "{value}");
    }
}
