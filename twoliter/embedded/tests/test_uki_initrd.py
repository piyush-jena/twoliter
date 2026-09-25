#!/usr/bin/env python3
"""Verify root-confined ELF dependency collection, not kernel boot."""
import pathlib
import runpy
import tempfile
import unittest
from unittest.mock import patch

module = runpy.run_path(str(pathlib.Path(__file__).resolve().parents[1] / 'uki-initrd'))


class InitrdTests(unittest.TestCase):
    def test_absolute_links_stay_inside_image(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / 'usr/lib').mkdir(parents=True)
            (root / 'lib64').symlink_to('/usr/lib')
            (root / 'usr/lib/libc.so.6').write_bytes(b'ELF')
            self.assertEqual(module['resolve'](root, '/lib64/libc.so.6'), root / 'usr/lib/libc.so.6')
            with self.assertRaises(ValueError):
                module['resolve'](root, '/../../etc/passwd')

    def test_symlink_cycle_is_bounded(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / 'cycle').symlink_to('cycle')
            with self.assertRaises(ValueError):
                module['resolve'](root, '/cycle')

    def test_collects_transitive_elf_dependencies_without_execution(self):
        with tempfile.TemporaryDirectory() as tmp:
            root, output = pathlib.Path(tmp) / 'root', pathlib.Path(tmp) / 'output'
            (root / 'usr/bin').mkdir(parents=True)
            (root / 'usr/sbin').mkdir()
            (root / 'usr/lib').mkdir()
            output.mkdir()
            for name in ('bin/uki-init', 'sbin/dmsetup', 'sbin/switch_root'):
                (root / 'usr' / name).write_bytes(b'target-ELF')
            for name in ('libc.so.6', 'ld-linux.so.2'):
                (root / 'usr/lib' / name).write_bytes(b'target-library')
            (root / 'lib64').symlink_to('/usr/lib')
            def headers(argv, **kwargs):
                path = pathlib.Path(argv[-1])
                if path.name in ('uki-init', 'dmsetup', 'switch_root'):
                    return '[Requesting program interpreter: /lib64/ld-linux.so.2]\n (NEEDED) [libc.so.6]'
                return ''
            with patch('subprocess.check_output', side_effect=headers):
                module['build'](root, output)
            self.assertTrue((output / 'init').is_symlink())
            self.assertEqual((output / 'lib64/libc.so.6').read_bytes(), b'target-library')
            self.assertTrue((output / 'lib64/ld-linux.so.2').exists())
            self.assertFalse((output / 'usr/bin/sh').exists())


if __name__ == '__main__':
    unittest.main()
