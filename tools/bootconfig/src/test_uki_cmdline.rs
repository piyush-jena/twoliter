use super::*;

#[test]
fn uki_arguments_keep_kernel_and_init_namespaces_separate() {
    let config =
        crate::parse_text("kernel { console=tty0,ttyS0; quiet }\ninit.systemd.unit=rescue.target")
            .unwrap();
    assert_eq!(
        config.uki_cmdline("root=/dev/dm-0", "").unwrap(),
        "console=tty0 console=ttyS0 quiet root=/dev/dm-0 -- systemd.unit=rescue.target"
    );
}

#[test]
fn load_options_normalize_whitespace_and_include_utf16_nul() {
    assert_eq!(uki_load_options(" \tA\tB ").unwrap(), b"A\0 \0B\0\0\0");
    assert!(uki_load_options("a\0b").is_err());
}

#[test]
fn section_decoding_accepts_only_trailing_nul_padding() {
    for section in [b"quiet --".as_slice(), b"quiet --\0\0\0".as_slice()] {
        assert_eq!(parse_uki_cmdline(section).unwrap(), "quiet --");
    }
    assert!(matches!(
        parse_uki_cmdline(b"quiet\0extra"),
        Err(crate::Error::UkiDataAfterNul)
    ));
    assert!(matches!(
        parse_uki_cmdline(b"\xff"),
        Err(crate::Error::InvalidUtf8 { .. })
    ));
    assert!(matches!(
        parse_uki_cmdline(b"\0"),
        Err(crate::Error::InvalidUkiSectionLength { .. })
    ));
    assert!(parse_uki_cmdline(&vec![b'x'; 2047]).is_ok());
    assert!(matches!(
        parse_uki_cmdline(&vec![b'x'; 2048]),
        Err(crate::Error::InvalidUkiSectionLength { .. })
    ));
}

#[test]
fn image_builder_arguments_are_used_without_an_extra_validation_pass() {
    let config = BootConfig::default();
    assert_eq!(
        config.uki_cmdline("example=\"open", "").unwrap(),
        "example=\"open --"
    );
    assert!(matches!(
        config.uki_cmdline(&"x".repeat(2048), ""),
        Err(crate::Error::UkiCommandLineTooLong { .. })
    ));
}
