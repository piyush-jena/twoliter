#!/usr/bin/env python3
"""Generate fixed two-bank expectations without invoking pcrsys.

Run with Python 3 and redirect stdout to bank-vectors.json. The PE fixtures match
pe.rs::tests::{build_test_uki, build_test_shim}; the hashing oracle uses hashlib and explicit
Authenticode byte ranges, not the Rust authenticode or prediction libraries.
"""

import hashlib
import json
import struct
import uuid


def pe_image(machine, sections):
    image = bytearray(1024)
    image[:2] = b"MZ"
    image[0x40:0x44] = b"PE\0\0"
    for offset, value in [
        (0x44, machine), (0x46, len(sections)), (0x54, 0xF0),
        (0x56, 0x22), (0x58, 0x20B), (0x9C, 10),
    ]:
        struct.pack_into("<H", image, offset, value)
    struct.pack_into("<Q", image, 0x70, 0x400000)
    for offset, value in [
        (0x3C, 0x40), (0x78, 4096), (0x7C, 512),
        (0x90, (len(sections) + 1) * 4096), (0x94, 1024), (0xC4, 16),
    ]:
        struct.pack_into("<I", image, offset, value)
    for index, (name, data) in enumerate(sections):
        header = 0x148 + 40 * index
        offset = len(image)
        size = (len(data) + 511) // 512 * 512
        image[header:header + len(name)] = name.encode()
        for field, value in [
            (8, len(data)), (12, (index + 1) * 4096), (16, size),
            (20, offset), (36, 0x40000040),
        ]:
            struct.pack_into("<I", image, header + field, value)
        image += data + bytes(size - len(data))
    return bytes(image)


def shim_image():
    """The existing shim fixture, including its COFF string-table overlay."""
    image = bytearray(0xA00)
    image[:2] = b"MZ"
    image[0x40:0x44] = b"PE\0\0"
    for offset, value in [
        (0x44, 0xAA64), (0x46, 3), (0x54, 0xF0),
        (0x56, 0x22), (0x58, 0x20B), (0x9C, 10),
    ]:
        struct.pack_into("<H", image, offset, value)
    image[0x5A] = 1
    for offset, value in [
        (0x3C, 0x40), (0x4C, 0x800), (0x78, 0x1000), (0x7C, 0x200),
        (0x80, 1), (0x88, 1), (0x90, 0x5000), (0x94, 0x200), (0xC4, 16),
        (0x404, 12), (0x600, 7), (0x608, 16), (0x800, 28),
    ]:
        struct.pack_into("<I", image, offset, value)
    for index, (name, virtual_size, characteristics) in enumerate([
        (b".text", 0x10, 0x60000020),
        (b"/4", 0x40, 0x40000040),
        (b"/15", 0x30, 0x40000040),
    ]):
        header = 0x148 + 40 * index
        image[header:header + len(name)] = name
        for field, value in [
            (8, virtual_size), (12, (index + 1) * 0x1000),
            (16, 0x200), (20, (index + 1) * 0x200), (36, characteristics),
        ]:
            struct.pack_into("<I", image, header + field, value)
    for offset, data in [
        (0x200, b"\xcc" * 16),
        (0x40C, b"sbat,1,2024010900\nshim,4\ngrub,3\n"),
        (0x610, bytes.fromhex("30820010020100")),
        (0x804, b".sbatlevel\0"),
        (0x80F, b".vendor_cert\0"),
    ]:
        image[offset:offset + len(data)] = data
    return bytes(image)


def guid(text):
    return uuid.UUID(text).bytes_le


GLOBAL = guid("8be4df61-93ca-11d2-aa0d-00e098032b8c")
DATABASE = guid("d719b2cb-3d3a-4596-a3bc-dad00e67656f")
SHIM = guid("605dab50-e046-4300-abb6-3dd810dd8b23")
X509 = guid("a5c059a1-94e4-4aa7-87b5-ab155c2bf072")
SHA256_ESL = guid("c1c41626-504c-4092-aca9-41f936934328")
PHASES = [b"sysinit", b"preconfigured", b"configured", b"ready", b"shutdown", b"final"]


def esl(cert, kind=X509):
    return kind + struct.pack("<III", 44 + len(cert), 0, 16 + len(cert)) + SHIM + cert


def variable(namespace, name, data):
    return namespace + struct.pack("<QQ", len(name), len(data)) + name.encode("utf-16le") + data


def cpio_entry(name, inode, mode, data):
    fields = [inode, mode, 0, 0, 1, 0, len(data), 0, 0, 0, 0, len(name) + 1, 0]
    result = b"070701" + "".join(f"{word:08x}" for word in fields).encode() + name + b"\0"
    result += bytes(-len(result) % 4)
    result += data
    return result + bytes(-len(result) % 4)


def predictions(machine, algorithm):
    hash_fn = getattr(hashlib, algorithm)
    zero = bytes(hash_fn().digest_size)

    def extend(current, event):
        return hash_fn(current + hash_fn(event).digest()).digest()

    def replay(events, initial=zero):
        current = initial
        for event in events:
            current = extend(current, event)
        return current

    def phases(initial):
        values = [initial.hex()]
        for phase in PHASES:
            initial = extend(initial, phase)
            values.append(initial.hex())
        return values

    kernel = pe_image(machine, [(".text", b"fixture kernel")])
    cmdline = b'console=ttyS0 dm-mod.create="root test" -- systemd.unit=fipscheck.target\0'
    sections = [
        (".cmdline", cmdline), (".uname", b"6.18.48\0"),
        (".osrel", b"ID=bottlerocket\n"), (".linux", kernel), (".sbat", b"sbat,1\n"),
    ]
    image = pe_image(machine, sections)
    # These unsigned, contiguous fixtures have no certificate table or overlay.
    # Exclude the four-byte checksum and eight-byte security directory.
    pe_digest = hash_fn(image[:0x98] + image[0x9C:0xE8] + image[0xF0:]).digest()
    pcr4 = hash_fn(
        replay([b"Calling EFI Application from Boot Option", bytes(4)]) + pe_digest
    ).hexdigest()
    cert = esl(b"fixture certificate")
    policy = [variable(GLOBAL, "SecureBoot", b"\x01")]
    policy += [
        variable(GLOBAL if name in ("PK", "KEK") else DATABASE, name, cert)
        for name in ("PK", "KEK", "db", "dbx")
    ]
    policy += [bytes(4), variable(DATABASE, "db", cert[28:])]
    archive = cpio_entry(b".extra", 1, 0o40555, b"")
    archive += cpio_entry(b".extra/os-release", 2, 0o100444, b"ID=bottlerocket\n")
    trailer = cpio_entry(b"TRAILER!!!", 0, 0, b"")
    # systemd v257.13's constant spells the trailer namesize with uppercase B.
    archive += trailer[:94] + b"0000000B" + trailer[102:]
    section_events = []
    payloads = dict(sections)
    for name in [".linux", ".osrel", ".cmdline", ".uname", ".sbat"]:
        section_events += [name.encode() + b"\0", payloads[name]]
    vendor = bytes.fromhex("30820010020100")
    grub_policy = policy + [
        variable(SHIM, "SbatLevel", b"sbat,1,2024010900\nshim,4\ngrub,3\n"),
        variable(SHIM, "MokListRT", SHIM + vendor),
    ]
    shim = shim_image()
    # Contiguous sections followed by the COFF overlay: all bytes after the
    # security directory are hashed, including the overlay and its padding.
    shim_digest = hash_fn(shim[:0x98] + shim[0x9C:0xE8] + shim[0xF0:]).digest()
    grub_pcr4 = replay([b"Calling EFI Application from Boot Option", bytes(4)])
    for _ in range(3):
        grub_pcr4 = hash_fn(grub_pcr4 + shim_digest).digest()
    return {
        "authenticode": pe_digest.hex(),
        "uki": {
            "4": [pcr4],
            "7": [replay(policy).hex()],
            "9": [replay([cmdline.decode().encode("utf-16le"), archive]).hex()],
            "11": phases(replay(section_events)),
        },
        "grub": {
            "4": [grub_pcr4.hex()],
            "7": [replay(grub_policy).hex()],
            "9": [replay([b"BOOT_IMAGE=/vmlinuz -- x\n"]).hex()],
            "11": phases(zero),
            "14": [replay([esl(vendor), esl(bytes(32), SHA256_ESL), b"\x01"]).hex()],
        },
    }


print(json.dumps({
    arch: {bank: predictions(machine, bank) for bank in ("sha256", "sha384")}
    for arch, machine in [("x86_64", 0x8664), ("aarch64", 0xAA64)]
}, indent=2))
