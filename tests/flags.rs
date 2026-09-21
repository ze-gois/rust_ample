pub struct Origin;

ample::trait_implement_primitive_numeric_bytes!(u8);

ample::flags!(
    u8;
    "Example flags";
    pub struct ExampleFlags {
        [0b0001; READ;  EXAMPLE_READ;  "EXAMPLE_READ";  "Read"],
        [0b0010; WRITE; EXAMPLE_WRITE; "EXAMPLE_WRITE"; "Write"]
    }
);

#[test]
fn empty_is_contained_by_every_flag_set() {
    let combined = ExampleFlags::READ | ExampleFlags::WRITE;
    assert!(combined.contains(ExampleFlags::default()));
}

#[test]
fn known_combinations_are_not_unknown() {
    let combined = ExampleFlags::READ | ExampleFlags::WRITE;

    assert!(combined.is_known());
    assert!(!combined.has_unknown_bits());
    assert_eq!(combined.unknown_bits(), 0);
    assert_eq!(combined.acronym(), "combined");
}

#[test]
fn unknown_bits_are_preserved() {
    let flags = ExampleFlags::from_bits(0b1001);

    assert_eq!(flags.bits(), 0b1001);
    assert_eq!(flags.known_bits(), ExampleFlags::READ.bits());
    assert_eq!(flags.unknown_bits(), 0b1000);
    assert!(flags.has_unknown_bits());
    assert!(!flags.is_known());
    assert_eq!(flags.acronym(), "unknown");
}
