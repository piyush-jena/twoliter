# Q9: Health claims and limitations

Read raw ready/system-status/failed-units/version/architecture/SecureBoot output for all ten; FIPS sysctl for four FIPS rows; NVIDIA driver version and nvidia-smi device enumeration for five GPU rows. These are fresh-boot health and GPU visibility checks, not GPU workload or comprehensive FIPS qualification. ECS membership receipts identify connected agents with zero tasks. No ECS4 FIPS variants exist in the inventoried checkout. Alternative PCR states, reboot/update transitions, other architectures and workloads were not live tested by this matrix.

Evidence: ../validation/validate.py:30; ../validation/compare.py:88; ../validation/report.md:18; independent-evidence-checks.json.
