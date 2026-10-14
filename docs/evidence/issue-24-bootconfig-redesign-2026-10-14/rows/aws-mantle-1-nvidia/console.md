# Exact console commands and output

Variant: `aws-mantle-1-nvidia`.

## disk-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/0346b16c-3d8a-4c94-b242-2ceb97b4c395/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/0346b16c-3d8a-4c94-b242-2ceb97b4c395/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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

## ami-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys ami --ami-id ami-0abcfa155905267d3 --region us-west-2
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

## boot-id-before

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
0fe74568-09d9-4974-9620-945aad3c9244
```

stderr:

```text
SSM command ID: c4757681-4dca-4a92-9445-6ff658a24f1c
```

## ready-before

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: 6e8470eb-bceb-4e15-b705-0a2d50185142
```

## cmdline

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'cat /proc/cmdline'
```

Exit code: 0.

stdout:

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 1635256 verity 1 PARTUUID=3791B59B-B9FC-4C46-8EAF-180663B600D0/PARTNROFF=1 PARTUUID=3791B59B-B9FC-4C46-8EAF-180663B600D0/PARTNROFF=2 4096 4096 204407 1 sha256 d5ab7a1a3397f380e8e3b2ca7f757951612f2397758a2cec09c62aa4698d394b 6caf0dbb4938a3fc042e70b30c7e62a6892e48075fbf9959bd709f2ab27f11d7 2 restart_on_corruption ignore_zero_blocks" -- systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

stderr:

```text
SSM command ID: 3c80495a-b1fb-4ae2-9368-221ff9ce0fb0
```

## system-status

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'systemctl status --no-pager'
```

Exit code: 0.

stdout:

```text
● ip-172-31-23-157.us-west-2.compute.internal
    State: running
    Units: 403 loaded (incl. loaded aliases)
     Jobs: 0 queued
   Failed: 0 units
    Since: Wed 2026-10-14 04:21:35 UTC; 3min 22s ago
  systemd: 257.13
  Tainted: unmerged-bin
   CGroup: /
           ├─default
           │ ├─admin
           │ │ ├─app.slice
           │ │ │ ├─app-getty.slice
           │ │ │ │ └─getty@tty0.service
           │ │ │ │   └─2466 /sbin/agetty -o "-- \\u" --noclear - xterm-256color
           │ │ │ ├─app-serial\x2dgetty.slice
           │ │ │ │ └─serial-getty@ttyS0.service
           │ │ │ │   └─2467 /sbin/agetty -o "-- \\u" --keep-baud 115200,57600,38400,9600 - xterm-256color
           │ │ │ └─sshd.service
           │ │ │   └─2469 "sshd: /usr/sbin/sshd -e -D [listener] 0 of 10-100 startups"
           │ │ └─init.scope
           │ │   ├─2213 /usr/lib/systemd/systemd --user --unit=admin.target
           │ │   ├─3061 nsenter -t 1 -a -- systemctl status --no-pager
           │ │   └─3068 systemctl status --no-pager
           │ └─control
           │   ├─2147 /usr/bin/amazon-ssm-agent
           │   ├─2457 /usr/bin/ssm-agent-worker
           │   ├─2485 /usr/bin/ssm-document-worker e698e7b2-3f4e-4928-af84-ab73634e29e8
           │   ├─2531 /bin/bash /var/lib/amazon/ssm/i-0f2809d6dec73df3d/document/orchestration/e698e7b2-3f4e-4928-af84-ab73634e29e8/invokeInspectorSsmPluginLinux/_script.sh
           │   ├─2708 /usr/bin/coreutils --coreutils-prog-shebang=sleep /usr/bin/sleep 60
           │   ├─3017 /usr/bin/ssm-document-worker 65d8974b-6d73-4677-9c35-6ddc057d9cbb
           │   ├─3028 sh -c /var/lib/amazon/ssm/i-0f2809d6dec73df3d/document/orchestration/65d8974b-6d73-4677-9c35-6ddc057d9cbb/awsrunShellScript/0.awsrunShellScript/_script.sh
           │   └─3029 apiclient exec admin sheltie -- systemctl status --no-pager
           ├─init.scope
           │ └─1 /sbin/init systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
           ├─runtime.slice
           │ └─containerd.service
           │   └─2037 /usr/bin/containerd
           └─system.slice
             ├─apiserver.service
             │ ├─1672 /usr/bin/apiserver --datastore-path /var/lib/bottlerocket/datastore/current --socket-gid 274
             │ └─3035 /usr/bin/ctr -a /run/host-containerd/containerd.sock task exec --exec-id apiexec-cVEBJvMqq58TlcLM admin sheltie systemctl status --no-pager
             ├─chronyd.service
             │ ├─1971 /usr/sbin/chronyd -d -F -1
             │ └─1973 /usr/sbin/chronyd -d -F -1
             ├─host-containerd.service
             │ ├─1972 /usr/bin/containerd --config /etc/host-containerd/config.toml
             │ ├─2121 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id control -address /run/host-containerd/containerd.sock
             │ └─2190 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id admin -address /run/host-containerd/containerd.sock
             ├─nvidia-persistenced.service
             │ └─1819 /usr/bin/nvidia-persistenced --user nvidia --verbose
             ├─system-host\x2dcontainers.slice
             │ ├─host-containers@admin.service
             │ │ └─2022 /usr/bin/host-ctr run --container-id=admin --source=328549459982.dkr.ecr.us-west-2.amazonaws.com/bottlerocket-admin:v0.21.6 --superpowered=true "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             │ └─host-containers@control.service
             │   └─2023 /usr/bin/host-ctr run --container-id=control --source=328549459982.dkr.ecr.us-west-2.amazonaws.com/bottlerocket-control:v0.21.6 --superpowered=false "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             ├─systemd-journald.service
             │ └─646 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-journald
             ├─systemd-logind.service
             │ └─1643 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-logind
             ├─systemd-networkd.service
             │ └─1459 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-networkd
             ├─systemd-resolved.service
             │ └─1378 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-resolved
             ├─systemd-udevd.service
             │ └─udev
             │   └─730 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-udevd
             └─whippet.service
               ├─ 727 /usr/bin/whippet
               └─1298 /usr/bin/dbus-broker --log 12 --controller 11 --machine-id ec26e10a93464ad42ac852c65cfef6a5 --max-bytes 536870912 --max-fds 4096 --max-matches 16384
```

stderr:

```text
SSM command ID: 65d8974b-6d73-4677-9c35-6ddc057d9cbb
```

## failed-units

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'systemctl --failed --no-pager'
```

Exit code: 0.

stdout:

```text
  UNIT LOAD ACTIVE SUB DESCRIPTION

0 loaded units listed.
```

stderr:

```text
SSM command ID: 9d12040f-fc80-4939-bd82-10a7e0b20d51
```

## secureboot

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'od -An -j4 -tu1 /sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c'
```

Exit code: 0.

stdout:

```text
   1
```

stderr:

```text
SSM command ID: 37332af6-32cc-410b-8cf5-373f1a2fa379
```

## pcrs

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'tpm2_pcrread sha256:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23+sha384:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23'
```

Exit code: 0.

stdout:

```text
  sha256:
    0 : 0x737F767A12F54E70EECBC8684011323AE2FE2DD9F90785577969D7A2013E8C12
    1 : 0x6198373F3B34E06DEA5723CAB8D1B9C4273E1F6ABCCF9BD98FB2DB4C728C640C
    2 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    3 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    4 : 0x20BD1E126F59A70F834E90F5A8A455909207DD26B130236077C3B82378810299
    5 : 0x68BB17B6FB97DAD78FFD6826D289D388B816913CC9A3CD30D60D314F3DD89ACA
    6 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    7 : 0x9E75496B3519BC3D94DEB7B7C5B8E37014D6D0116A687FDD1149BB416CF09D91
    8 : 0xA797A2C04985E1AAF2A4804193A5FA6EDEFB3324F51DCF3F7F8F4D34A4056A82
    9 : 0x056810427AF3BFC3050B1FA102963E7D9C83155EDC9DFE53192825FC12466770
    10: 0x0000000000000000000000000000000000000000000000000000000000000000
    11: 0x2E17A2D60B4A31D755A432FADACB4D78DD776E395E0EF9118EA88F91CC64C8BE
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
    1 : 0xE8C6F7ADDC1D7B801567916599E3928CF42506FE683C29991FBD38089244CA69941ED4137C0861B143E37BD89DAFCB3E
    2 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    3 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    4 : 0x89E559A037E13B5FDBD0C6A78AA345B5E7BE66627655986BA2AD4E2BDD72987B5FD6A17958AC28A007C93664372C8774
    5 : 0xA39745DFEE25C8200CC0C373E63FA3A8446934FA768654D26D8DE9CC843D65A3AF982585C23B808B257B551D28CAFD80
    6 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    7 : 0x3154D152FFCC39B1260695A86D94C4DC09B51EBA3E1A875B42000FF68BBC55F3C1433CD3F5CC1C5025738CE68E617496
    8 : 0xE31BF2CBD9843E8AB6C00A0643F9DC73AA806B5362FAD53D99C8E5A1A57A2F1F875953AE76DBBE31662C5316BD68EE62
    9 : 0x7AFCEF40C141ED8F251F3B4BFE277DBF4054A9671E652954C5E623A65F96766B51336A38AE08B4A1A30455B0E6CB6414
    10: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    11: 0x7DF1CC4AAB92E8C0282B092B08DF934BD4637A4D65B6AF3B2AFC37E9EEA9B25A3FD6321C1714C359E810A3544D810722
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
SSM command ID: a010ac42-a396-451c-925c-33cdd2be9fb5
```

## ready-after

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: b01efbe2-8c92-4811-8998-d0f1930c11e3
```

## boot-id-after

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
0fe74568-09d9-4974-9620-945aad3c9244
```

stderr:

```text
SSM command ID: ef0eaefd-d21e-4c81-8af6-40a35bbdb1ff
```

## nvidia-driver

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'cat /proc/driver/nvidia/version'
```

Exit code: 0.

stdout:

```text
NVRM version: NVIDIA UNIX x86_64 Kernel Module  580.178.04  Tue Jul  7 12:38:21 UTC 2026
GCC version:  gcc version 13.4.0 (Buildroot 2025.05.1) 
```

stderr:

```text
SSM command ID: 21bd1139-0479-41ee-89cd-2ffbbd2b05b0
```

## nvidia-devices

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0f2809d6dec73df3d us-west-2 'nvidia-smi --query-gpu=name,driver_version,pci.bus_id --format=csv,noheader'
```

Exit code: 0.

stdout:

```text
NVIDIA A10G, 580.178.04, 00000000:00:1E.0
```

stderr:

```text
SSM command ID: 0caf56b7-7d83-468e-960a-1d055e482a17
```

## compare

```console
$ MATRIX_VARIANT=aws-mantle-1-nvidia /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/compare.py
```

Exit code: 0.

stdout:

```text
{
  "variant": "aws-mantle-1-nvidia",
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
    "live": 822,
    "predicted": 822
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
    "nvidia_driver_loaded": true,
    "nvidia_device_visible": true
  }
}
```

stderr:

```text
```
