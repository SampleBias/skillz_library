#!/usr/bin/env python3
"""Run the demo in temporary Git repositories and compare stable report fields."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
DEMO = Path(__file__).resolve().parent
VERIFY = ROOT / "skills/vers/scripts/verify.sh"
REVIEW_IDS = ["G0-COVERAGE", "G2-SAFETY", "G3-CONTRACT", "G4-SECURITY", "G5-ARCHITECTURE", "G2-MIRI", "G3-FUZZ", "G3-COVERAGE"]


def command(args, cwd=None):
    return subprocess.run(args, cwd=cwd, text=True, capture_output=True, check=True)


def project(parent, name):
    path = parent / name
    shutil.copytree(DEMO / "crate", path, ignore=shutil.ignore_patterns("target"))
    command(["git", "init", "-q"], path)
    command(["git", "add", "."], path)
    command(["git", "-c", "user.name=Demo", "-c", "user.email=demo@example.invalid", "commit", "-qm", "fixture"], path)
    return path


def run(path, *args):
    result = subprocess.run([str(VERIFY), "--root", str(path), "--execute", *args], text=True, capture_output=True)
    if not result.stdout.strip():
        raise RuntimeError(f"No report (exit {result.returncode}): {result.stderr}")
    report_path = Path(result.stdout.strip())
    report = json.loads(report_path.read_text())
    if "--validate-schema" in sys.argv:
        import jsonschema
        schema = json.loads((ROOT / "skills/vers/schemas/report.schema.json").read_text())
        jsonschema.Draft202012Validator.check_schema(schema)
        jsonschema.Draft202012Validator(schema).validate(report)
    expected_exit = {"PASS": 0, "FAIL": 1, "BLOCKED": 2}[report["verdict"]]
    if result.returncode != expected_exit:
        raise AssertionError((result.returncode, report["verdict"], result.stderr))
    for check in report["checks"]:
        if check["evidence"] and not (report_path.parent / check["evidence"]).is_file():
            raise AssertionError(f"missing evidence for {check['id']}")
    return report


def compare(name, report):
    expected = json.loads((DEMO / "expected" / f"{name}.json").read_text())
    statuses = {check["id"]: check["status"] for check in report["checks"]}
    actual = {"verdict": report["verdict"], "next_action": report["next_action"],
              "checks": {ident: statuses.get(ident) for ident in expected["checks"]}}
    if actual != expected:
        raise AssertionError(f"{name}: expected {expected}, got {actual}")
    print(f"{name}: {report['verdict']}")


def synthetic_reviews(parent, report):
    evidence = parent / "synthetic-reviews"
    evidence.mkdir()
    artifact = evidence / "review.txt"
    artifact.write_text("Fabricated fixture data. This is not an independent review.\n")
    digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
    policy = evidence / "policy.json"
    policy.write_text(json.dumps({"version": "1", "implementers": ["fixture-author"], "authorized_reviewers": ["fixture-reviewer"]}))
    bundle = evidence / "bundle.json"
    bundle.write_text(json.dumps({"version": "1", "source_sha256": report["source_sha256"], "config_sha256": report["config_sha256"],
        "reviews": [{"id": ident, "reviewer": "fixture-reviewer", "status": "PASS" if index < 5 else "NOT_APPLICABLE",
                     "rationale": "Fabricated test record; not evidence of an actual review.", "artifact": "review.txt", "artifact_sha256": digest}
                    for index, ident in enumerate(REVIEW_IDS)]}))
    return policy, bundle


def main():
    if set(sys.argv[1:]) - {"--validate-schema"}:
        raise SystemExit("usage: python3 examples/demo/run.py [--validate-schema]")
    if "--validate-schema" in sys.argv:
        import jsonschema
        schema = json.loads((ROOT / "skills/qc/schemas/report.schema.json").read_text())
        jsonschema.Draft202012Validator.check_schema(schema)
        validator = jsonschema.Draft202012Validator(schema, format_checker=jsonschema.FormatChecker())
        examples = [ROOT / "skills/qc/examples/checkpoint.example.json", *sorted((DEMO / "expected").glob("qc-*.json"))]
        for example in examples:
            validator.validate(json.loads(example.read_text()))
    for name, decision in (("blocked", "BLOCKED"), ("fail", "REJECT")):
        qc_example = json.loads((DEMO / "expected" / f"qc-{name}.json").read_text())
        if qc_example["decision"] != decision or qc_example["score"] is not None:
            raise AssertionError(f"unexpected illustrative QC {name} decision")
    with tempfile.TemporaryDirectory(prefix="skillz-demo-") as temporary:
        parent = Path(temporary)
        good = project(parent, "good")
        blocked = run(good)
        compare("blocked", blocked)
        policy, bundle = synthetic_reviews(parent, blocked)
        compare("pass", run(good, "--review-policy", str(policy), "--review-bundle", str(bundle), "--trust-reviews"))
        broken_runtime = project(parent, "broken-runtime")
        source = broken_runtime / "src/main.rs"
        source.write_text(source.read_text().replace("usage: greet NAME", "incorrect help"))
        compare("fail-runtime", run(broken_runtime))
        broken_test = project(parent, "broken-test")
        source = broken_test / "src/main.rs"
        source.write_text(source.read_text().replace('format!("hello {name}")', 'format!("goodbye {name}")'))
        compare("fail-test", run(broken_test))
    print("PASS uses fabricated review records; it is not qualifying independent verification.")


if __name__ == "__main__":
    main()
