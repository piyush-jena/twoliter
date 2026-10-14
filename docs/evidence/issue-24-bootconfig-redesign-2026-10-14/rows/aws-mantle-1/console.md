# Exact console commands and output

Variant: `aws-mantle-1`.

## disk-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/2bbc9163-1a48-4c11-8303-45b99e671b39/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/2bbc9163-1a48-4c11-8303-45b99e671b39/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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

## ami-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys ami --ami-id ami-0e89c2e4b400118a8 --region us-west-2
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

## boot-id-before

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
e8c51452-6cb5-4730-8f89-0c4a2f75a513
```

stderr:

```text
SSM command ID: 80fbea0c-8ddc-4ff7-b448-9d56c50a3ef3
```

## ready-before

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: f2d85a43-fdde-4aee-93f8-0ad0bebabcd1
```

## cmdline

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'cat /proc/cmdline'
```

Exit code: 0.

stdout:

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 417848 verity 1 PARTUUID=58F856E8-A21D-423F-8D6E-C864A80BB96E/PARTNROFF=1 PARTUUID=58F856E8-A21D-423F-8D6E-C864A80BB96E/PARTNROFF=2 4096 4096 52231 1 sha256 ae062934a6bb320d7d4181cb3900cc907bc63758bf0db9dad0e05dbd64cbf829 8c95a684bb0bd90f43988dbe06a760718e65650887ce585eae0884f676663771 2 restart_on_corruption ignore_zero_blocks" -- systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

stderr:

```text
SSM command ID: e2f7eff5-41df-4dfb-955f-d0bda1a970a8
```

## system-status

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'systemctl status --no-pager'
```

Exit code: 0.

stdout:

```text
● ip-172-31-24-229.us-west-2.compute.internal
    State: running
    Units: 361 loaded (incl. loaded aliases)
     Jobs: 0 queued
   Failed: 0 units
    Since: Wed 2026-10-14 04:17:22 UTC; 2min 32s ago
  systemd: 257.13
  Tainted: unmerged-bin
   CGroup: /
           ├─default
           │ ├─admin
           │ │ ├─app.slice
           │ │ │ ├─app-getty.slice
           │ │ │ │ └─getty@tty0.service
           │ │ │ │   └─2194 /sbin/agetty -o "-- \\u" --noclear - xterm-256color
           │ │ │ ├─app-serial\x2dgetty.slice
           │ │ │ │ └─serial-getty@ttyS0.service
           │ │ │ │   └─2195 /sbin/agetty -o "-- \\u" --keep-baud 115200,57600,38400,9600 - xterm-256color
           │ │ │ └─sshd.service
           │ │ │   └─2197 "sshd: /usr/sbin/sshd -e -D [listener] 0 of 10-100 startups"
           │ │ └─init.scope
           │ │   ├─1997 /usr/lib/systemd/systemd --user --unit=admin.target
           │ │   ├─2784 nsenter -t 1 -a -- systemctl status --no-pager
           │ │   └─2792 systemctl status --no-pager
           │ └─control
           │   ├─1930 /usr/bin/amazon-ssm-agent
           │   ├─2200 /usr/bin/ssm-agent-worker
           │   ├─2214 /usr/bin/ssm-document-worker 6b5a8519-e83f-40db-a4d6-919d66c1e4fc
           │   ├─2260 /bin/bash /var/lib/amazon/ssm/i-0c1e6db9fcb23bdd4/document/orchestration/6b5a8519-e83f-40db-a4d6-919d66c1e4fc/invokeInspectorSsmPluginLinux/_script.sh
           │   ├─2541 /usr/bin/coreutils --coreutils-prog-shebang=sleep /usr/bin/sleep 60
           │   ├─2693 /usr/bin/ssm-document-worker e2f7eff5-41df-4dfb-955f-d0bda1a970a8
           │   ├─2744 /usr/bin/ssm-document-worker 556da959-144e-42af-b9b8-baefabb8d560
           │   ├─2752 sh -c /var/lib/amazon/ssm/i-0c1e6db9fcb23bdd4/document/orchestration/556da959-144e-42af-b9b8-baefabb8d560/awsrunShellScript/0.awsrunShellScript/_script.sh
           │   └─2753 apiclient exec admin sheltie -- systemctl status --no-pager
           ├─init.scope
           │ └─1 /sbin/init systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
           ├─runtime.slice
           │ └─containerd.service
           │   └─1841 /usr/bin/containerd
           └─system.slice
             ├─apiserver.service
             │ ├─1601 /usr/bin/apiserver --datastore-path /var/lib/bottlerocket/datastore/current --socket-gid 274
             │ └─2759 /usr/bin/ctr -a /run/host-containerd/containerd.sock task exec --exec-id apiexec-LO7Vkrj14eeFe2Zr admin sheltie systemctl status --no-pager
             ├─chronyd.service
             │ ├─1786 /usr/sbin/chronyd -d -F -1
             │ └─1790 /usr/sbin/chronyd -d -F -1
             ├─host-containerd.service
             │ ├─1787 /usr/bin/containerd --config /etc/host-containerd/config.toml
             │ ├─1908 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id control -address /run/host-containerd/containerd.sock
             │ └─1971 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id admin -address /run/host-containerd/containerd.sock
             ├─system-host\x2dcontainers.slice
             │ ├─host-containers@admin.service
             │ │ └─1834 /usr/bin/host-ctr run --container-id=admin --source=328549459982.dkr.ecr.us-west-2.amazonaws.com/bottlerocket-admin:v0.21.6 --superpowered=true "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             │ └─host-containers@control.service
             │   └─1835 /usr/bin/host-ctr run --container-id=control --source=328549459982.dkr.ecr.us-west-2.amazonaws.com/bottlerocket-control:v0.21.6 --superpowered=false "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             ├─systemd-journald.service
             │ └─642 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-journald
             ├─systemd-logind.service
             │ └─1588 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-logind
             ├─systemd-networkd.service
             │ └─1429 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-networkd
             ├─systemd-resolved.service
             │ └─1266 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-resolved
             ├─systemd-udevd.service
             │ └─udev
             │   └─1270 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-udevd
             └─whippet.service
               ├─ 720 /usr/bin/whippet
               └─1275 /usr/bin/dbus-broker --log 12 --controller 11 --machine-id ec27f0ddfadd57e6b1d1f13d9c895573 --max-bytes 536870912 --max-fds 4096 --max-matches 16384
```

stderr:

```text
SSM command ID: 556da959-144e-42af-b9b8-baefabb8d560
```

## failed-units

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'systemctl --failed --no-pager'
```

Exit code: 0.

stdout:

```text
  UNIT LOAD ACTIVE SUB DESCRIPTION

0 loaded units listed.
```

stderr:

```text
SSM command ID: 20479e07-f2f8-4c5a-949f-19e8563f4636
```

## secureboot

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'od -An -j4 -tu1 /sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c'
```

Exit code: 0.

stdout:

```text
   1
```

stderr:

```text
SSM command ID: 72fa0e6a-dc79-450a-8b70-900bc991eab3
```

## pcrs

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'tpm2_pcrread sha256:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23+sha384:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23'
```

Exit code: 0.

stdout:

```text
  sha256:
    0 : 0x737F767A12F54E70EECBC8684011323AE2FE2DD9F90785577969D7A2013E8C12
    1 : 0x9E4BD460B758D931C90D597F7C891F1A089D1E35A0D6D00CCFADA9E03EABCBCC
    2 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    3 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    4 : 0xB0AB078F1E3C21552AD782DB5F879A70C3C4BE66E4785389C90D03B6527CEAC5
    5 : 0xEA0BE0EF400FFD0E01904AA4B3E66EA28171875F8B2BDE6DE640BB85D185980E
    6 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    7 : 0x9E75496B3519BC3D94DEB7B7C5B8E37014D6D0116A687FDD1149BB416CF09D91
    8 : 0x11B0912281B2656BD2678A8D1D6A817996F9ABC59EBBF0FC32F25FED4458956C
    9 : 0x9D4C28285AC00EBEBA67AA73B4F88EC84DF0441F32E67DB4E301EB2142CF57F5
    10: 0x0000000000000000000000000000000000000000000000000000000000000000
    11: 0x592049A0FEE1D260B0C20E359C3CF7A63CD322DACF4E2CE17BE1E8B7FE3D3016
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
    1 : 0x039C2F2EB1EB02E57CD7F725360C488A6D961A4561FDB9F798AB036121BD0FB5A4C47C5CB551A8A56F2B41C5EBF5D7AC
    2 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    3 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    4 : 0x5C0DF8B8546F77C03ED8184BF054EA6C1ADD95CEBDBA78976A3EE0CC401BF2BA19015B353E0FA16F8180906ED451F464
    5 : 0x20074D84DBA914B31211DCB268E997FAA09310D1A652AC7CC70637DE8A59F594F2BCCAB5B57452D69B5ADC29C5D73EC6
    6 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    7 : 0x3154D152FFCC39B1260695A86D94C4DC09B51EBA3E1A875B42000FF68BBC55F3C1433CD3F5CC1C5025738CE68E617496
    8 : 0x17B6B5FB7BD441A0B53157DABEF7F603CD71B1413F7281C6867806113CEBEF9FBE1BB439AC84C19B815316E41674A1B0
    9 : 0xD85A2D24C0A95F703F8D4A71C0D47207CC702A2C13C584798F9520359FFE5CBF0824BD512E604625CDDDF2459C9ED394
    10: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    11: 0xBB0EA64C126E6A05AD482B6C963A55BADCF93F5B429D3BFD5C41CE163B3510D8EAC96227F309484FC53E7D3EF20BB144
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
SSM command ID: 2a9d586e-c77e-41ec-8814-cf28af4c2269
```

## ready-after

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: c6e80f59-e5a1-4900-b95e-ad67e9928f72
```

## boot-id-after

```console
$ MATRIX_VARIANT=aws-mantle-1 bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0c1e6db9fcb23bdd4 us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
e8c51452-6cb5-4730-8f89-0c4a2f75a513
```

stderr:

```text
SSM command ID: ddb8a6e5-1747-49ba-b154-e5b594415f3a
```

## compare

```console
$ MATRIX_VARIANT=aws-mantle-1 /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/compare.py
```

Exit code: 0.

stdout:

```text
{
  "variant": "aws-mantle-1",
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
    "live": 820,
    "predicted": 820
  },
  "direct_pcr9_from_live_cmdline": {},
  "health": {
    "ready": true,
    "same_boot": true,
    "system_running": true,
    "no_failed_units": true,
    "version": true,
    "architecture": true,
    "secure_boot": true
  }
}
```

stderr:

```text
```
