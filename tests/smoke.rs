use adbimport::{parse_astrodatabank_export_file, write_gedcom7_file};

#[test]
fn smoke_parse_and_emit_gedcom() {
    let export = parse_astrodatabank_export_file("assets/c_sample.xml").unwrap();

    let tmp = std::env::temp_dir().join("adbimport-smoke.ged");
    write_gedcom7_file(&export, &tmp).unwrap();

    let content = std::fs::read_to_string(&tmp).unwrap();

    // Header
    assert!(content.contains("0 HEAD"));
    assert!(content.contains("1 GEDC"));
    assert!(content.contains("2 VERS 7.0"));

    // At least one individual with core tags.
    assert!(content.contains("0 @I1@ INDI"));
    assert!(content.contains("1 NAME "));

    // Birth / place should exist for at least one individual in the sample.
    assert!(content.contains("1 BIRT"));
    assert!(content.contains("2 PLAC "));

    // Birth time should be preserved as a custom tag under BIRT.
    assert!(content.contains("2 _OURANIA_TIME "));

    // Source record
    assert!(content.contains("0 @S1@ SOUR"));

    // Trailer
    assert!(content.contains("0 TRLR"));
}
