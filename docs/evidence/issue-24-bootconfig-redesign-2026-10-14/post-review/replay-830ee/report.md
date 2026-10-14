# Post-review final-tool replay

Final tools: `830ee97501b05adbf6dc76d13383643e440a6002`; original image builds and live boots: `09380d356886842b8987b602ed2ef7daaefec4f3`.

Final conductor source check, integration and release jobs passed. Final `pcrsys` output is byte-for-byte identical to the original generated, disk and direct-AMI prediction JSON for all ten retained images. All 256 generated PCR bank/register comparisons match the recorded live readings (128 per bank). Six final GRUB formatter outputs match the complete recorded command lines, including newline, and their 12 separate PCR9 diagnostics match. Four retained UKI section interpretations and old/final derived stubs agree.

The actual final release `ukisys` rejects the mixed block/canonical blacklist review case and preserves its input file. Five valid controls produce identical old/final bytes. See `release-blacklist-console.md` and `release-blacklist-comparison.json` for exact commands and results.

| Variant | SHA256 | SHA384 | Cmdline bytes | JSON bytes equal |
| --- | --- | --- | --- | --- |
| aws-ecs-3 | 12/12 | 12/12 | 760 | true |
| aws-ecs-3-fips | 12/12 | 12/12 | 797 | true |
| aws-ecs-3-nvidia | 12/12 | 12/12 | 762 | true |
| aws-ecs-3-nvidia-fips | 12/12 | 12/12 | 799 | true |
| aws-ecs-4 | 12/12 | 12/12 | 760 | true |
| aws-ecs-4-nvidia | 12/12 | 12/12 | 762 | true |
| aws-mantle-1 | 14/14 | 14/14 | 820 | true |
| aws-mantle-1-fips | 14/14 | 14/14 | 857 | true |
| aws-mantle-1-nvidia | 14/14 | 14/14 | 822 | true |
| aws-mantle-1-nvidia-fips | 14/14 | 14/14 | 859 | true |

## Provenance and limits

No new image was built or booted, and no new EC2/SSM/AMI queries were made for this replay. The live observations remain the original 09380d captures; 830ee is validated here by passing its gates and reproducing those positive artifacts and outputs. This does not claim a live boot from 830ee.

This follows the single review's rejection-only fix. The source diff changes only the blacklist form guard ordering and its regression. Main owns the review finding closure and documentation publication; no second critic round ran.

PCR11 comparison uses recorded ready phase (index 4) and stable boot IDs. Unpredicted indices remain listed separately in each comparison JSON. The same original fresh-boot x86_64 scope applies; no workloads, reboot/update transitions or absent ecs-4 FIPS variants are covered.

Full exact command receipts, raw outputs, tool binaries and provenance are retained at `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/post-review-validation`. Original live matrix evidence is unchanged at `/home/fedora/Repositories/symphony/stages/24-uki-pcr-predictions/.symphony/issue24-bootconfig-redesign-2026-10-14/validation`. This selected bundle contains text evidence and hashes, without images, binaries, private keys or archives.
