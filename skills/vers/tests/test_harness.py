#!/usr/bin/env python3
"""Exercise VERS against isolated, intentionally broken local Rust projects."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

SKILL = Path(__file__).resolve().parents[1]


def cmd(args, cwd=None):
    return subprocess.run(args, cwd=cwd, text=True, capture_output=True, check=True)


def project(tmp, name, source):
    path = Path(tmp) / name
    shutil.copytree(SKILL / "examples" / source, path)
    cmd(["git", "init", "-q"], path)
    cmd(["git", "add", "."], path)
    cmd(["git", "-c", "user.name=VERS Test", "-c", "user.email=vers@example.invalid", "commit", "-qm", "fixture"], path)
    return path


def run(binary, path, *args):
    p = subprocess.run([str(binary), "--root", str(path), *args], text=True, capture_output=True)
    assert p.stdout.strip(), (p.returncode, p.stderr)
    report_path = Path(p.stdout.strip())
    report = json.loads(report_path.read_text())
    assert report["skill"] == "vers" and report["revision"]
    for c in report["checks"]:
        if c["evidence"]:
            assert (report_path.parent / c["evidence"]).is_file(), c
    return p, report, report_path


def status(report, ident):
    return next(c["status"] for c in report["checks"] if c["id"] == ident)


def main():
    cmd(["cargo", "build", "--offline", "--locked", "--manifest-path", str(SKILL / "Cargo.toml")])
    binary = SKILL / "target" / "debug" / ("vers.exe" if os.name == "nt" else "vers")
    with tempfile.TemporaryDirectory(prefix="vers-tests-") as tmp:
        cli = project(tmp, "cli", "cli")
        p, report, _ = run(binary, cli)
        assert p.returncode == 2 and report["verdict"] == "BLOCKED"
        assert status(report, "G1-CHECK") == "BLOCKED"
        p, report, report_path = run(binary, cli, "--execute", "--full")
        assert p.returncode == 2 and report["mode"] == "full" and report["source_unchanged"]
        assert status(report, "G1-CHECK") == "PASS" and status(report, "G3-TEST") == "PASS"
        assert status(report, "G6-RUNTIME-000") == "PASS" and status(report, "G6-RUNTIME-001") == "PASS"
        assert status(report, "G2-SAFETY") == "BLOCKED" and report["verdict"] == "BLOCKED"
        display = cmd([str(binary), "--report", str(report_path)])
        assert json.loads(display.stdout) == report
        assert json.loads(cmd([str(binary), "--root", str(cli), "--report"]).stdout) == report
        # Tests compile and pass, but the independently expected CLI output is wrong.
        main_rs = cli / "src/main.rs"
        main_rs.write_text(main_rs.read_text().replace("usage: greet NAME", "incorrect help"))
        p, report, _ = run(binary, cli, "--execute", "--diff")
        assert p.returncode == 1 and report["verdict"] == "FAIL" and report["mode"] == "diff"
        assert status(report, "G3-TEST") == "PASS" and status(report, "G6-RUNTIME-000") == "FAIL"
        assert report["git_status"] and report["findings"]
        # Actual regression test detects seeded incorrect arithmetic.
        lib = project(tmp, "lib", "library")
        with (lib / "src/lib.rs").open("a") as f:
            f.write('\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn increment_contract() {\n        assert_eq!(super::increment(1), Some(2));\n    }\n}\n')
        path = lib / "src/lib.rs"
        path.write_text(path.read_text().replace("value.checked_add(1)", "value.checked_add(2)"))
        p, report, _ = run(binary, lib, "--execute")
        assert p.returncode == 1 and status(report, "G3-TEST") == "FAIL"
        # A nonexistent executable must block, not pass or masquerade as a code failure.
        missing = project(tmp, "missing", "cli")
        config = missing / ".vers/config.toml"
        config.write_text('version = "0.1"\ntimeout_seconds = 30\n[[runtime]]\nname = "missing"\nargv = ["vers-deliberately-nonexistent-binary"]\nexpected_exit = 0\nstdout = ""\nstderr = ""\n')
        p, report, _ = run(binary, missing, "--execute", "--strict")
        assert p.returncode == 2 and report["verdict"] == "BLOCKED"
        assert status(report, "G6-RUNTIME-000") == "BLOCKED" and status(report, "STRICT-COVERAGE") == "BLOCKED"
        # Missing contracts cannot become a successful verification.
        empty = project(tmp, "empty", "library")
        (empty / ".vers/config.toml").write_text('version = "0.1"\nruntime = []\n')
        p, report, _ = run(binary, empty, "--execute")
        assert p.returncode == 2 and status(report, "G6-RUNTIME") == "BLOCKED"
        # Reject malformed configuration rather than silently accepting misspelled requirements.
        (empty / ".vers/config.toml").write_text('version = "0.1"\nrequired_securty = false\n')
        p = subprocess.run([str(binary), "--root", str(empty)], capture_output=True, text=True)
        assert p.returncode == 2 and not p.stdout.strip() and "unknown field" in p.stderr
        (empty / ".vers/config.toml").write_text('version = "0.1"\n[checks]\nsecurity = "conditional"\n')
        p = subprocess.run([str(binary), "--root", str(empty)], capture_output=True, text=True)
        assert p.returncode == 2 and not p.stdout.strip() and "must remain required" in p.stderr
        # Symlinks in the candidate prevent an incomplete snapshot from being approved.
        symlink = project(tmp, "symlink", "cli")
        (symlink / "source-link").symlink_to("src/main.rs")
        p, report, _ = run(binary, symlink)
        assert p.returncode == 2 and status(report, "G0-SNAPSHOT") == "BLOCKED"
    print("VERS harness integration tests passed")


if __name__ == "__main__":
    main()
