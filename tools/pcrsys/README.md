# pcrsys

A TPM Platform Configuration Register (PCR) prediction tool for Bottlerocket.

## Overview

`pcrsys` predicts TPM PCR values (SHA-256) for Bottlerocket images without requiring a running system. It analyzes disk images and EFI variables to compute what the PCR values would be after boot, enabling pre-registration of expected measurements for attestation policies.

## Usage

### From a local disk image

```bash
pcrsys disk --image /path/to/disk.img --efi-vars /path/to/efi-vars.json [--platform aws|vmware|metal]
```

### From an AWS AMI

```bash
pcrsys ami --ami-id ami-0123456789abcdef0 [--region us-west-2] [--profile myprofile]
```

The AMI subcommand downloads the root snapshot using coldsnap and retrieves UEFI data from the AMI attributes.

## Output

JSON output with predicted PCR values, keyed by PCR index:

```json
{
  "pcrs": {
    "0": { "sha256": ["<hex digest>"] },
    "4": { "sha256": ["<hex digest>"] },
    "7": { "sha256": ["<hex digest>"] }
  }
}
```

## Supported PCRs

The [PCR reference](../../docs/design/systemd-boot-ab-pcrs.md) explains all
PCRs 0–23, their actual producers, platform assumptions and measured-test limits.
Predictions are SHA-256 model outputs; an omitted PCR is unknown, not zero.

| PCR | Prediction behavior |
| --- | --- |
| 0 | Hard-coded AWS value; omitted on VMware/metal. |
| 1 | Separator-only on VMware; omitted on AWS/metal. |
| 2 | Legacy AWS/VMware separator-only; omitted on metal and dedicated systemd path. |
| 3 | AWS/VMware separator-only; omitted on metal. |
| 4 | Legacy single-bank executable-chain model; omitted for A/B. |
| 5 | AWS/metal GPT candidates: 72 for A/B, 12 for single-bank; omitted on VMware. |
| 6 | AWS/VMware separator-only; omitted on metal. |
| 7 | Legacy Secure Boot policy model; omitted on dedicated systemd path. |
| 9 | Legacy single-bank command-line model; omitted for A/B. |
| 10 | Zero assumption. |
| 11 | Zero plus six cumulative Bottlerocket boot-phase states. |
| 12 | Legacy zero assumption; omitted on dedicated systemd path. |
| 13 | Zero assumption. |
| 14 | Legacy shim MOK model; omitted on dedicated systemd path. |
| 15 | Zero assumption. |

PCRs 8 and 16–23 are not predicted. Zero assumptions do not establish that a
register is unused on a running system.

## Supported Platforms

- **aws**: AWS Nitro (EC2 instances)
- **vmware**: VMware vSphere
- **metal**: Bare metal servers

Platform differences affect PCR 0, 1, 4, 5, and 7 calculations due to firmware behavior variations.

## Input Requirements

### efi-vars.json

JSON file containing Secure Boot variables:

```json
{
  "variables": [
    { "name": "PK", "guid": "8be4df61-93ca-11d2-aa0d-00e098032b8c", "data": "<hex>" },
    { "name": "KEK", "guid": "8be4df61-93ca-11d2-aa0d-00e098032b8c", "data": "<hex>" },
    { "name": "db", "guid": "d719b2cb-3d3a-4596-a3bc-dad00e67656f", "data": "<hex>" },
    { "name": "dbx", "guid": "d719b2cb-3d3a-4596-a3bc-dad00e67656f", "data": "<hex>" }
  ]
}
```

### Disk image

GPT-partitioned disk image containing:

- EFI-A (FAT) with `/EFI/BOOT/boot{aa64,x64}.efi` (shim). Legacy prediction also
  requires `/EFI/BOOT/grub{aa64,x64}.efi`; dedicated-loader detection checks
  `/EFI/BOOT/systemd-boot{aa64,x64}.efi`.
- BOOT-A (Bottlerocket BOOT type, ext4) with `/vmlinuz` and `/grub/grub.cfg`.
- PRIVATE (ext4) with `/bootconfig.data`.

BOOT-B is optional for legacy images; its presence selects A/B prediction.
The dedicated systemd loader requires BOOT-B.

## GPT systemd-boot images

The dedicated `systemd-boot-ab` loader is detected on EFI-A. It requires an A/B disk layout. Signed GRUB configuration remains in the intermediate image for legacy loader recovery, but does not select the PCR prediction model. Prediction still extracts BOOT-A `/vmlinuz`, BOOT-A `/grub/grub.cfg`, and PRIVATE `/bootconfig.data` on this path; only extraction of the GRUB EFI binary is skipped. Missing required files fail the run even when their associated PCRs are omitted.

PCRs 4 and 9 remain omitted for A/B images. This loader also omits PCRs 2, 7, 12 and 14: driver measurements depend on the firmware/shim verification path, load options depend on the selected bank, and delegation to a retained legacy shim can add policy measurements. These registers are not reported as zero or as legacy GRUB values. Platform-specific predictions for the remaining registers retain their existing behavior. Qualify the actual firmware and installed transition before using predictions for attestation policy.

PCR 5 retains its AWS/metal candidate set; those candidates do not model arbitrary
GPT states or installed BootOrder/BootNext changes. Detection inspects EFI-A only.
An installed migration retaining legacy EFI-A while firmware selects new EFI-B
is not modeled correctly by that detector. See the [installed migration
design](../../docs/design/systemd-boot-ab.md) before using image predictions for
an upgraded machine.
