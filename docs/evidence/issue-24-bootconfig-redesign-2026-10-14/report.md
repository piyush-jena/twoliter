# Bootconfig redesign live matrix

AMI build and live-test Twoliter: `09380d356886842b8987b602ed2ef7daaefec4f3`.
Status: 10/10 rows passed; 256/256 generated PCR bank/register comparisons matched.

| Variant | Status | SHA256 | SHA384 | Exact cmdline | AMI | Instance |
| --- | --- | --- | --- | --- | --- | --- |
| aws-ecs-3 | passed | 12/12 | 12/12 | True | ami-04e6a5f5ac705d24f | i-0d8de2e0e5c7a41c0 |
| aws-ecs-3-fips | passed | 12/12 | 12/12 | True | ami-0cb466f09ddc54edb | i-072638ca96ffbd2e5 |
| aws-ecs-3-nvidia | passed | 12/12 | 12/12 | True | ami-04a0f6f02cf1c6864 | i-0e38c23bfc3565904 |
| aws-ecs-3-nvidia-fips | passed | 12/12 | 12/12 | True | ami-04da6f2ab03f37052 | i-0d675caf3f5902aed |
| aws-ecs-4 | passed | 12/12 | 12/12 | True | ami-0209b72b06de0b6f6 | i-0b691843d806f06bc |
| aws-ecs-4-nvidia | passed | 12/12 | 12/12 | True | ami-02f1acf56cc89115c | i-0c24dea93544ba99a |
| aws-mantle-1 | passed | 14/14 | 14/14 | True | ami-0e89c2e4b400118a8 | i-0c1e6db9fcb23bdd4 |
| aws-mantle-1-fips | passed | 14/14 | 14/14 | True | ami-0d430eed38c01e49c | i-0991c697890b79d01 |
| aws-mantle-1-nvidia | passed | 14/14 | 14/14 | True | ami-0abcfa155905267d3 | i-0f2809d6dec73df3d |
| aws-mantle-1-nvidia-fips | passed | 14/14 | 14/14 | True | ami-0b00bbba9be690efa | i-07a548c96b0845c0c |

Existing variants only: aws-ecs-4-fips and aws-ecs-4-nvidia-fips are absent from the inventoried checkout.
Both TPM banks are read in full. Unpredicted indices are recorded per row and are not counted as passing generated predictions.
GRUB PCR9 diagnostics remain separate whenever the generated A/B predictions omit PCR9. UKI PCR9 is validated through the generated LoadOptions/initrd prediction.
Each published row contains exact command stdout/stderr in console.md, comparison.json and expected-cmdline.txt. Full SSM invocation readbacks remain in the canonical local evidence identified by full-evidence-pointers.json; the combined PCR comparison CSV is included here.
All command-line equality checks compare complete bytes, including quotes, whitespace, ordering and the final newline.

New AMIs and instances are retained. Prior resources were not modified. No runtime archive was uploaded. The validation scripts do not publish PR or issue comments.
The final adversarial review is under `review/`; its one finding and the subsequent fix are described in `post-review/resolution.md`. The matrix was not rebuilt for that malformed-input rejection fix.
