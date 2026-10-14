# Exact console commands and output

Variant: `aws-mantle-1-fips`.

## disk-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-fips /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys disk --image /tmp/symphony-issue24-redesign-440e8668-90fe-4f5c-871a-9db2b91d0493/raw-images/e365f3c2-d085-4b61-b7ef-32fb59687d78/os.img --efi-vars /home/fedora/Repositories/symphony/.symphony/builds/e365f3c2-d085-4b61-b7ef-32fb59687d78/artifacts/state/sbkeys/local/efi-vars.json --platform aws
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

## ami-predictions

```console
$ MATRIX_VARIANT=aws-mantle-1-fips /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/tools/pcrsys ami --ami-id ami-0d430eed38c01e49c --region us-west-2
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

## boot-id-before

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
acb0bb1e-ad3d-4445-b7b2-ce39346d5b52
```

stderr:

```text
SSM command ID: 35573547-d13c-42b4-b5d5-182bf79a9816
```

## ready-before

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: 79872ddd-d5aa-4ddd-94b4-e87bd8c36027
```

## cmdline

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'cat /proc/cmdline'
```

Exit code: 0.

stdout:

```text
SYSTEMD_CGROUP_ENABLE_LEGACY_FORCE=1 SYSTEMD_DEFAULT_MOUNT_RATE_LIMIT_BURST=25 fips=1 sysctl.kernel.kexec_load_disabled=1 systemd.mask=proc-sys-fs-binfmt_misc.mount module_blacklist=binfmt_misc,i8042 console=tty0 console=ttyS0,115200n8 net.ifnames=0 netdog.default-interface=eth0:dhcp4,dhcp6? quiet root=/dev/dm-0 rootwait ro raid=noautodetect random.trust_cpu=on selinux=1 enforcing=1 dm-mod.create="root,,,ro,0 417032 verity 1 PARTUUID=E6D99874-E945-4103-89AF-78A2DD130182/PARTNROFF=1 PARTUUID=E6D99874-E945-4103-89AF-78A2DD130182/PARTNROFF=2 4096 4096 52129 1 sha256 62d6051fda6585e4719459202c911fae3632a5651a6ce695d1f953e802ec0701 21d812b0c4278be416e07bed0bddee60d906f911fdb5d942011a9cf28d4ab1d4 2 restart_on_corruption ignore_zero_blocks" -- systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
```

stderr:

```text
SSM command ID: 5f66748d-6cb0-4b07-b8da-5df482f2f38f
```

## system-status

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'systemctl status --no-pager'
```

Exit code: 0.

stdout:

```text
● ip-172-31-17-96.us-west-2.compute.internal
    State: running
    Units: 395 loaded (incl. loaded aliases)
     Jobs: 0 queued
   Failed: 0 units
    Since: Wed 2026-10-14 04:20:35 UTC; 3min 14s ago
  systemd: 257.13
  Tainted: unmerged-bin
   CGroup: /
           ├─default
           │ ├─admin
           │ │ ├─app.slice
           │ │ │ ├─app-getty.slice
           │ │ │ │ └─getty@tty0.service
           │ │ │ │   └─2435 /sbin/agetty -o "-- \\u" --noclear - xterm-256color
           │ │ │ ├─app-serial\x2dgetty.slice
           │ │ │ │ └─serial-getty@ttyS0.service
           │ │ │ │   └─2436 /sbin/agetty -o "-- \\u" --keep-baud 115200,57600,38400,9600 - xterm-256color
           │ │ │ └─sshd.service
           │ │ │   └─2438 "sshd: /usr/sbin/sshd -e -D [listener] 0 of 10-100 startups"
           │ │ └─init.scope
           │ │   ├─2231 /usr/lib/systemd/systemd --user --unit=admin.target
           │ │   ├─2991 nsenter -t 1 -a -- systemctl status --no-pager
           │ │   └─2999 systemctl status --no-pager
           │ └─control
           │   ├─2162 /usr/bin/amazon-ssm-agent
           │   ├─2441 /usr/bin/ssm-agent-worker
           │   ├─2463 /usr/bin/ssm-document-worker 0fe09f44-5076-4102-b016-8c49669ecd56
           │   ├─2502 /bin/bash /var/lib/amazon/ssm/i-0991c697890b79d01/document/orchestration/0fe09f44-5076-4102-b016-8c49669ecd56/invokeInspectorSsmPluginLinux/_script.sh
           │   ├─2641 /usr/bin/coreutils --coreutils-prog-shebang=sleep /usr/bin/sleep 60
           │   ├─2951 /usr/bin/ssm-document-worker 43a40fa2-4110-4ba7-8b9a-4a95d469dfac
           │   ├─2959 sh -c /var/lib/amazon/ssm/i-0991c697890b79d01/document/orchestration/43a40fa2-4110-4ba7-8b9a-4a95d469dfac/awsrunShellScript/0.awsrunShellScript/_script.sh
           │   └─2960 apiclient exec admin sheltie -- systemctl status --no-pager
           ├─init.scope
           │ └─1 /sbin/init systemd.unit=fipscheck.target systemd.log_target=journal-or-kmsg systemd.log_color=0 systemd.show_status=true
           ├─runtime.slice
           │ └─containerd.service
           │   └─2070 /usr/bin/containerd
           └─system.slice
             ├─apiserver.service
             │ ├─1829 /usr/bin/apiserver --datastore-path /var/lib/bottlerocket/datastore/current --socket-gid 274
             │ └─2966 /usr/bin/ctr -a /run/host-containerd/containerd.sock task exec --exec-id apiexec-d8y47dD5Whbip9MA admin sheltie systemctl status --no-pager
             ├─chronyd.service
             │ ├─2015 /usr/sbin/chronyd -d -F -1
             │ └─2019 /usr/sbin/chronyd -d -F -1
             ├─host-containerd.service
             │ ├─2016 /usr/bin/containerd --config /etc/host-containerd/config.toml
             │ ├─2138 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id control -address /run/host-containerd/containerd.sock
             │ └─2208 /x86_64-bottlerocket-linux-gnu/sys-root/usr/bin/containerd-shim-runc-v2 -namespace default -id admin -address /run/host-containerd/containerd.sock
             ├─system-host\x2dcontainers.slice
             │ ├─host-containers@admin.service
             │ │ └─2063 /usr/bin/host-ctr run --container-id=admin --source=328549459982.dkr.ecr-fips.us-west-2.amazonaws.com/bottlerocket-admin:v0.21.6 --superpowered=true "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             │ └─host-containers@control.service
             │   └─2064 /usr/bin/host-ctr run --container-id=control --source=328549459982.dkr.ecr-fips.us-west-2.amazonaws.com/bottlerocket-control:v0.21.6 --superpowered=false "--command=[]" --registry-config=/etc/host-containers/host-ctr.toml
             ├─systemd-journald.service
             │ └─864 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-journald
             ├─systemd-logind.service
             │ └─1816 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-logind
             ├─systemd-networkd.service
             │ └─1660 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-networkd
             ├─systemd-resolved.service
             │ └─938 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-resolved
             ├─systemd-udevd.service
             │ └─udev
             │   └─1507 /x86_64-bottlerocket-linux-gnu/sys-root/usr/lib/systemd/systemd-udevd
             └─whippet.service
               ├─ 940 /usr/bin/whippet
               └─1505 /usr/bin/dbus-broker --log 12 --controller 11 --machine-id ec2618234dabc4cab0c4e75d59977bf8 --max-bytes 536870912 --max-fds 4096 --max-matches 16384
```

stderr:

```text
SSM command ID: 43a40fa2-4110-4ba7-8b9a-4a95d469dfac
```

## failed-units

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'systemctl --failed --no-pager'
```

Exit code: 0.

stdout:

```text
  UNIT LOAD ACTIVE SUB DESCRIPTION

0 loaded units listed.
```

stderr:

```text
SSM command ID: 7e847b6e-056b-444c-934e-d22506a49be7
```

## secureboot

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'od -An -j4 -tu1 /sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c'
```

Exit code: 0.

stdout:

```text
   1
```

stderr:

```text
SSM command ID: 74463f07-88fc-4233-a6d5-0d2031d01f77
```

## pcrs

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'tpm2_pcrread sha256:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23+sha384:0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23'
```

Exit code: 0.

stdout:

```text
  sha256:
    0 : 0x737F767A12F54E70EECBC8684011323AE2FE2DD9F90785577969D7A2013E8C12
    1 : 0x7E3169F2A09C49891768B2DE17ED716CF10C3C36B7264CC1D11ED000ADC9DB0E
    2 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    3 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    4 : 0xD48CFFB9DA19041E5588AFD78A1368F727ADDEB1BE226B663720C722D15839F1
    5 : 0xBF187B9A0AA126A79D9623F5774E701257186F904C532EDC682041979FA7F3CF
    6 : 0x3D458CFE55CC03EA1F443F1562BEEC8DF51C75E14A9FCF9A7234A13F198E7969
    7 : 0x9E75496B3519BC3D94DEB7B7C5B8E37014D6D0116A687FDD1149BB416CF09D91
    8 : 0xE25DB77D4AE2778F49F166B37F4B794184A844F8C9BAFC2DC3F43A77E755AD68
    9 : 0x2962E217C282119FDC6B96C652E3F1EBA46DB5B964D65EFD32597FE794ECC124
    10: 0x0000000000000000000000000000000000000000000000000000000000000000
    11: 0x7BE033A545EA86BF615F298390E1B820E5EADE1AC50A196CDA08D6ED382AF218
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
    1 : 0x0081198509B7DC2CF48CFD5B31689378B8D663BCE2A6F34BD9C17685F93DEE03B48B49374C6367A998E35E4C10FC73AF
    2 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    3 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    4 : 0x886E2381789D1C6C6222DA4F81B048C4E05B9685FA1FDEA35AC7031F34BAE8E8E90527B4EB17A8628EE8AF602E8262BD
    5 : 0xBBC796729DAF41E5FC4DCB9AB969A75EF1EBEA5EBD028BACC268312A0442096D4CF7AA492C4049AD59D8A2378EA807B3
    6 : 0x518923B0F955D08DA077C96AABA522B9DECEDE61C599CEA6C41889CFBEA4AE4D50529D96FE4D1AFDAFB65E7F95BF23C4
    7 : 0x3154D152FFCC39B1260695A86D94C4DC09B51EBA3E1A875B42000FF68BBC55F3C1433CD3F5CC1C5025738CE68E617496
    8 : 0xEB2AC6FCCB0492BD3A5B003953B4D39886280252321B55CD0CD3D83F4C7383D3A3C4AFCFA81F3DEB53F9030E474830D4
    9 : 0x45031D411F08A40738A751257130CB55F2AA9AA50357518A24F7C8FA909C6469108FE63D14F0E3A8406D638260AB2356
    10: 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
    11: 0x2A57A305444C23EEA7E126DF90166052B0728A962F215344D445B3E62DCCD62A32F90C577A04D6D666D5373792684BBD
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
SSM command ID: 0f006591-43f6-4d4c-9c55-072955c12987
```

## ready-after

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'systemctl is-active multi-user.target'
```

Exit code: 0.

stdout:

```text
active
```

stderr:

```text
SSM command ID: 6ec50df4-0afe-463c-9a5f-b0271a8c28a7
```

## boot-id-after

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'cat /proc/sys/kernel/random/boot_id'
```

Exit code: 0.

stdout:

```text
acb0bb1e-ad3d-4445-b7b2-ce39346d5b52
```

stderr:

```text
SSM command ID: 1f318181-753e-47be-a961-477c8c9c0e35
```

## fips

```console
$ MATRIX_VARIANT=aws-mantle-1-fips bash /home/fedora/Repositories/symphony/skills/cloud/ssm-executor/scripts/sheltie-command.sh i-0991c697890b79d01 us-west-2 'cat /proc/sys/crypto/fips_enabled'
```

Exit code: 0.

stdout:

```text
1
```

stderr:

```text
SSM command ID: 88ba0d86-14fe-4d36-9b8f-d3efd9f8069a
```

## compare

```console
$ MATRIX_VARIANT=aws-mantle-1-fips /usr/bin/python3 /home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation/compare.py
```

Exit code: 0.

stdout:

```text
{
  "variant": "aws-mantle-1-fips",
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
    "live": 857,
    "predicted": 857
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
    "fips_enabled": true
  }
}
```

stderr:

```text
```
