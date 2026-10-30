//! What the firmware boots, read once from the disk.

use crate::disk::DiskImage;
use crate::diskfs::{Esp, Ext4Partition};
use crate::error::Result;
use crate::gpt::{Gpt, PartitionLayout};
use crate::pe::{ensure_signed, get_authenticode_hash, Arch, Fallback, Shim, Uki};
use crate::platform::Platform;
use crate::predict::PcrDigests;
use snafu::{ensure_whatever, ResultExt};
use std::sync::Arc;

/// What the firmware runs from the ESP's fallback path, and what that loads.
pub enum BootChain {
    /// The firmware runs a UKI directly.
    Uki(Uki),
    /// shim and GRUB boot one of two banks, whose kernels change with updates.
    GrubAb { shim: Shim, layout: PartitionLayout },
    /// shim and GRUB boot the only bank's kernel.
    GrubSingle {
        shim: Shim,
        layout: PartitionLayout,
        payload: Box<GrubPayload>,
    },
}

/// What a single-bank GRUB image measures after the firmware starts shim.
pub struct GrubPayload {
    pub shim: PcrDigests,
    pub grub: PcrDigests,
    pub vmlinuz: PcrDigests,
    /// The architecture vmlinuz is built for, which decides how GRUB starts it.
    pub arch: Arch,
    pub grub_cfg: Vec<u8>,
    pub bootconfig: Vec<u8>,
    pub boot_partuuid: String,
}

impl BootChain {
    /// Read the boot chain, mounting each filesystem once, and reading only what is measured.
    pub fn read(disk: &Arc<dyn DiskImage>, gpt: &Gpt, platform: Platform) -> Result<Self> {
        let esp = Esp::mount(disk, &gpt.esp()?)?;
        let shim = match Fallback::parse(esp.fallback()?)? {
            Fallback::Uki(uki) => {
                ensure_whatever!(
                    platform == Platform::Aws,
                    "direct UKI prediction supports AWS only"
                );
                return Ok(Self::Uki(uki));
            }
            Fallback::Shim(shim) => shim,
        };

        let layout = gpt.grub_layout()?;
        if layout.boot_b.is_some() {
            // Updates change what these measure, so only check that they are present.
            esp.ensure_grub()?;
            let boot_a = Ext4Partition::mount(disk, &layout.boot_a, "BOOT-A")?;
            boot_a.ensure_file("/vmlinuz")?;
            boot_a.ensure_file("/grub/grub.cfg")?;
            Ext4Partition::mount(disk, &layout.private, "PRIVATE")?
                .ensure_file("/bootconfig.data")?;
            return Ok(Self::GrubAb { shim, layout });
        }

        let grub = esp.grub()?;
        let boot_a = Ext4Partition::mount(disk, &layout.boot_a, "BOOT-A")?;
        let vmlinuz = boot_a.read("/vmlinuz")?;
        let grub_cfg = boot_a.read("/grub/grub.cfg")?;
        let bootconfig =
            Ext4Partition::mount(disk, &layout.private, "PRIVATE")?.read("/bootconfig.data")?;
        ensure_signed(&grub, "grub")?;
        ensure_signed(&vmlinuz, "vmlinuz")?;
        let payload = Box::new(GrubPayload {
            shim: shim.authenticode()?,
            grub: get_authenticode_hash(&grub)?,
            vmlinuz: get_authenticode_hash(&vmlinuz)?,
            arch: Arch::of(&vmlinuz)
                .whatever_context("failed to read the architecture of vmlinuz")?,
            grub_cfg,
            bootconfig,
            boot_partuuid: layout.boot_a.partuuid(),
        });
        Ok(Self::GrubSingle {
            shim,
            layout,
            payload,
        })
    }

    /// The shim that chain-loads GRUB, unless the firmware runs a UKI directly.
    pub fn shim(&self) -> Option<&Shim> {
        match self {
            Self::Uki(_) => None,
            Self::GrubAb { shim, .. } | Self::GrubSingle { shim, .. } => Some(shim),
        }
    }
}
