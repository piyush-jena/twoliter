#!/usr/bin/env python3
"""Replay the review case and supported drop-ins with old and final CLI tools."""
import hashlib, json, shlex, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parent
OLD = ROOT.parent / "validation/tools/ukisys"
NEW = Path("/tmp/symphony-ukisys-redesign-check/target/debug/ukisys")
CASES = {
    "mixed-block": "kernel { module_blacklist = binfmt_misc }\nkernel.module_blacklist = i8042\n",
    "canonical": "kernel.module_blacklist = i8042\nkernel.module_blacklist = binfmt_misc\nkernel.module_blacklist = i8042\nkernel.systemd.mask = proc-sys-fs-binfmt_misc.mount\n",
    "aliases": "kernel.module-blacklist = i8042\nkernel.module_blacklist = binfmt_misc\n",
    "arrays": "kernel.module_blacklist = i8042, binfmt_misc\nkernel.console = tty0, ttyS0\n",
    "plain": "kernel.console = tty0, ttyS0\nkernel.quiet\n",
    "fips": "kernel.fips = 1\nkernel.module_blacklist = i8042\nkernel.module-blacklist = binfmt_misc\n",
}
results = {}
transcript = ["# Post-review blacklist regression and output comparison", "", "The old binary is the release used for all ten live boots. The new binary is", "a local Rust 1.97.1 build of final source `830ee97501b05adbf6dc76d13383643e440a6002`.", ""]
for name, text in CASES.items():
    pair = {}
    for label, tool in (("old", OLD), ("new", NEW)):
        case = ROOT / "blacklist-cases" / name / label
        case.mkdir(parents=True, exist_ok=True)
        data = case / "bootconfig.in"
        data.write_text(text)
        argv = [str(tool), "merge-bootconfig", "--input", str(data), "--output", str(data)]
        run = subprocess.run(argv, capture_output=True, text=True)
        cat = subprocess.run(["cat", str(data)], capture_output=True, text=True, check=True)
        assert cat.stdout == data.read_text()
        pair[label] = {"cat_argv": ["cat", str(data)], "cat_exit_code": cat.returncode, "cat_stdout": cat.stdout, "cat_stderr": cat.stderr, "argv": argv, "exit_code": run.returncode, "stdout": run.stdout, "stderr": run.stderr, "output": data.read_text(), "input_preserved": data.read_text() == text}
        transcript += [f"## {name}: {label}", "", "Input:", "```text", text.rstrip("\n"), "```", "", "```console", "$ " + shlex.join(argv), f"[exit {run.returncode}]", "stdout:", run.stdout.rstrip("\n"), "stderr:", run.stderr.rstrip("\n"), "$ cat " + shlex.quote(str(data)), cat.stdout.rstrip("\n"), "```", ""]
    if name == "mixed-block":
        assert pair["old"]["exit_code"] == 0
        assert pair["new"]["exit_code"] != 0 and pair["new"]["input_preserved"]
    else:
        assert pair["old"]["exit_code"] == pair["new"]["exit_code"] == 0
        assert pair["old"]["output"] == pair["new"]["output"]
    results[name] = pair
payload = {"passed": True, "old_source": "09380d356886842b8987b602ed2ef7daaefec4f3", "new_source": "830ee97501b05adbf6dc76d13383643e440a6002", "tools": {label: {"path": str(tool), "sha256": hashlib.sha256(tool.read_bytes()).hexdigest()} for label, tool in (("old", OLD), ("new", NEW))}, "cases": results}
(ROOT / "blacklist-comparison.json").write_text(json.dumps(payload, indent=2) + "\n")
(ROOT / "blacklist-console.md").write_text("\n".join(transcript))
print("PASS: mixed block-form input rejected without changing its file; five supported cases byte-identical.")
