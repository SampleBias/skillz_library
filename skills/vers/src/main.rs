mod reviews;
mod schema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::{DirEntry, WalkDir};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Config {
    version: String,
    timeout_seconds: u64,
    runtime: Vec<RuntimeCase>,
    checks: CheckPolicy,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: "0.1".into(),
            timeout_seconds: 120,
            runtime: vec![],
            checks: CheckPolicy::default(),
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Requirement {
    Required,
    Conditional,
}
#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct CheckPolicy {
    format: Requirement,
    compile: Requirement,
    clippy: Requirement,
    tests: Requirement,
    build: Requirement,
    runtime: Requirement,
    security: Requirement,
    miri: Requirement,
    fuzz: Requirement,
    coverage: Requirement,
}
impl Default for CheckPolicy {
    fn default() -> Self {
        Self {
            format: Requirement::Required,
            compile: Requirement::Required,
            clippy: Requirement::Required,
            tests: Requirement::Required,
            build: Requirement::Required,
            runtime: Requirement::Required,
            security: Requirement::Required,
            miri: Requirement::Conditional,
            fuzz: Requirement::Conditional,
            coverage: Requirement::Conditional,
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeCase {
    name: String,
    argv: Vec<String>,
    expected_exit: i32,
    stdout: String,
    stderr: String,
}
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Status {
    Pass,
    Fail,
    Blocked,
    NotApplicable,
}
#[derive(Serialize)]
struct Check {
    id: String,
    status: Status,
    command: Vec<String>,
    exit_code: Option<i32>,
    evidence: Option<String>,
    detail: String,
}
#[derive(Serialize)]
struct Report {
    skill: &'static str,
    version: &'static str,
    mode: String,
    revision: Option<String>,
    git_status: Option<String>,
    source_sha256: Option<String>,
    config_sha256: String,
    source_unchanged: bool,
    toolchain: Vec<Check>,
    checks: Vec<Check>,
    findings: Vec<Finding>,
    verdict: Status,
    next_action: &'static str,
}
#[derive(Serialize)]
struct Finding {
    id: String,
    severity: &'static str,
    category: &'static str,
    file: Option<String>,
    line: Option<u64>,
    description: String,
    reproduction: String,
    recommended_action: &'static str,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("vers: {e}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut mode = "standard";
    let mut execute = false;
    let mut root = env::current_dir()?;
    let mut report_arg = None;
    let mut review_policy = None;
    let mut review_bundle = None;
    let mut trust_reviews = false;
    let mut args = env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--diff" | "--full" | "--strict" => {
                if mode != "standard" {
                    return Err("choose only one mode".into());
                }
                mode = match arg.as_str() {
                    "--diff" => "diff",
                    "--full" => "full",
                    _ => "strict",
                };
            }
            "--execute" => execute = true,
            "--review-policy" => {
                review_policy = Some(PathBuf::from(
                    args.next().ok_or("--review-policy requires a path")?,
                ))
            }
            "--review-bundle" => {
                review_bundle = Some(PathBuf::from(
                    args.next().ok_or("--review-bundle requires a path")?,
                ))
            }
            "--trust-reviews" => trust_reviews = true,
            "--root" => root = PathBuf::from(args.next().ok_or("--root requires a path")?),
            "--report" => {
                let path = if args.peek().is_some_and(|x| !x.starts_with('-')) {
                    args.next()
                } else {
                    None
                };
                report_arg = Some(path);
            }
            "--help" | "-h" => {
                println!("Usage: vers [--root PATH] [--diff|--full|--strict] [--execute] [--review-policy PATH --review-bundle PATH --trust-reviews]\n       vers [--root PATH] --report [PATH]\n\n--execute confirms candidate execution is acceptable. --trust-reviews confirms the operator has independently verified review provenance and independence. Report paths are printed on stdout.");
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }
    root = fs::canonicalize(root)?;
    if let Some(path) = report_arg {
        let path = match path {
            Some(path) => PathBuf::from(path),
            None => {
                let dir = root.join(".vers/reports");
                let mut paths = fs::read_dir(dir)?
                    .filter_map(Result::ok)
                    .map(|e| e.path().join("report.json"))
                    .filter(|p| p.is_file())
                    .collect::<Vec<_>>();
                paths.sort();
                paths.pop().ok_or("no report found")?
            }
        };
        print!("{}", fs::read_to_string(path)?);
        return Ok(());
    }
    for relative in [".vers", ".vers/reports", ".vers/config.toml"] {
        if fs::symlink_metadata(root.join(relative)).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!("refusing symlink at {relative}").into());
        }
    }
    let config_path = root.join(".vers/config.toml");
    let config_text = if config_path.exists() {
        fs::read_to_string(&config_path)?
    } else {
        String::new()
    };
    let cfg: Config = if config_text.is_empty() {
        Config::default()
    } else {
        toml::from_str(&config_text)?
    };
    if cfg.version != "0.1" || !(1..=3600).contains(&cfg.timeout_seconds) {
        return Err(
            "configuration requires version = \"0.1\" and timeout_seconds in 1..=3600".into(),
        );
    }
    if [
        cfg.checks.format,
        cfg.checks.compile,
        cfg.checks.clippy,
        cfg.checks.tests,
        cfg.checks.build,
        cfg.checks.runtime,
        cfg.checks.security,
    ]
    .iter()
    .any(|v| *v != Requirement::Required)
    {
        return Err(
            "format, compile, clippy, tests, build, runtime and security must remain required"
                .into(),
        );
    }
    for c in &cfg.runtime {
        if c.name.trim().is_empty() || c.argv.is_empty() || c.argv.iter().any(|x| x.is_empty()) {
            return Err("each runtime case needs a name and nonempty argv elements".into());
        }
    }
    let run_id = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
        std::process::id()
    );
    let out_dir = root.join(".vers/reports").join(&run_id);
    fs::create_dir_all(out_dir.join("evidence"))?;
    let temp = env::temp_dir().join(format!("vers-{run_id}"));
    fs::create_dir(&temp)?;
    let _guard = Cleanup(temp.clone());
    let snapshot = temp.join("candidate");
    let mut checks = Vec::new();
    let mut tools = Vec::new();
    let mut findings = Vec::new();
    let git_revision = capture(&root, "git", &["rev-parse", "HEAD"]);
    let git_status = capture(
        &root,
        "git",
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            ".",
            ":(exclude).vers/reports",
        ],
    );
    let git_top = capture(&root, "git", &["rev-parse", "--show-toplevel"]);
    let revision = git_revision
        .as_ref()
        .filter(|o| o.status.success())
        .map(|o| text(&o.stdout).trim().to_owned());
    let git_status = git_status
        .as_ref()
        .filter(|o| o.status.success())
        .map(|o| text(&o.stdout));
    let root_is_git = git_top
        .as_ref()
        .filter(|o| o.status.success())
        .and_then(|o| fs::canonicalize(text(&o.stdout).trim()).ok())
        .is_some_and(|p| p == root);
    let tracked_target = capture(&root, "git", &["ls-files", "--", "target", ".vers/reports"]);
    let excluded_tracked = tracked_target
        .as_ref()
        .is_none_or(|o| !o.status.success() || !o.stdout.is_empty());
    let source_identity = match snapshot_copy(&root, &snapshot) {
        Ok(hash) => Some(hash),
        Err(e) => {
            checks.push(check(
                "G0-SNAPSHOT",
                Status::Blocked,
                &format!("Snapshot failed: {e}"),
            ));
            None
        }
    };
    if source_identity.is_some() {
        let good = revision.is_some()
            && git_status.is_some()
            && root_is_git
            && !excluded_tracked
            && root.join("Cargo.toml").is_file();
        checks.push(check("G0-DISCOVERY", if good {Status::Pass} else {Status::Blocked}, if good {"Git root and Cargo manifest discovered; candidate snapshot recorded."} else {"Need a Git-root Cargo workspace, readable Git state, and no tracked files in excluded target or .vers/reports."}));
    }
    if source_identity.is_some() {
        let snapshot_config = snapshot.join(".vers/config.toml");
        let snapshot_text = match fs::read(&snapshot_config) {
            Ok(bytes) => Some(bytes),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Some(Vec::new()),
            Err(_) => None,
        };
        let matches = snapshot_text.as_deref() == Some(config_text.as_bytes());
        checks.push(check(
            "G0-CONFIG",
            if matches {
                Status::Pass
            } else {
                Status::Blocked
            },
            if matches {
                "Loaded configuration matches candidate snapshot."
            } else {
                "Configuration changed while the candidate was being snapshotted."
            },
        ));
    }
    for (id, cmd, argv) in [
        ("TOOL-RUSTC", "rustc", vec!["-vV"]),
        ("TOOL-CARGO", "cargo", vec!["--version"]),
        ("TOOL-GIT", "git", vec!["--version"]),
    ] {
        let output = capture(&root, cmd, &argv);
        let status = if output.as_ref().is_some_and(|o| o.status.success()) {
            Status::Pass
        } else {
            Status::Blocked
        };
        let mut c = check(
            id,
            status,
            &output
                .as_ref()
                .map(|o| text(&o.stdout))
                .unwrap_or_else(|| "Tool unavailable".into()),
        );
        c.command = std::iter::once(cmd.to_owned())
            .chain(argv.iter().map(|x| x.to_string()))
            .collect();
        c.exit_code = output.and_then(|o| o.status.code());
        tools.push(c);
    }
    let commands: [(&str, &[&str]); 5] = [
        ("G1-FMT", &["fmt", "--all", "--", "--check"]),
        (
            "G1-CHECK",
            &["check", "--offline", "--workspace", "--all-targets"],
        ),
        (
            "G1-CLIPPY",
            &[
                "clippy",
                "--offline",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("G3-TEST", &["test", "--offline", "--workspace"]),
        ("G1-BUILD", &["build", "--offline", "--workspace"]),
    ];
    let ready =
        execute && source_identity.is_some() && checks.iter().all(|c| c.status == Status::Pass);
    let target = temp.join("cargo-target");
    for (id, argv) in commands {
        let command: Vec<String> = std::iter::once("cargo".to_owned())
            .chain(argv.iter().map(|s| s.to_string()))
            .collect();
        let c = if ready {
            execute_case(
                id,
                &command,
                &snapshot,
                &target,
                &out_dir,
                cfg.timeout_seconds,
                None,
            )?
        } else {
            blocked_command(
                id,
                command,
                "Execution not authorized or discovery incomplete",
            )
        };
        if c.status == Status::Fail {
            findings.push(finding(&c, "correctness"));
        }
        checks.push(c);
    }
    if cfg.runtime.is_empty() {
        checks.push(check("G6-RUNTIME", Status::Blocked, "No runtime contracts configured; compilation and tests do not establish real behavior."));
    }
    for (idx, case) in cfg.runtime.iter().enumerate() {
        let id = format!("G6-RUNTIME-{idx:03}");
        let c = if ready {
            execute_case(
                &id,
                &case.argv,
                &snapshot,
                &target,
                &out_dir,
                cfg.timeout_seconds,
                Some(case),
            )?
        } else {
            blocked_command(
                &id,
                case.argv.clone(),
                "Execution not authorized or discovery incomplete",
            )
        };
        if c.status == Status::Fail {
            findings.push(finding(&c, "runtime"));
        }
        checks.push(c);
    }
    // Review authenticity and isolation are the operator's responsibility; names are not signatures.
    let config_sha256 = hex(&Sha256::digest(config_text.as_bytes()));
    checks.extend(reviews::load(reviews::ReviewContext {
        root: &root,
        out: &out_dir,
        policy_path: review_policy.as_deref(),
        bundle_path: review_bundle.as_deref(),
        trusted: trust_reviews,
        source: source_identity.as_deref(),
        config: &config_sha256,
        conditional: [cfg.checks.miri, cfg.checks.fuzz, cfg.checks.coverage],
    }));
    for review in checks
        .iter()
        .filter(|c| reviews::REVIEW_IDS.contains(&c.id.as_str()) && c.status == Status::Fail)
    {
        findings.push(finding(review, "review"));
    }
    if mode == "strict" {
        checks.push(check(
            "STRICT-COVERAGE",
            Status::Blocked,
            "Strict-mode additional checks are not implemented.",
        ));
    }
    // Recheck both source and copy: builds may mutate either. Generated target/report files are excluded.
    let after_source = hash_tree(&root).ok();
    let after_snapshot = hash_tree(&snapshot).ok();
    let unchanged = source_identity.is_some()
        && source_identity == after_source
        && source_identity == after_snapshot;
    checks.push(check("G7-IDENTITY", if unchanged {Status::Pass} else {Status::Blocked}, if unchanged {"Source and snapshot match the recorded identity after execution."} else {"Source or snapshot changed, or identity could not be checked; evidence cannot approve the candidate."}));
    let verdict = if checks.iter().any(|c| c.status == Status::Fail) {
        Status::Fail
    } else if checks.iter().any(|c| c.status == Status::Blocked)
        || tools.iter().any(|c| c.status == Status::Blocked)
    {
        Status::Blocked
    } else {
        Status::Pass
    };
    let report = Report {
        skill: "vers",
        version: VERSION,
        mode: mode.into(),
        revision,
        git_status,
        source_sha256: source_identity,
        config_sha256: hex(&Sha256::digest(config_text.as_bytes())),
        source_unchanged: unchanged,
        toolchain: tools,
        checks,
        findings,
        verdict,
        next_action: match verdict {
            Status::Pass => "ACCEPT",
            Status::Fail => "TRIAGE_AND_REPAIR",
            _ => "COMPLETE_VERIFICATION",
        },
    };
    validate(&report)?;
    schema::validate_report(&serde_json::to_value(&report)?)?;
    fs::write(
        out_dir.join("report.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    let mut summary = format!("# VERS verification\n\nVerdict: **{:?}**\n\nRevision: `{}`\n\nSource SHA-256: `{}`\n\nMode: `{}`\n\n", report.verdict, report.revision.as_deref().unwrap_or("unavailable"), report.source_sha256.as_deref().unwrap_or("unavailable"), report.mode);
    for c in &report.checks {
        summary.push_str(&format!(
            "- `{}`: **{:?}** — {}\n",
            c.id,
            c.status,
            c.detail.replace('\n', " ")
        ));
    }
    fs::write(out_dir.join("summary.md"), summary)?;
    println!("{}", out_dir.join("report.json").display());
    drop(_guard);
    std::process::exit(match verdict {
        Status::Pass => 0,
        Status::Fail => 1,
        _ => 2,
    });
}
fn capture(root: &Path, cmd: &str, args: &[&str]) -> Option<Output> {
    Command::new(cmd).args(args).current_dir(root).output().ok()
}
fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
fn check(id: &str, status: Status, detail: &str) -> Check {
    Check {
        id: id.into(),
        status,
        command: vec![],
        exit_code: None,
        evidence: None,
        detail: detail.into(),
    }
}
fn blocked_command(id: &str, command: Vec<String>, reason: &str) -> Check {
    let mut c = check(id, Status::Blocked, reason);
    c.command = command;
    c
}
fn finding(c: &Check, category: &'static str) -> Finding {
    Finding { id: format!("{}-FAIL", c.id), severity: "high", category, file: None, line: None, description: c.detail.clone(), reproduction: format!("See {} in report.json", c.id), recommended_action: "Inspect the evidence, reproduce the failure, and repair in a separate implementation step." }
}
fn execute_case(
    id: &str,
    argv: &[String],
    root: &Path,
    target: &Path,
    out: &Path,
    timeout: u64,
    contract: Option<&RuntimeCase>,
) -> io::Result<Check> {
    let mut cmd = Command::new("timeout");
    cmd.arg("--kill-after=5s")
        .arg(format!("{timeout}s"))
        .args(argv)
        .current_dir(root)
        .env("CARGO_NET_OFFLINE", "true")
        .env("CARGO_TARGET_DIR", target)
        .env("CARGO_TERM_COLOR", "never");
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return Ok(blocked_command(
                id,
                argv.to_vec(),
                &format!("Could not start timeout/command: {e}"),
            ))
        }
    };
    let code = output.status.code();
    let log_rel = format!("evidence/{id}.log");
    let mut log = Vec::new();
    log.extend_from_slice(b"--- stdout ---\n");
    log.extend_from_slice(&output.stdout);
    log.extend_from_slice(b"\n--- stderr ---\n");
    log.extend_from_slice(&output.stderr);
    fs::write(out.join(&log_rel), log)?;
    let (status, detail) = if code == Some(124) || code == Some(137) || code.is_none() {
        (
            Status::Blocked,
            "Command timed out or was terminated".to_owned(),
        )
    } else if code == Some(127) {
        (
            Status::Blocked,
            "Command or required tool was unavailable".to_owned(),
        )
    } else if contract.is_none()
        && (text(&output.stderr).contains("no such command:")
            || text(&output.stderr).contains("no matching package named")
            || text(&output.stderr).contains("failed to download")
            || text(&output.stderr).contains("component 'rustfmt'")
            || text(&output.stderr).contains("component 'clippy'"))
    {
        (
            Status::Blocked,
            "Cargo command or offline dependency unavailable; inspect evidence".to_owned(),
        )
    } else if let Some(case) = contract {
        if code == Some(case.expected_exit)
            && output.stdout == case.stdout.as_bytes()
            && output.stderr == case.stderr.as_bytes()
        {
            (
                Status::Pass,
                format!(
                    "Runtime contract '{}' matched exit code, stdout and stderr",
                    case.name
                ),
            )
        } else {
            (
                Status::Fail,
                format!(
                    "Runtime contract '{}' did not match expected exit code, stdout or stderr",
                    case.name
                ),
            )
        }
    } else if output.status.success() {
        (Status::Pass, "Command exited successfully".into())
    } else {
        (Status::Fail, "Command failed; inspect evidence and distinguish unavailable prerequisites from code defects".into())
    };
    Ok(Check {
        id: id.into(),
        status,
        command: argv.to_vec(),
        exit_code: code,
        evidence: Some(log_rel),
        detail,
    })
}
fn include(e: &DirEntry) -> bool {
    if e.depth() == 0 {
        return true;
    }
    let p = e.path();
    if e.depth() == 1 && (e.file_name() == ".git" || e.file_name() == "target") {
        return false;
    }
    !(e.depth() == 2
        && p.parent()
            .and_then(Path::file_name)
            .is_some_and(|v| v == ".vers")
        && e.file_name() == "reports")
}
fn entries(root: &Path) -> Result<Vec<DirEntry>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(include)
    {
        let e = entry?;
        if e.depth() == 0 {
            continue;
        }
        if !e.file_type().is_file() && !e.file_type().is_dir() {
            return Err(format!(
                "unsupported symlink or special file: {}",
                e.path().display()
            )
            .into());
        }
        result.push(e);
    }
    result.sort_by(|a, b| a.path().cmp(b.path()));
    Ok(result)
}
fn hash_tree(root: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut h = Sha256::new();
    for e in entries(root)? {
        let rel = e.path().strip_prefix(root)?;
        let path = rel.to_str().ok_or("non-UTF-8 path unsupported")?.as_bytes();
        h.update((path.len() as u64).to_be_bytes());
        h.update(path);
        h.update(if e.file_type().is_dir() { b"d" } else { b"f" });
        if e.file_type().is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                h.update((e.metadata()?.permissions().mode() & 0o777).to_be_bytes());
            }
            let mut f = fs::File::open(e.path())?;
            let len = f.metadata()?.len();
            h.update(len.to_be_bytes());
            let mut buf = [0u8; 8192];
            loop {
                let n = f.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                h.update(&buf[..n]);
            }
        }
    }
    Ok(hex(&h.finalize()))
}
fn snapshot_copy(root: &Path, dest: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let before = hash_tree(root)?;
    fs::create_dir(dest)?;
    for e in entries(root)? {
        let out = dest.join(e.path().strip_prefix(root)?);
        if e.file_type().is_dir() {
            fs::create_dir_all(out)?;
        } else {
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(e.path(), out)?;
        }
    }
    let copied = hash_tree(dest)?;
    let after = hash_tree(root)?;
    if before != copied || before != after {
        return Err("candidate changed while snapshot was made".into());
    }
    Ok(before)
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn validate(r: &Report) -> Result<(), Box<dyn std::error::Error>> {
    if r.checks.is_empty()
        || (r.verdict == Status::Pass
            && (!r.source_unchanged
                || r.checks
                    .iter()
                    .any(|c| c.status == Status::Blocked || c.status == Status::Fail)))
    {
        return Err("invalid report invariants".into());
    }
    let mut ids = std::collections::HashSet::new();
    if r.checks.iter().any(|c| !ids.insert(&c.id)) {
        return Err("duplicate check IDs".into());
    }
    Ok(())
}
struct Cleanup(PathBuf);
impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
