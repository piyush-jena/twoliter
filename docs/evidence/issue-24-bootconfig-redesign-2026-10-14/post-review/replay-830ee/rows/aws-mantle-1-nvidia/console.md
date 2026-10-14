# Final-tool replay: aws-mantle-1-nvidia

Final tool source `830ee97501b05adbf6dc76d13383643e440a6002`; retained image/live source `09380d356886842b8987b602ed2ef7daaefec4f3`.

Recorded instance: `i-0f2809d6dec73df3d`; AMI: `ami-0abcfa155905267d3`.

## disk-predictions

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/0346b16c-3d8a-4c94-b242-2ceb97b4c395/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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
        "20bd1e126f59a70f834e90f5a8a455909207dd26b130236077c3b82378810299"
      ],
      "sha384": [
        "89e559a037e13b5fdbd0c6a78aa345b5e7be66627655986ba2ad4e2bdd72987b5fd6a17958ac28a007c93664372c8774"
      ]
    },
    "5": {
      "sha256": [
        "68bb17b6fb97dad78ffd6826d289d388b816913cc9a3cd30d60d314f3dd89aca"
      ],
      "sha384": [
        "a39745dfee25c8200cc0c373e63fa3a8446934fa768654d26d8de9cc843d65a3af982585c23b808b257b551d28cafd80"
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
        "056810427af3bfc3050b1fa102963e7d9c83155edc9dfe53192825fc12466770"
      ],
      "sha384": [
        "7afcef40c141ed8f251f3b4bfe277dbf4054a9671e652954c5e623a65f96766b51336a38ae08b4a1a30455b0e6cb6414"
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
        "153f606c04ab2b7c04365f47d218f2c61055d670505e0b73709d5dcfc81ea5e3",
        "92c0320147ba8cd43c96ddaf913e024742eeebed1a0d53722de131812e0858f7",
        "49af4eba500f6fe05c31e5ca68fc518c90cf7a3618f9f4d35bf5de5724cc9f2d",
        "e5b89651b29cf350c10e0827aad9b36fbcdfad1be0bdbfebe8d1306f364e9dd1",
        "2e17a2d60b4a31d755a432fadacb4d78dd776e395e0ef9118ea88f91cc64c8be",
        "9300f657abf4a154ec612a8e95b5324e146e9e60f913926eeaf48c38a6dcf1ee",
        "5557de905ff9263628498e5df48901d0e6fefeb86dbc97b5c40b1d9ceaf54b56"
      ],
      "sha384": [
        "d63e1394a476a833035d5c8c8300217e0440d4e4d8b9c5150fdaf56fbf671f55f45ad4984540ff1a36cde7c712a41561",
        "11617e16f600d827f9d811eedd8e9d2e1310c062182ca68df9e885e419b7eab3f5b116790479b4277731dee7a8ae0310",
        "30c1d2fbc2e333d18297eaa1ca78f344ff0ca0caaab2af1c8a5afa38e8e859b33027f83c8b56c81aa132f91eccdbbc40",
        "43384c2f61e9b550abd517c557e8cd751eb2313adff3e7d99aa513d0c5c51220dc8e22a388e604458edc6708d0f22164",
        "7df1cc4aab92e8c0282b092b08df934bd4637a4d65b6af3b2afc37e9eea9b25a3fd6321c1714c359e810a3544d810722",
        "7341be2a310d7dddc9c0f9a4a90874954e628c4cf6a5f01b6d221138574aac983b06c8d02ac8ad572debbde0376395c5",
        "4553c65eac1d7c53d7be6ce5db60caf071cef345509596b16ebd1032dc44e69dc229165b988e02b412ed74aba0565e16"
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia/final-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia/final-stub.efi'
```

## old-derive-stub

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia/old-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia/old-stub.efi'
```

## uki-sections

```console
$ /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/inspect-uki.py /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia/expected-cmdline.txt
```

Exit code: 0.

stdout:
```text
{
  "uki_sha256": "f2f7cbb804879361d79ac26f6e67740fa7e4399c9cf79ccf2b6bcd4ff0066b5d",
  "sections_sha256": {
    ".text": "e3dda3240638729037b8d0b22237add48c3e8c052b39db95a399925bddfac17c",
    ".rodata": "608ebd8a96428c7cab9d97cca1b6b7cb3d17f84bbf12a2540fa4ef928e78787d",
    ".data": "67c599e7fa754f14f6d1128dea3d89d491352ac3d592ad78d2d38ff0e0131bf2",
    ".sbat": "3660a29a1b9b8956be4db84932ada381ccff8672237be522b2ff367651545d66",
    ".sdmagic": "692df12d2f74f40ad6f9e745ed61d0efc338fcc3597cd2f0a2ad1afbb260acbf",
    ".reloc": "15af962ff1d18dfa4f3ce6e0a53849cd5bf88a6a1ddffec53a6028b83614cf02",
    ".osrel": "b6cfa20b3f8251719507f88f04acb7dc36b533850400e168f84de0112a5f6d89",
    ".cmdline": "10eb2811838aa2d70c1f29c57b8d7a4d636b990c29041a89014ed42329f86559",
    ".uname": "b3cdd940842b4f01d88d1e7ea52f4d73350237c53cfecc6122f9b8cfdb11d13d",
    ".linux": "93f245cfebaaa8cbea150f15847d03fa2c0a6ab0f94362005cb4274aee61a6ac"
  },
  "cmdline_bytes_with_proc_newline": 822,
  "cmdline_sha256_with_proc_newline": "bea8caf50b9ed3db0fc2019dfab0ee372192cbf81dd13a34190b1bcbc4e0fddc"
}
```

stderr:
```text
```

## Final expected command-line bytes

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 1635256 verity 1 PARTUUID=3791B59B-B9FC-4C46-8EAF-180663B600D0/PARTNROFF=1 PARTUUID=3791B59B-B9FC-4C46-8EAF-180663B600D0/PARTNROFF=2 4096 4096 204407 1 sha256 d5ab7a1a3397f380e8e3b2ca7f757951612f2397758a2cec09c62aa4698d394b 6caf0dbb4938a3fc042e70b30c7e62a6892e48075fbf9959bd709f2ab27f11d7 2 restart_on_corruption ignore_zero_blocks" -- systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

Byte-identical to retained expected bytes and the recorded SSM `/proc/cmdline` output.

Original live receipts: `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/runs/aws-mantle-1-nvidia/0346b16c-3d8a-4c94-b242-2ceb97b4c395/live/`.
