# Final-tool replay: aws-mantle-1

Final tool source `830ee97501b05adbf6dc76d13383643e440a6002`; retained image/live source `09380d356886842b8987b602ed2ef7daaefec4f3`.

Recorded instance: `i-0c1e6db9fcb23bdd4`; AMI: `ami-0e89c2e4b400118a8`.

## disk-predictions

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/2bbc9163-1a48-4c11-8303-45b99e671b39/artifacts/state/sbkeys/local/efi-vars.json --platform aws
```

Exit code: 0.

stdout:
```text
{
  "pcrs": {
    "0": {
      "sha256": [
        "737f767a12f54e70eecbc8684011323ae2fe2dd9f90785577969d7a2013e8c12"
      ],
      "sha384": [
        "6e901b16932f6e036747d7a57696e4a2cc864008ebc016826ca1d7bd42ab5ac8286ccf49cde6c0284cbc4b63d978a2ec"
      ]
    },
    "2": {
      "sha256": [
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969"
      ],
      "sha384": [
        "518923b0f955d08da077c96aaba522b9decede61c599cea6c41889cfbea4ae4d50529d96fe4d1afdafb65e7f95bf23c4"
      ]
    },
    "3": {
      "sha256": [
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969"
      ],
      "sha384": [
        "518923b0f955d08da077c96aaba522b9decede61c599cea6c41889cfbea4ae4d50529d96fe4d1afdafb65e7f95bf23c4"
      ]
    },
    "4": {
      "sha256": [
        "b0ab078f1e3c21552ad782db5f879a70c3c4be66e4785389c90d03b6527ceac5"
      ],
      "sha384": [
        "5c0df8b8546f77c03ed8184bf054ea6c1add95cebdba78976a3ee0cc401bf2ba19015b353e0fa16f8180906ed451f464"
      ]
    },
    "5": {
      "sha256": [
        "ea0be0ef400ffd0e01904aa4b3e66ea28171875f8b2bde6de640bb85d185980e"
      ],
      "sha384": [
        "20074d84dba914b31211dcb268e997faa09310d1a652ac7cc70637de8a59f594f2bccab5b57452d69b5adc29c5d73ec6"
      ]
    },
    "6": {
      "sha256": [
        "3d458cfe55cc03ea1f443f1562beec8df51c75e14a9fcf9a7234a13f198e7969"
      ],
      "sha384": [
        "518923b0f955d08da077c96aaba522b9decede61c599cea6c41889cfbea4ae4d50529d96fe4d1afdafb65e7f95bf23c4"
      ]
    },
    "7": {
      "sha256": [
        "9e75496b3519bc3d94deb7b7c5b8e37014d6d0116a687fdd1149bb416cf09d91"
      ],
      "sha384": [
        "3154d152ffcc39b1260695a86d94c4dc09b51eba3e1a875b42000ff68bbc55f3c1433cd3f5cc1c5025738ce68e617496"
      ]
    },
    "9": {
      "sha256": [
        "9d4c28285ac00ebeba67aa73b4f88ec84df0441f32e67db4e301eb2142cf57f5"
      ],
      "sha384": [
        "d85a2d24c0a95f703f8d4a71c0d47207cc702a2c13c584798f9520359ffe5cbf0824bd512e604625cdddf2459c9ed394"
      ]
    },
    "10": {
      "sha256": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sha384": [
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      ]
    },
    "11": {
      "sha256": [
        "adfad420fe1518791ad6c19f045255dd0c0323a89be60d762a2be05a2d4d93c8",
        "06fd494b38ce9de90a381efc0893a11a5b7535c5b1f9239a6cf8ed484f055f55",
        "5ae7a095c977aab0ae2e222c1049619c9edf0177ed38f1fd5a117edafc8fe380",
        "f271a01a45df7f854d1cf44917b0d54bd82da672b3e37658df9652d3829530e0",
        "592049a0fee1d260b0c20e359c3cf7a63cd322dacf4e2ce17be1e8b7fe3d3016",
        "e52f4eb70b834e0a3194b40a9195dabe6ea7d829cb3a5861f9d1d135accfbb0f",
        "9f12d4dfeab594d3eb67a9d15a6bd0f2a1532879c38c55b1612b0c1cd1168e41"
      ],
      "sha384": [
        "7fcdece2b83ab4b8d04e46a4d688e7bb16b2d99bf00461b0d36cdbb110784f5ac7da4f30106509e6bc2aed0ae7561901",
        "a74ced54c1d2949c1da97d13ab0af229a63b3fb093ed9e78543e543472c53135384110f8209d8292c1689ffcffb2b7dc",
        "882aed9aa37d77d936b5ee68d6c3fa964f114fe85501e4889c075454754e7c039a479d907b88b4bfa6f46b9370b5f746",
        "77cc683c864aef332bf3bde14f9c311da68848944eb2cfa4abc57a2e288ca0e2a49ecededc91be613c4f618d785dd2cd",
        "bb0ea64c126e6a05ad482b6c963a55badcf93f5b429d3bfd5c41ce163b3510d8eac96227f309484fc53e7d3ef20bb144",
        "e589ea78005ff09f224172fa7258d5d9b775805dde5f6862584e33ae1fd31da24bf5d8ee58eaa1802105a516d36483b9",
        "fa0bc08287e9129685a74b9ad4df24611bbb6c097f379dec170bad7cbb4949b23ac4e486596846b7e55898c28a9f91a4"
      ]
    },
    "12": {
      "sha256": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sha384": [
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      ]
    },
    "13": {
      "sha256": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sha384": [
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      ]
    },
    "14": {
      "sha256": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sha384": [
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      ]
    },
    "15": {
      "sha256": [
        "0000000000000000000000000000000000000000000000000000000000000000"
      ],
      "sha384": [
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
      ]
    }
  }
}
```

stderr:
```text
```

## final-derive-stub

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1/final-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1/final-stub.efi'
```

## old-derive-stub

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1/old-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1/old-stub.efi'
```

## uki-sections

```console
$ /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/inspect-uki.py /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1/expected-cmdline.txt
```

Exit code: 0.

stdout:
```text
{
  "uki_sha256": "e30272afb5426253c91440aadd608ce300026c1c7cbe1472d26371f86901fd3f",
  "sections_sha256": {
    ".text": "e3dda3240638729037b8d0b22237add48c3e8c052b39db95a399925bddfac17c",
    ".rodata": "608ebd8a96428c7cab9d97cca1b6b7cb3d17f84bbf12a2540fa4ef928e78787d",
    ".data": "67c599e7fa754f14f6d1128dea3d89d491352ac3d592ad78d2d38ff0e0131bf2",
    ".sbat": "3660a29a1b9b8956be4db84932ada381ccff8672237be522b2ff367651545d66",
    ".sdmagic": "692df12d2f74f40ad6f9e745ed61d0efc338fcc3597cd2f0a2ad1afbb260acbf",
    ".reloc": "15af962ff1d18dfa4f3ce6e0a53849cd5bf88a6a1ddffec53a6028b83614cf02",
    ".osrel": "dcdb30f033946c9e64234d748d78b06d6312b69d0543f9caef6232035daf4d62",
    ".cmdline": "965691c5463c17cf0facafc30f0e5cf391f94737a90727d46fab7b90f89f0d31",
    ".uname": "b3cdd940842b4f01d88d1e7ea52f4d73350237c53cfecc6122f9b8cfdb11d13d",
    ".linux": "93f245cfebaaa8cbea150f15847d03fa2c0a6ab0f94362005cb4274aee61a6ac"
  },
  "cmdline_bytes_with_proc_newline": 820,
  "cmdline_sha256_with_proc_newline": "4af150327d4437fd77d76034f563471244d48a73923afbdbfc08e52ef6bfb54f"
}
```

stderr:
```text
```

## Final expected command-line bytes

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 417848 verity 1 PARTUUID=58F856E8-A21D-423F-8D6E-C864A80BB96E/PARTNROFF=1 PARTUUID=58F856E8-A21D-423F-8D6E-C864A80BB96E/PARTNROFF=2 4096 4096 52231 1 sha256 ae062934a6bb320d7d4181cb3900cc907bc63758bf0db9dad0e05dbd64cbf829 8c95a684bb0bd90f43988dbe06a760718e65650887ce585eae0884f676663771 2 restart_on_corruption ignore_zero_blocks" -- systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

Byte-identical to retained expected bytes and the recorded SSM `/proc/cmdline` output.

Original live receipts: `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/runs/aws-mantle-1/2bbc9163-1a48-4c11-8303-45b99e671b39/live/`.
