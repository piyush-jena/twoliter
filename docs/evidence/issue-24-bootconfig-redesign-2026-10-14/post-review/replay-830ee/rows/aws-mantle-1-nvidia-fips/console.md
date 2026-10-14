# Final-tool replay: aws-mantle-1-nvidia-fips

Final tool source `830ee97501b05adbf6dc76d13383643e440a6002`; retained image/live source `09380d356886842b8987b602ed2ef7daaefec4f3`.

Recorded instance: `i-07a548c96b0845c0c`; AMI: `ami-0b00bbba9be690efa`.

## disk-predictions

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/0ca29c05-e38b-4d60-916f-d44be1e37177/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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
        "6e828b2501a5f060d789361f24c4e6db51c54e730c1b36cc87a72a0d94a4d0f0"
      ],
      "sha384": [
        "10acf868a2a1847e47a3fa8fe7bb903aa8917661de7b4ab2955b0ca87d01e7f400e5126d4c820d70f146c5de28753c47"
      ]
    },
    "5": {
      "sha256": [
        "fb6ac381aae504e3c7b4fb01d39c1d8f22004d8a8e5ed969330799e47fa11df0"
      ],
      "sha384": [
        "374b28ef25ee3b8da7400b9ceab321acec6c6fe673b307a1a33b21947dcd7307e6fb55a8b9c8475ff3f6ed489fb6ab60"
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
        "d1c9ab38d431bd4429749eb1be8af952a4f7c6654723bbca9d2fa40564cc256a"
      ],
      "sha384": [
        "313912152fd50e2df461b37a5dc58f0665177b0490eed69a27349db61e2abb7aeb0a86ba6529afed24361acbaae54090"
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
        "e8a740f2f765da5861d1c61e85ef9665a42a45c644722e2abb80d45d02afd2bf",
        "3166bf0a7baa506a9bae7b8183ccd9149c12e8171ea9569087d92a951497e52a",
        "e1c6dfa6b06688755e6d35e07f11b6bdb89ae18fd88be481b757d11e6b7eddb6",
        "4a321c4af24246f8de9eb08a51c1aada22ae5e6068e4cc6e4c42531ae3f761fb",
        "a4df91049b707f98a428913172910698bf4ddc02b0cc87b1c6234bc0a6b9ea56",
        "4fef795637be598e22996c2e38b788201f47ebe94bd1b5ca23786c73e1c702c7",
        "c96aca175ee057ea3c3574eb7c86d6132ecc2a5dbc133566e0604e9388b9e7c6"
      ],
      "sha384": [
        "4e2e63eaa6e1f2d2365b0a81678f5496f7d1d1bf1567a25b00b9bbe620541b1342be92fda723233a5675464f498facec",
        "ada35a9fe42799f0a43277b312749443e8151e26545e66e367f46c4e10b139c614146d8bbb06a93cc486cedf8375ff14",
        "e9f493004885dce61781dd1c0382dceb5caf6c807f47895687b77d156822f2d7f99502cbbe988e50cec5482c910cbea1",
        "0e06eae5cebaa22daaef715fbf670aedf25f306eec0da1224f2a7590a797a717b0816e3e665318213a4172ae18b38dc2",
        "f7f6d3fbb6d248a4b9151dd35805bc050e01fb3704d9ab30c69a3a2165482917e015a88bb025a43cee48133f8070e80a",
        "242f3b01b083bb3f759ece585ae80280eda037342bd3c9794caf19b6a40330f99bbc568fe59dd1526780bf3cade4554c",
        "febfff88f6f5654d17ab494bbcc8c97b2dedf214d7eccc53cba64793d11bac82c6636e975920a366c81784f9a99a7847"
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
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia-fips/final-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia-fips/final-stub.efi'
```

## old-derive-stub

```console
$ /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/ukisys derive-stub /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia-fips/old-stub.efi
```

Exit code: 0.

stdout:
```text
```

stderr:
```text
ukisys: stripped signature and trailing sections [".osrel", ".cmdline", ".uname", ".linux"] from '/tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/bootx64.efi', wrote 84992 bytes to '/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia-fips/old-stub.efi'
```

## uki-sections

```console
$ /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/inspect-uki.py /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/bootx64.efi /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation/rows/aws-mantle-1-nvidia-fips/expected-cmdline.txt
```

Exit code: 0.

stdout:
```text
{
  "uki_sha256": "f8b2273e341a94269683faa010bcffb09a2d40dc5aec033b175886dfcd1524f6",
  "sections_sha256": {
    ".text": "e3dda3240638729037b8d0b22237add48c3e8c052b39db95a399925bddfac17c",
    ".rodata": "608ebd8a96428c7cab9d97cca1b6b7cb3d17f84bbf12a2540fa4ef928e78787d",
    ".data": "67c599e7fa754f14f6d1128dea3d89d491352ac3d592ad78d2d38ff0e0131bf2",
    ".sbat": "3660a29a1b9b8956be4db84932ada381ccff8672237be522b2ff367651545d66",
    ".sdmagic": "692df12d2f74f40ad6f9e745ed61d0efc338fcc3597cd2f0a2ad1afbb260acbf",
    ".reloc": "15af962ff1d18dfa4f3ce6e0a53849cd5bf88a6a1ddffec53a6028b83614cf02",
    ".osrel": "b05449fb0e9fd2ad45cf0411742c041253e3b4c867c72d7a5f8b7cc39f4d7b74",
    ".cmdline": "744e30370e44bd9c40e82b2b0aa6c444848f3e75a005a3712d3b11d6bc128efb",
    ".uname": "b3cdd940842b4f01d88d1e7ea52f4d73350237c53cfecc6122f9b8cfdb11d13d",
    ".linux": "93f245cfebaaa8cbea150f15847d03fa2c0a6ab0f94362005cb4274aee61a6ac"
  },
  "cmdline_bytes_with_proc_newline": 859,
  "cmdline_sha256_with_proc_newline": "e078421d33dae97169554b535b352ed378ea9653d86b398d70d42d5c2262cceb"
}
```

stderr:
```text
```

## Final expected command-line bytes

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 fips=1 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 1634440 verity 1 PARTUUID=15A8AEA2-663E-4848-883A-DEC59D1B2BEB/PARTNROFF=1 PARTUUID=15A8AEA2-663E-4848-883A-DEC59D1B2BEB/PARTNROFF=2 4096 4096 204305 1 sha256 47848abbf47b768648578e2a128c1403d5c7dad065dae2cdd0007a7cf2ecd3d4 a29922282fab97a963f69e707ff964b9ffcc760587696e7b9f6df472aa11dc9f 2 restart_on_corruption ignore_zero_blocks" -- systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

Byte-identical to retained expected bytes and the recorded SSM `/proc/cmdline` output.

Original live receipts: `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/runs/aws-mantle-1-nvidia-fips/0ca29c05-e38b-4d60-916f-d44be1e37177/live/`.
