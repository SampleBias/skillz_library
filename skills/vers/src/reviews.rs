//! Import review attestations from a caller-controlled trust boundary.
use crate::{check, hex, Check, Requirement, Status};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const REVIEW_IDS: [&str; 8] = [
    "G0-COVERAGE",
    "G2-SAFETY",
    "G3-CONTRACT",
    "G4-SECURITY",
    "G5-ARCHITECTURE",
    "G2-MIRI",
    "G3-FUZZ",
    "G3-COVERAGE",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Policy {
    version: String,
    implementers: Vec<String>,
    authorized_reviewers: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bundle {
    version: String,
    source_sha256: String,
    config_sha256: String,
    reviews: Vec<Review>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Review {
    id: String,
    reviewer: String,
    status: ReviewStatus,
    rationale: String,
    artifact: String,
    artifact_sha256: String,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ReviewStatus {
    Pass,
    Fail,
    NotApplicable,
}

fn external_file(path: &Path, root: &Path) -> Result<PathBuf, String> {
    let canonical =
        fs::canonicalize(path).map_err(|e| format!("cannot access {}: {e}", path.display()))?;
    if canonical.starts_with(root) || !canonical.is_file() {
        return Err(
            "review policy, bundle and artifacts must be regular files outside the candidate"
                .into(),
        );
    }
    Ok(canonical)
}

fn nonempty_unique(values: &[String]) -> bool {
    let mut seen = HashSet::new();
    !values.is_empty()
        && values
            .iter()
            .all(|v| !v.trim().is_empty() && v == v.trim() && seen.insert(v))
}

pub struct ReviewContext<'a> {
    pub root: &'a Path,
    pub out: &'a Path,
    pub policy_path: Option<&'a Path>,
    pub bundle_path: Option<&'a Path>,
    pub trusted: bool,
    pub source: Option<&'a str>,
    pub config: &'a str,
    pub conditional: [Requirement; 3],
}

/// Results remain blocked on any malformed or untrusted input; names are not authenticated.
pub fn load(context: ReviewContext<'_>) -> Vec<Check> {
    match load_inner(context) {
        Ok(checks) => checks,
        Err(reason) => REVIEW_IDS
            .iter()
            .map(|id| check(id, Status::Blocked, &reason))
            .collect(),
    }
}

fn load_inner(context: ReviewContext<'_>) -> Result<Vec<Check>, String> {
    let ReviewContext {
        root,
        out,
        policy_path,
        bundle_path,
        trusted,
        source,
        config,
        conditional,
    } = context;
    if !trusted {
        return Err("Review provenance has not been explicitly trusted by the operator".into());
    }
    let policy_path = external_file(
        policy_path.ok_or("No external review policy supplied")?,
        root,
    )?;
    let bundle_path = external_file(
        bundle_path.ok_or("No external review bundle supplied")?,
        root,
    )?;
    let policy_bytes = fs::read(&policy_path).map_err(|e| e.to_string())?;
    let bundle_bytes = fs::read(&bundle_path).map_err(|e| e.to_string())?;
    let policy: Policy =
        serde_json::from_slice(&policy_bytes).map_err(|e| format!("Invalid review policy: {e}"))?;
    let bundle: Bundle =
        serde_json::from_slice(&bundle_bytes).map_err(|e| format!("Invalid review bundle: {e}"))?;
    if policy.version != "1" || bundle.version != "1" {
        return Err("Unsupported review policy or bundle version".into());
    }
    if !nonempty_unique(&policy.implementers)
        || !nonempty_unique(&policy.authorized_reviewers)
        || policy
            .implementers
            .iter()
            .any(|v| policy.authorized_reviewers.contains(v))
    {
        return Err(
            "Policy needs distinct, nonempty implementer and authorized reviewer identities".into(),
        );
    }
    if source != Some(bundle.source_sha256.as_str()) || bundle.config_sha256 != config {
        return Err("Review candidate or configuration hash does not match".into());
    }
    let mut reviews = HashMap::new();
    for review in &bundle.reviews {
        if !REVIEW_IDS.contains(&review.id.as_str())
            || reviews.insert(review.id.as_str(), review).is_some()
        {
            return Err("Unknown or duplicate review ID".into());
        }
    }
    let artifact_root = bundle_path
        .parent()
        .ok_or("Review bundle has no parent directory")?;
    let mut checks = Vec::new();
    for (i, id) in REVIEW_IDS.iter().enumerate() {
        let Some(review) = reviews.get(id) else {
            checks.push(check(
                id,
                Status::Blocked,
                "Required review record is missing",
            ));
            continue;
        };
        if !policy.authorized_reviewers.contains(&review.reviewer)
            || policy.implementers.contains(&review.reviewer)
        {
            checks.push(check(
                id,
                Status::Blocked,
                "Reviewer is not authorized and independent according to the supplied policy",
            ));
            continue;
        }
        if review.rationale.trim().is_empty() {
            checks.push(check(id, Status::Blocked, "Review rationale is empty"));
            continue;
        }
        let rel = Path::new(&review.artifact);
        if rel.as_os_str().is_empty()
            || rel.components().any(|c| !matches!(c, Component::Normal(_)))
        {
            checks.push(check(
                id,
                Status::Blocked,
                "Artifact must be a relative path within the bundle directory",
            ));
            continue;
        }
        let artifact = match external_file(&artifact_root.join(rel), root) {
            Ok(path) if path.starts_with(artifact_root) => path,
            _ => {
                checks.push(check(
                    id,
                    Status::Blocked,
                    "Review artifact is missing or outside the bundle directory",
                ));
                continue;
            }
        };
        let artifact_bytes = match fs::read(&artifact) {
            Ok(bytes) if !bytes.is_empty() => bytes,
            _ => {
                checks.push(check(
                    id,
                    Status::Blocked,
                    "Review artifact is empty or unreadable",
                ));
                continue;
            }
        };
        if hex(&Sha256::digest(&artifact_bytes)) != review.artifact_sha256 {
            checks.push(check(
                id,
                Status::Blocked,
                "Review artifact hash does not match",
            ));
            continue;
        }
        let status = match review.status {
            ReviewStatus::Pass => Status::Pass,
            ReviewStatus::Fail => Status::Fail,
            ReviewStatus::NotApplicable
                if i >= 5 && conditional[i - 5] == Requirement::Conditional =>
            {
                Status::NotApplicable
            }
            ReviewStatus::NotApplicable => Status::Blocked,
        };
        let mut c = check(
            id,
            status,
            &format!(
                "Reviewer {}: {} (artifact SHA-256 {})",
                review.reviewer, review.rationale, review.artifact_sha256
            ),
        );
        c.evidence = Some("evidence/reviews.json".into());
        checks.push(c);
    }
    let record = serde_json::json!({
        "trust_notice": "Operator attested provenance and independence; identities and review quality are not authenticated by VERS.",
        "policy_sha256": hex(&Sha256::digest(&policy_bytes)),
        "bundle_sha256": hex(&Sha256::digest(&bundle_bytes)),
        "policy": serde_json::from_slice::<serde_json::Value>(&policy_bytes).map_err(|e| e.to_string())?,
        "bundle": serde_json::from_slice::<serde_json::Value>(&bundle_bytes).map_err(|e| e.to_string())?,
        "bundle_directory": artifact_root.display().to_string()
    });
    fs::write(
        out.join("evidence/reviews.json"),
        serde_json::to_vec_pretty(&record).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(checks)
}
