#!/usr/bin/env python3
"""Exercise VERS against isolated, intentionally broken local Rust projects."""
import json
import hashlib
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


REVIEW_IDS = ["G0-COVERAGE", "G2-SAFETY", "G3-CONTRACT", "G4-SECURITY", "G5-ARCHITECTURE", "G2-MIRI", "G3-FUZZ", "G3-COVERAGE"]


def review_fixture(tmp, report):
    directory = Path(tmp) / "independent-reviews"
    directory.mkdir()
    artifact = directory / "review.txt"
    artifact.write_text("Synthetic fixture evidence, not a real review.\n")
    digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
    policy = directory / "policy.json"
    policy.write_text(json.dumps({"version": "1", "implementers": ["developer"], "authorized_reviewers": ["reviewer"]}))
    bundle = directory / "bundle.json"
    data = {"version": "1", "source_sha256": report["source_sha256"], "config_sha256": report["config_sha256"],
            "reviews": [{"id": ident, "reviewer": "reviewer", "status": "PASS" if i < 5 else "NOT_APPLICABLE",
                         "rationale": "Synthetic assessment and applicability rationale", "artifact": "review.txt",
                         "artifact_sha256": digest} for i, ident in enumerate(REVIEW_IDS)]}
    bundle.write_text(json.dumps(data))
    return policy, bundle, data, artifact


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
        # A separately supplied policy and synthetic evidence can complete a review.
        policy, bundle, review, artifact = review_fixture(tmp, report)
        review_args = ("--execute", "--review-policy", str(policy), "--review-bundle", str(bundle), "--trust-reviews")
        p, approved, _ = run(binary, cli, *review_args)
        assert p.returncode == 0 and approved["verdict"] == "PASS"
        assert status(approved, "G2-SAFETY") == "PASS" and status(approved, "G2-MIRI") == "NOT_APPLICABLE"
        # Without explicit provenance acceptance, a bundle cannot produce approval.
        p, no_trust, _ = run(binary, cli, *review_args[:-1])
        assert p.returncode == 2 and status(no_trust, "G2-SAFETY") == "BLOCKED"
        # Evidence cannot assert an unapproved identity, waive a required review, or omit a check.
        review["reviews"][1]["reviewer"] = "developer"
        bundle.write_text(json.dumps(review))
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G2-SAFETY") == "BLOCKED"
        review["reviews"][1]["reviewer"] = "reviewer"
        review["reviews"][1]["status"] = "NOT_APPLICABLE"
        bundle.write_text(json.dumps(review))
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G2-SAFETY") == "BLOCKED"
        review["reviews"][1]["status"] = "FAIL"
        removed = review["reviews"].pop()
        bundle.write_text(json.dumps(review))
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 1 and status(rejected, "G2-SAFETY") == "FAIL" and status(rejected, "G3-COVERAGE") == "BLOCKED"
        review["reviews"][1]["status"] = "PASS"
        review["reviews"].append(removed)
        bundle.write_text(json.dumps(review))
        artifact.write_text("changed after review")
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G2-SAFETY") == "BLOCKED"
        artifact.write_text("Synthetic fixture evidence, not a real review.\n")
        review["source_sha256"] = "0" * 64
        bundle.write_text(json.dumps(review))
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G2-SAFETY") == "BLOCKED"
        review["source_sha256"] = report["source_sha256"]
        bundle.write_text(json.dumps(review))
        p, strict, _ = run(binary, cli, *review_args, "--strict")
        assert p.returncode == 2 and status(strict, "STRICT-COVERAGE") == "BLOCKED"
        # Duplicate IDs and policy files under the candidate are rejected.
        review["reviews"].append(dict(review["reviews"][0]))
        bundle.write_text(json.dumps(review))
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G0-COVERAGE") == "BLOCKED"
        review["reviews"].pop()
        bundle.write_text(json.dumps(review))
        inside_policy = cli / ".vers/reports/policy.json"
        inside_policy.write_text(policy.read_text())
        p, rejected, _ = run(binary, cli, "--execute", "--review-policy", str(inside_policy), "--review-bundle", str(bundle), "--trust-reviews")
        assert p.returncode == 2 and status(rejected, "G0-COVERAGE") == "BLOCKED"
        # A changed candidate invalidates previously reviewed source identity.
        candidate_file = cli / "src/main.rs"
        original_source = candidate_file.read_text()
        candidate_file.write_text("// Candidate changed.\n" + original_source)
        p, rejected, _ = run(binary, cli, *review_args)
        assert p.returncode == 2 and status(rejected, "G0-COVERAGE") == "BLOCKED"
        candidate_file.write_text(original_source)
        display = cmd([str(binary), "--report", str(report_path)])
        assert json.loads(display.stdout) == report
        assert json.loads(cmd([str(binary), "--root", str(cli), "--report"]).stdout) == rejected
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
        # A check explicitly configured as required cannot be marked not applicable.
        required = project(tmp, "required", "cli")
        with (required / ".vers/config.toml").open("a") as f:
            f.write('\n[checks]\nmiri = "required"\n')
        p, preliminary, _ = run(binary, required, "--execute")
        assert p.returncode == 2 and preliminary["verdict"] == "BLOCKED"
        evidence_parent = Path(tmp) / "required-evidence"
        evidence_parent.mkdir()
        required_policy, required_bundle, required_review, _ = review_fixture(evidence_parent, preliminary)
        required_args = ("--execute", "--review-policy", str(required_policy), "--review-bundle", str(required_bundle), "--trust-reviews")
        p, rejected, _ = run(binary, required, *required_args)
        assert p.returncode == 2 and status(rejected, "G2-MIRI") == "BLOCKED"
        required_review["reviews"][5]["status"] = "PASS"
        required_bundle.write_text(json.dumps(required_review))
        p, accepted, _ = run(binary, required, *required_args)
        assert p.returncode == 0 and status(accepted, "G2-MIRI") == "PASS"
        required_review["config_sha256"] = "0" * 64
        required_bundle.write_text(json.dumps(required_review))
        p, rejected, _ = run(binary, required, *required_args)
        assert p.returncode == 2 and status(rejected, "G2-MIRI") == "BLOCKED"
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
