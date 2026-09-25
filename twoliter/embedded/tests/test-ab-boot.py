#!/usr/bin/env python3
"""Exercise real PE manifest generation/repack in the SDK with supplied test inputs.

Usage: test-ab-boot.py HELPER STUB SIGNED_KERNEL OUTPUT_DIRECTORY
The helper directory must also contain the built ukisys binary.
"""
from pathlib import Path
import hashlib
import subprocess
import sys

helper, stub, kernel, output = map(Path, sys.argv[1:])
output.mkdir(parents=True, exist_ok=True)
manifest = output / 'manifest.efi'
template = 'console=ttyS0 bootconfig root=/dev/dm-0 ro dm-mod.create="root,,,ro,0 8 verity 1 PARTUUID=@ROOT@ PARTUUID=@HASH@ 4096 4096 1 1 sha256 ' + 'a' * 64 + ' ' + 'b' * 64 + ' 2 restart_on_corruption ignore_zero_blocks" -- systemd.log_color=0'

def call(function, *args, success=True):
    command = ['bash', '-euc', 'source "$1"; shift; "$@"', 'ab-manifest-test', str(helper), function, *map(str, args)]
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    print('+', *command)
    print(result.stdout)
    assert (result.returncode == 0) == success, result.returncode

call('ab_boot_write_manifest', stub, kernel, template, manifest)
raw = output / 'manifest.data'
subprocess.run(['ukify', 'inspect', str(manifest), '--section', f'.brconf:binary@{raw}'], check=True)
expected = b'BRBOOT1\0' + hashlib.sha256(kernel.read_bytes()).digest() + template.encode('ascii') + b'\0'
assert raw.read_bytes() == expected
# Regeneration must bind changed signed-kernel bytes, not a stale prior hash.
changed = output / 'changed-kernel'
changed.write_bytes(kernel.read_bytes() + b'changed')
call('ab_boot_write_manifest', stub, changed, template, output / 'changed.efi')
subprocess.run(['ukify', 'inspect', str(output / 'changed.efi'), '--section', f'.brconf:binary@{raw}'], check=True)
assert raw.read_bytes()[8:40] == hashlib.sha256(changed.read_bytes()).digest()
assert raw.read_bytes()[8:40] != expected[8:40]
# Recover the unsigned stub using the production repack tool and rebuild it.
derived = output / 'derived.efi.stub'
subprocess.run([str(helper.parent / 'ukisys'), 'derive-manifest-stub', str(manifest), str(derived)], check=True)
call('ab_boot_write_manifest', derived, kernel, template, output / 'rebuilt.efi')
subprocess.run(['ukify', 'inspect', str(output / 'rebuilt.efi'), '--section', f'.brconf:binary@{raw}'], check=True)
assert raw.read_bytes() == expected
for i, bad in enumerate([template.replace('@ROOT@', 'missing'), template + ' @ROOT@', template + ' @OTHER@', template + '\n', template + 'x' * 2048, template + 'x' * 8192]):
    call('ab_boot_write_manifest', stub, kernel, bad, output / f'bad-{i}.efi', success=False)
    assert not (output / f'bad-{i}.efi').exists()
print('PASS exact manifest bytes, signed-kernel binding, stub recovery, and malformed-template rejection')
