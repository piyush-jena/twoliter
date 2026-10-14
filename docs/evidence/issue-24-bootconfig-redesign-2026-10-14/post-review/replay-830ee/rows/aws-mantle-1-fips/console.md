# Final-tool replay: aws-mantle-1-fips

Final tool source `830ee97501b05adbf6dc76d13383643e440a6002`; retained image/live source `09380d356886842b8987b602ed2ef7daaefec4f3`.

Recorded instance: `i-0991c697890b79d01`; AMI: `ami-0d430eed38c01e49c`.

## disk-predictions

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/e365f3c2-d085-4b61-b7ef-32fb59687d78/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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
        "d48cffb9da19041e5588afd78a1368f727addeb1be226b663720c722d15839f1"
      ],
      "sha384": [
        "886e2381789d1c6c6222da4f81b048c4e05b9685fa1fdea35ac7031f34bae8e8e90527b4eb17a8628ee8af602e8262bd"
      ]
    },
    "5": {
      "sha256": [
        "bf187b9a0aa126a79d9623f5774e701257186f904c532edc682041979fa7f3cf"
      ],
      "sha384": [
        "bbc796729daf41e5fc4dcb9ab969a75ef1ebea5ebd028bacc268312a0442096d4cf7aa492c4049ad59d8a2378ea807b3"
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
        "2962e217c282119fdc6b96c652e3f1eba46db5b964d65efd32597fe794ecc124"
      ],
      "sha384": [
        "45031d411f08a40738a751257130cb55f2aa9aa50357518a24f7c8fa909c6469108fe63d14f0e3a8406d638260ab2356"
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
        "532450239a8c0841b56f7756725600e5377e1330b45372c7af629956fb38e0e4",
        "b64f1e0510e338b0161835009a63dbf95b1e1e7c32bb9fdb5303778b450a0156",
        "4ffd84aa631fb25f693c314ee2477edb14bec0686ebbc4c63386efb2ff3da46c",
        "f88c65ef2e8c63f20cbf252f6d23505de934b45a974874f3ee2a83692ea60a01",
        "7be033a545ea86bf615f298390e1b820e5eade1ac50a196cda08d6ed382af218",
        "bf3f19d982e2a79d14a9fe7c4ebc1bf2deaeba130d9e30ad31243a6abdba203a",
        "8fd3d6a8d9dcda8b65d31b76a062a517659802e75bd364a4376d4cfbadff874e"
      ],
      "sha384": [
        "809933950e6fed9620ade513c1411f5b80f5734b8355e0ccb4ace62c8378eec56c0d216bc6a5dbfc3c1098b264b215b7",
        "66d2080bb8cee87f9611ee8aa646cf21b7594f90137281311f375719fbb5f1ba3d90aee89402cefc8ddca4c0f7a92d2f",
        "4a9775ed19e1902f0e82a00eb8988c1fd86f0ea4fef41d58707175a54eabb38390b4bf1d01e4186a34bb26c8add7fec1",
        "cfb5269e659c05af20fad571d240e4f28c36afcae322a9a36ab34ae6f22074a64d14cb7282303aab6342afb24efc1db4",
        "2a57a305444c23eea7e126df90166052b0728a962f215344d445b3e62dccd62a32f90c577a04d6d666d5373792684bbd",
        "4ef2c8a86b6a7f0fa6efee89d0be69cf674f6ababe59a606a8de169265d136b8f7ff36446c1db742283355fdfc26e467",
        "c2e705e6ab01e4692692d527daed812b07ec136b9159a7511bca1b82d4550882defcf295168a9bcea765a07c0ba375b6"
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-fips/final-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-fips/final-stub.efi'
```

## old-derive-stub

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-fips/old-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-fips/old-stub.efi'
```

## uki-sections

```console
$ /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/inspect-uki.py /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-fips/expected-cmdline.txt
```

Exit code: 0.

stdout:
```text
{
  "uki_sha256": "69ba40f4041c46f3b0a43eb3fe7a1b3dc0b1a422b6b7647fc02bc7035129f516",
  "sections_sha256": {
    ".text": "e3dda3240638729037b8d0b22237add48c3e8c052b39db95a399925bddfac17c",
    ".rodata": "608ebd8a96428c7cab9d97cca1b6b7cb3d17f84bbf12a2540fa4ef928e78787d",
    ".data": "67c599e7fa754f14f6d1128dea3d89d491352ac3d592ad78d2d38ff0e0131bf2",
    ".sbat": "3660a29a1b9b8956be4db84932ada381ccff8672237be522b2ff367651545d66",
    ".sdmagic": "692df12d2f74f40ad6f9e745ed61d0efc338fcc3597cd2f0a2ad1afbb260acbf",
    ".reloc": "15af962ff1d18dfa4f3ce6e0a53849cd5bf88a6a1ddffec53a6028b83614cf02",
    ".osrel": "1f9c5dc1f95b4c24cf0ca7190955ab54ec6b965404d77efaa5d1ecee242b7fa8",
    ".cmdline": "dc0f680a0722a856d2e3ab078f14734e23305e1da25666887c4fe52b06116482",
    ".uname": "b3cdd940842b4f01d88d1e7ea52f4d73350237c53cfecc6122f9b8cfdb11d13d",
    ".linux": "93f245cfebaaa8cbea150f15847d03fa2c0a6ab0f94362005cb4274aee61a6ac"
  },
  "cmdline_bytes_with_proc_newline": 857,
  "cmdline_sha256_with_proc_newline": "bbd6154a7f9cc0da6bcee1f41fce38115bdf2b338386ed7b7fe0242acfa38976"
}
```

stderr:
```text
```

## Final expected command-line bytes

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 fips=1 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 417032 verity 1 PARTUUID=E6D99874-E945-4103-89AF-78A2DD130182/PARTNROFF=1 PARTUUID=E6D99874-E945-4103-89AF-78A2DD130182/PARTNROFF=2 4096 4096 52129 1 sha256 62d6051fda6585e4719459202c911fae3632a5651a6ce695d1f953e802ec0701 21d812b0c4278be416e07bed0bddee60d906f911fdb5d942011a9cf28d4ab1d4 2 restart_on_corruption ignore_zero_blocks" -- systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

Byte-identical to retained expected bytes and the recorded SSM `/proc/cmdline` output.

Original live receipts: `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/runs/aws-mantle-1-fips/e365f3c2-d085-4b61-b7ef-32fb59687d78/live/`.
