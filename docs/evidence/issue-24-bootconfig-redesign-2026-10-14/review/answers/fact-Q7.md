# Q7: Image and tool provenance

Rehashed all 386 returned image-build artifacts (14,225,226,110 bytes), inputs and build logs. Compared 62 bootconfig/ukisys/pcrsys/manifests source files in each of ten build input archives to PR6. Verified final tool hashes and successful extraction receipts; diagnostic wrapper links directly to the candidate bootconfig crate. Recompared original build predictions with disk and direct registered-AMI JSON. Checked exact pcrsys ami --ami-id commands, exit codes, region/account, instance-to-image mapping, snapshot mappings and registered EFI bytes. All image source receipts identify Bottlerocket 5cfeacd and core-kit 17.0.1-symphony.24.build-fc201212.

Evidence: independent-evidence-checks.json; verify-evidence.py; ../validation/bootstrap.py:23; ../validation/prepare.py:9; ../validation/validate.py:10.
