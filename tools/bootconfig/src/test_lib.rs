use super::*;

#[test]
fn binary_trailer_is_required_and_checked() {
    // Padded "kernel.x=1", size 12, byte-sum checksum 917, and magic.
    let data = b"kernel.x=1\0\0\x0c\0\0\0\x95\x03\0\0#BOOTCONFIG\n";
    assert_eq!(parse(data).unwrap().kernel[0].value.as_deref(), Some("1"));
    assert!(parse(&data[..BOOTCONFIG_TRAILER_SIZE - 1]).is_err());
    let mut corrupt = *data;
    corrupt[0] = b'K';
    assert!(parse(&corrupt).is_err());
}

#[test]
fn text_preserves_tree_order_and_replaces_duplicate_values() {
    let config = parse_text("kernel.z.x=first\nkernel.a=middle\nkernel.z.y=last").unwrap();
    assert_eq!(
        format_params(&config.kernel).unwrap(),
        "z.x=first z.y=last a=middle "
    );
    let config = parse_text("kernel.x=a\nkernel.x=b,c").unwrap();
    assert_eq!(format_params(&config.kernel).unwrap(), "x=b x=c ");
}

#[test]
fn grammar_handles_quotes_comments_and_nested_blocks() {
    for prefix in [
        "kernel.x=\"a\\\"b\"\n",
        "kernel.x=\"a\\\\\"\n",
        "kernel.x='a\\'\n",
        "# comment\r",
    ] {
        let nested = format!("{prefix}{}{}", "a {".repeat(65), "}".repeat(65));
        assert!(parse_text(&nested).is_ok(), "{prefix:?}");
    }
}

#[test]
fn flag_with_children_matches_linux_namespace_handling() {
    let config = parse_text("kernel.debug\nkernel.debug.level=1").unwrap();
    assert_eq!(format_params(&config.kernel).unwrap(), "debug.level=1 ");
}

#[test]
fn text_parser_leaves_size_and_character_policy_to_callers() {
    let key = "x".repeat(256);
    let value = "é".repeat(17000);
    let config = parse_text(&format!("kernel.{key}=\"{value}\"")).unwrap();
    assert_eq!(config.kernel[0].name, key);
    assert_eq!(config.kernel[0].value.as_deref(), Some(value.as_str()));
}

#[test]
fn syntax_error_keeps_the_parser_diagnostic() {
    let error = parse_text("kernel.x=\"unterminated").unwrap_err();
    assert!(matches!(error, Error::BootconfigParse { .. }));
    assert!(error.to_string().contains("kernel.x"));
}

#[test]
fn combines_package_blacklists_into_one_kernel_argument() {
    let input = concat!(
        "kernel.module_blacklist = i8042\n",
        "kernel.fips = 1\n",
        "kernel.module_blacklist = binfmt_misc\n",
        "kernel.module_blacklist = \"i8042,binfmt_misc\"\n",
        "kernel.sysctl.kernel.kexec_load_disabled = 1\n",
        "kernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
        "init.systemd.unit = fipscheck.target\n",
    );
    let merged = merge_blacklisted_modules(input).unwrap();
    let config = parse_text(&merged).unwrap();
    assert_eq!(
        config.uki_cmdline("root=/dev/dm-0", "").unwrap(),
        concat!(
            "fips=1 ",
            "sysctl.kernel.kexec_load_disabled=1 ",
            "systemd.mask=proc-sys-fs-binfmt_misc.mount ",
            "module_blacklist=binfmt_misc,i8042 ",
            "root=/dev/dm-0 -- systemd.unit=fipscheck.target"
        )
    );
    assert_eq!(merge_blacklisted_modules(&merged).unwrap(), merged);
}

#[test]
fn preserves_other_arrays_and_namespaces() {
    let input = concat!(
        "kernel.console = tty0, ttyS0\n",
        "kernel.module_blacklist = i8042, binfmt_misc, i8042\n",
        "init.module_blacklist = userspace\n",
    );
    let config = parse_text(&merge_blacklisted_modules(input).unwrap()).unwrap();
    assert_eq!(
        format_params(&config.kernel).unwrap(),
        "console=tty0 console=ttyS0 module_blacklist=binfmt_misc,i8042 "
    );
    assert_eq!(
        format_params(&config.init).unwrap(),
        "module_blacklist=userspace "
    );
}

#[test]
fn merges_kernel_parameter_alias_without_losing_contributions() {
    let merged = merge_blacklisted_modules(concat!(
        "kernel.module-blacklist = i8042\n",
        "kernel.module_blacklist = binfmt_misc\n",
    ))
    .unwrap();
    assert_eq!(merged, "kernel.module_blacklist = \"binfmt_misc,i8042\"\n");
    assert_eq!(
        format_params(&parse_text(&merged).unwrap().kernel).unwrap(),
        "module_blacklist=binfmt_misc,i8042 "
    );
}

#[test]
fn rejects_empty_or_malformed_blacklist_assignments() {
    for input in [
        "kernel.module_blacklist =",
        "kernel.module_blacklist = \"i8042,,binfmt_misc\"",
        "kernel.module_blacklist = \"i8042 binfmt_misc\"",
        "kernel.module_blacklist = ../binfmt_misc",
        "kernel.module_blacklist = i8042; init.extra = value",
        "kernel.module_blacklist = i8042; other.extra = value",
        "kernel.module_blacklist += binfmt_misc",
        "kernel.module_blacklist",
        "kernel { module_blacklist = i8042 }",
        "kernel { module_blacklist = binfmt_misc }\nkernel.module_blacklist = i8042",
    ] {
        assert!(merge_blacklisted_modules(input).is_err(), "{input}");
    }
}
