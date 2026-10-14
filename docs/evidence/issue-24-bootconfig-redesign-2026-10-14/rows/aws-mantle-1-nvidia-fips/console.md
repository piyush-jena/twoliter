# Exact console commands and output

Variant: `aws-mantle-1-nvidia-fips`.

## disk-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0ca29c05-e38b-4d60-916f-d44be1e37177/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/0ca29c05-e38b-4d60-916f-d44be1e37177/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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

## ami-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys ami --ami-id ami-0b00bbba9be690efa --region us-west-2
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

## boot-id-before

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
4de2ac48-de39-4348-a135-5797231a73ff
```

stderr:

```text
SSM command ID: 6ce8e615-51c4-4461-9ec2-e71b8d27b9e3
```

## ready-before

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: ea4c6782-5197-4dc3-a54b-0cd9483a3037
```

## cmdline

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'cat /proc/cmdline'
```

Exit code: 0.

stdout:

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 fips=1 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 1634440 verity 1 PARTUUID=15A8AEA2-663E-4848-883A-DEC59D1B2BEB/PARTNROFF=1 PARTUUID=15A8AEA2-663E-4848-883A-DEC59D1B2BEB/PARTNROFF=2 4096 4096 204305 1 sha256 47848abbf47b768648578e2a128c1403d5c7dad065dae2cdd0007a7cf2ecd3d4 a29922282fab97a963f69e707ff964b9ffcc760587696e7b9f6df472aa11dc9f 2 restart_on_corruption ignore_zero_blocks" -- systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

stderr:

```text
SSM command ID: be191ffa-4065-4e9a-ae93-e78abdc473d6
```

## system-status

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'systemctl status --no-pager'
```

Exit code: 0.

stdout:

```text
● ip-172-31-31-23.us-west-2.compute.internal
    State: running
    Units: 437 loaded (incl. loaded aliases)
     Jobs: 0 queued
   Failed: 0 units
    Since: Wed 2026-10-14 04:22:55 UTC; 2min 53s ago
  systemd: 257.13
  Tainted: unmerged-bin
   CGroup: /
           ├─default
           │ ├─admin
           │ │ ├─app.slice
           │ │ │ ├─app-getty.slice
           │ │ │ │ └─getty@tty0.service
           │ │ │ │   └─2718 /sbin/agetty -o "-- \\u" --noclear - xterm-256color
           │ │ │ ├─app-serial\x2dgetty.slice
           │ │ │ │ └─serial-getty@ttyS0.service
           │ │ │ │   └─2719 /sbin/agetty -o "-- \\u" --keep-baud 115200,57600,38400,9600 - xterm-256color
           │ │ │ └─sshd.service
           │ │ │   └─2721 "sshd: /usr/sbin/sshd -e -D [listener] 0 of 10-100 startups"
           │ │ └─init.scope
           │ │   ├─2468 /usr/lib/systemd/systemd --user --unit=admin.target
           │ │   ├─3294 nsenter -t 1 -a -- systemctl status --no-pager
           │ │   └─3301 systemctl status --no-pager
           │ └─control
           │   ├─2380 /usr/bin/amazon-ssm-agent
           │   ├─2591 /usr/bin/ssm-agent-worker
           │   ├─2727 /usr/bin/ssm-document-worker bb2abd49-40c3-4e24-b7d0-728ed3b41564
           │   ├─2778 /bin/bash /var/lib/amazon/ssm/i-07a548c96b0845c0c/document/orchestration/bb2abd49-40c3-4e24-b7d0-728ed3b41564/invokeInspectorSsmPluginLinux/_script.sh
           │   ├─2950 /usr/bin/coreutils --coreutils-prog-shebang=sleep /usr/bin/sleep 60
           │   ├─3254 /usr/bin/ssm-document-worker f41d31e7-932d-4869-bf1b-045f4bdfb3c0
           │   ├─3262 sh -c /var/lib/amazon/ssm/i-07a548c96b0845c0c/document/orchestration/f41d31e7-932d-4869-bf1b-045f4bdfb3c0/awsrunShellScript/0.awsrunShellScript/_script.sh
           │   └─3263 apiclient exec admin sheltie -- systemctl status --no-pager
           ├─init.scope
           │ └─1 /sbin/init systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
           ├─runtime.slice
           │ └─containerd.service
           │   └─2274 /usr/bin/containerd
           └─system.slice
             ├─apiserver.service
             │ ├─1888 /usr/bin/apiserver --datastore-path /var/lib/bottlerocket/datastore/current --socket-gid 274
             │ └─3271 /usr/bin/ctr -a /run/host-containerd/containerd.sock task exec --exec-id apiexec-lvELPAS6xq4ltqJq admin sheltie systemctl status --no-pager
             ├─chronyd.service
             │ ├─2208 /usr/sbin/chronyd -d -F -1
             │ └─2210 /usr/sbin/chronyd -d -F -1
             ├─host-containerd.service
             │ ├─2209 /usr/bin/containerd --config /etc/host-containerd/config.toml
             │ ├─2356 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id control -address /run/host-containerd/containerd.sock
             │ └─2447 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id admin -address /run/host-containerd/containerd.sock
             ├─nvidia-persistenced.service
             │ └─2059 /usr/bin/nvidia-persistenced --user nvidia --verbose
             ├─system-host\x2dcontainers.slice
             │ ├─host-containers@admin.service
             │ │ └─2259 /usr/bin/host-ctr run --container-id=admin --source=328549459982.dkr.ecr-fips.us-west-2.amazonaws.com/bottlerocket-admin:v0.21.6 --superpowered=true "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             │ └─host-containers@control.service
             │   └─2260 /usr/bin/host-ctr run --container-id=control --source=328549459982.dkr.ecr-fips.us-west-2.amazonaws.com/bottlerocket-control:v0.21.6 --superpowered=false "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             ├─systemd-journald.service
             │ └─872 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-journald
             ├─systemd-logind.service
             │ └─1868 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-logind
             ├─systemd-networkd.service
             │ └─1694 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-networkd
             ├─systemd-resolved.service
             │ └─965 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-resolved
             ├─systemd-udevd.service
             │ └─udev
             │   └─968 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-udevd
             └─whippet.service
               ├─ 962 /usr/bin/whippet
               └─1566 /usr/bin/dbus-broker --log 12 --controller 11 --machine-id ec2eaf6506573d7ada74a71858d1df26 --max-bytes 536870912 --max-fds 4096 --max-matches 16384
```

stderr:

```text
SSM command ID: f41d31e7-932d-4869-bf1b-045f4bdfb3c0
```

## failed-units

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'systemctl --failed --no-pager'
```

Exit code: 0.

stdout:

```text
  UNIT LOAD ACTIVE SUB DESCRIPTION

0 loaded units listed.
```

stderr:

```text
SSM command ID: 34b28505-0c39-425e-bf7c-17734e71f92b
```

## secureboot

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'od -An -j4 -tu1 /sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c'
```

Exit code: 0.

stdout:

```text
   1
```

stderr:

```text
SSM command ID: 3546c1ee-6c31-41f5-9766-978e56dcc428
```

## pcrs

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'tpm2_pcrread sha256:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23+sha384:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23'
```

Exit code: 0.

stdout:

```text
  sha256:
    0 : 0x737F767A12F54E70EECBC8684011323AE2FE2DD9F90785577969D7A2013E8C12
    1 : 0x48B022210D833987A8519EE6E53A368D3E23BC42FE91466223995736A6914650
    2 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    3 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    4 : 0x6E828B2501A5F060D789361F24C4E6DB51C54E730C1B36CC87A72A0D94A4D0F0
    5 : 0xFB6AC381AAE504E3C7B4FB01D39C1D8F22004D8A8E5ED969330799E47FA11DF0
    6 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    7 : 0x9E75496B3519BC3D94DEB7B7C5B8E37014D6D0116A687FDD1149BB416CF09D91
    8 : 0x3828AC81992C83F4D0DB0A67C4C8D07F9E72457942CD1043BD4CAC95D17C1D1D
    9 : 0xD1C9AB38D431BD4429749EB1BE8AF952A4F7C6654723BBCA9D2FA40564CC256A
    10: 0x0000000000000000000000000000000000000000000000000000000000000000
    11: 0xA4DF91049B707F98A428913172910698BF4DDC02B0CC87B1C6234BC0A6B9EA56
    12: 0x0000000000000000000000000000000000000000000000000000000000000000
    13: 0x0000000000000000000000000000000000000000000000000000000000000000
    14: 0x0000000000000000000000000000000000000000000000000000000000000000
    15: 0x0000000000000000000000000000000000000000000000000000000000000000
    16: 0x0000000000000000000000000000000000000000000000000000000000000000
    17: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    18: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    19: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    20: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    21: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    22: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    23: 0x0000000000000000000000000000000000000000000000000000000000000000
  sha384:
    0 : 0x6E901B16932F6E036747D7A57696E4A2CC864008EBC016826CA1D7BD42AB5AC8286CCF49CDE6C0284CBC4B63D978A2EC
    1 : 0x0F12D00F7A6F9DCEAC2565A569BD840DB441A9B5D9DAD0973F8647D49EC95324FE511C81FDEC4B24E9DC41BBA1F6A66F
    2 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    3 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    4 : 0x10ACF868A2A1847E47A3FA8FE7BB903AA8917661DE7B4AB2955B0CA87D01E7F400E5126D4C820D70F146C5DE28753C47
    5 : 0x374B28EF25EE3B8DA7400B9CEAB321ACEC6C6FE673B307A1A33B21947DCD7307E6FB55A8B9C8475FF3F6ED489FB6AB60
    6 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    7 : 0x3154D152FFCC39B1260695A86D94C4DC09B51EBA3E1A875B42000FF68BBC55F3C1433CD3F5CC1C5025738CE68E617496
    8 : 0x3712A51C30007504B7356C8F774335CB63CDE7FA237F44FB4BCBD48C2B545CD20D4F506A5EC9B64D760B8370B610F1D1
    9 : 0x313912152FD50E2DF461B37A5DC58F0665177B0490EED69A27349DB61E2ABB7AEB0A86BA6529AFED24361ACBAAE54090
    10: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    11: 0xF7F6D3FBB6D248A4B9151DD35805BC050E01FB3704D9AB30C69A3A2165482917E015A88BB025A43CEE48133F8070E80A
    12: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    13: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    14: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    15: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    16: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    17: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    18: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    19: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    20: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    21: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    22: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    23: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

stderr:

```text
SSM command ID: 87000eac-3769-4a75-bf1f-55fbc53e7934
```

## ready-after

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: d0f649db-1d2d-4407-bb14-ef2ebeddb5ad
```

## boot-id-after

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
4de2ac48-de39-4348-a135-5797231a73ff
```

stderr:

```text
SSM command ID: c8fbf5b3-1090-496b-9ee4-6629c486736f
```

## fips

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'cat /proc/sys/crypto/fips_enabled'
```

Exit code: 0.

stdout:

```text
1
```

stderr:

```text
SSM command ID: f0aa7b06-d492-4af0-9359-7da8662cf272
```

## nvidia-driver

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'cat /proc/driver/nvidia/version'
```

Exit code: 0.

stdout:

```text
NVRM version: NVIDIA UNIX x86_64 Kernel Module  580.178.04  Tue Jul  7 12:38:21 UTC 2026
GCC version:  gcc version 13.4.0 (Buildroot 2025.05.1) 
```

stderr:

```text
SSM command ID: ec10bef5-bbb8-40aa-90b6-e41ffdd86ada
```

## nvidia-devices

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-07a548c96b0845c0c us-west-2 'nvidia-smi --query-gpu=name,driver_version,pci.bus_id --format=csv,noheader'
```

Exit code: 0.

stdout:

```text
NVIDIA A10G, 580.178.04, 00000000:00:1E.0
```

stderr:

```text
SSM command ID: f9037054-4ad2-412f-a1f4-8b788ba60b7c
```

## compare

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia-fips /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/compare.py
```

Exit code: 0.

stdout:

```text
{
  "variant": "aws-mantle-1-nvidia-fips",
  "passed": true,
  "banks": {
    "sha256": {
      "matches": 14,
      "comparisons": 14
    },
    "sha384": {
      "matches": 14,
      "comparisons": 14
    }
  },
  "predicted_indices": [
    0,
    2,
    3,
    4,
    5,
    6,
    7,
    9,
    10,
    11,
    12,
    13,
    14,
    15
  ],
  "all_predicted_match": true,
  "cmdline_exact_match": true,
  "cmdline_bytes": {
    "live": 859,
    "predicted": 859
  },
  "direct_pcr9_from_live_cmdline": {},
  "health": {
    "ready": true,
    "same_boot": true,
    "system_running": true,
    "no_failed_units": true,
    "version": true,
    "architecture": true,
    "secure_boot": true,
    "fips_enabled": true,
    "nvidia_driver_loaded": true,
    "nvidia_device_visible": true
  }
}
```

stderr:

```text
```
