// Policy validation and conflict detection (FR-5.6)
//
// Detects and reports conflicting policy rules before code generation
// Provides clear error messages with resolution suggestions

use super::types::{PolicyConfig, PolicyRule};
use std::collections::{HashMap, HashSet};

/// Type of validation issue
#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    /// Critical error that prevents code generation
    Error,
    /// Warning that suggests potential issues
    Warning,
    /// Informational message
    Info,
}

/// Resolution suggestion for conflicts
#[derive(Debug, Clone)]
pub struct ResolutionSuggestion {
    pub description: String,
    pub action: String,
}

/// A single validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub issue_type: IssueType,
    pub message: String,
    pub details: Option<String>,
    pub suggestions: Vec<ResolutionSuggestion>,
    pub policy_indices: Vec<usize>,
}

impl ValidationIssue {
    fn error(
        message: String,
        details: Option<String>,
        suggestions: Vec<ResolutionSuggestion>,
        policy_indices: Vec<usize>,
    ) -> Self {
        Self {
            issue_type: IssueType::Error,
            message,
            details,
            suggestions,
            policy_indices,
        }
    }

    fn warning(
        message: String,
        details: Option<String>,
        suggestions: Vec<ResolutionSuggestion>,
        policy_indices: Vec<usize>,
    ) -> Self {
        Self {
            issue_type: IssueType::Warning,
            message,
            details,
            suggestions,
            policy_indices,
        }
    }

    fn info(message: String, details: Option<String>) -> Self {
        Self {
            issue_type: IssueType::Info,
            message,
            details,
            suggestions: Vec::new(),
            policy_indices: Vec::new(),
        }
    }
}

/// Complete validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
    pub has_errors: bool,
    pub has_warnings: bool,
}

impl ValidationReport {
    fn new() -> Self {
        Self {
            issues: Vec::new(),
            has_errors: false,
            has_warnings: false,
        }
    }

    fn add_issue(&mut self, issue: ValidationIssue) {
        match issue.issue_type {
            IssueType::Error => self.has_errors = true,
            IssueType::Warning => self.has_warnings = true,
            IssueType::Info => {}
        }
        self.issues.push(issue);
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        !self.has_errors
    }

    /// Get count of each issue type
    pub fn counts(&self) -> (usize, usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;
        let mut info = 0;

        for issue in &self.issues {
            match issue.issue_type {
                IssueType::Error => errors += 1,
                IssueType::Warning => warnings += 1,
                IssueType::Info => info += 1,
            }
        }

        (errors, warnings, info)
    }
}

/// Validate policy rules and detect conflicts (FR-5.6)
///
/// Detects:
/// 1. Direct conflicts (allowlist + denylist same value)
/// 2. Overlapping rate limits on same resource
/// 3. Multiple spending caps (warns to use most restrictive)
pub fn validate_policies(policy_config: &PolicyConfig) -> ValidationReport {
    let mut report = ValidationReport::new();
    let policies = &policy_config.policies;

    if policies.is_empty() {
        report.add_issue(ValidationIssue::info(
            "No policies defined".to_string(),
            Some("Policy file contains no policy rules".to_string()),
        ));
        return report;
    }

    // Validate individual policies first
    for (idx, policy) in policies.iter().enumerate() {
        if let Err(e) = policy.validate() {
            report.add_issue(ValidationIssue::error(
                format!("Invalid policy configuration at index #{}", idx),
                Some(e),
                vec![ResolutionSuggestion {
                    description: "Fix policy configuration".to_string(),
                    action: "Ensure all required fields are properly set with valid values"
                        .to_string(),
                }],
                vec![idx],
            ));
        }
    }

    // Check for allowlist/denylist conflicts
    detect_allowlist_denylist_conflicts(policies, &mut report);

    // Check for rate limit conflicts
    detect_rate_limit_conflicts(policies, &mut report);

    // Check for spending cap conflicts
    detect_spending_cap_conflicts(policies, &mut report);

    if !report.has_errors && !report.has_warnings {
        report.add_issue(ValidationIssue::info(
            "All policies valid".to_string(),
            Some(format!(
                "Validated {} policy rules with no conflicts",
                policies.len()
            )),
        ));
    }

    report
}

/// Detect allowlist and denylist conflicts (FR-5.6)
///
/// Checks if the same value appears in both allowlist and denylist for the same field
fn detect_allowlist_denylist_conflicts(policies: &[PolicyRule], report: &mut ValidationReport) {
    // Group policies by field
    let mut allowlists: HashMap<String, Vec<(usize, &[String])>> = HashMap::new();
    let mut denylists: HashMap<String, Vec<(usize, &[String])>> = HashMap::new();

    for (idx, policy) in policies.iter().enumerate() {
        match policy {
            PolicyRule::Allowlist { field, values } => {
                allowlists
                    .entry(field.clone())
                    .or_default()
                    .push((idx, values.as_slice()));
            }
            PolicyRule::Denylist { field, values } => {
                denylists
                    .entry(field.clone())
                    .or_default()
                    .push((idx, values.as_slice()));
            }
            _ => {}
        }
    }

    // Check for conflicts in each field
    for (field, allow_policies) in &allowlists {
        if let Some(deny_policies) = denylists.get(field) {
            // Check for overlapping values
            for (allow_idx, allow_values) in allow_policies {
                let allow_set: HashSet<_> = allow_values.iter().collect();

                for (deny_idx, deny_values) in deny_policies {
                    let deny_set: HashSet<_> = deny_values.iter().collect();

                    // Find intersection
                    let conflicts: Vec<_> = allow_set.intersection(&deny_set).collect();

                    if !conflicts.is_empty() {
                        let conflict_list: Vec<String> =
                            conflicts.iter().map(|s| s.to_string()).collect();

                        report.add_issue(ValidationIssue::error(
                            format!("CONFLICT: {} in both allowlist and denylist", field),
                            Some(format!(
                                "Conflicting values: {}\nPolicy indices: #{}, #{}",
                                conflict_list.join(", "),
                                allow_idx,
                                deny_idx
                            )),
                            vec![
                                ResolutionSuggestion {
                                    description: "Remove from denylist".to_string(),
                                    action: format!(
                                        "Remove {} from denylist policy (index #{})",
                                        conflict_list.join(", "),
                                        deny_idx
                                    ),
                                },
                                ResolutionSuggestion {
                                    description: "Remove from allowlist".to_string(),
                                    action: format!(
                                        "Remove {} from allowlist policy (index #{})",
                                        conflict_list.join(", "),
                                        allow_idx
                                    ),
                                },
                                ResolutionSuggestion {
                                    description: "Policy precedence (fail-fast)".to_string(),
                                    action:
                                        "Deny rules are evaluated first. If a value is in both, it will be denied."
                                            .to_string(),
                                },
                            ],
                            vec![*allow_idx, *deny_idx],
                        ));
                    }
                }
            }
        }
    }
}

/// Detect overlapping rate limits
fn detect_rate_limit_conflicts(policies: &[PolicyRule], report: &mut ValidationReport) {
    let rate_limits: Vec<(usize, &PolicyRule)> = policies
        .iter()
        .enumerate()
        .filter(|(_, p)| matches!(p, PolicyRule::RateLimit { .. }))
        .collect();

    if rate_limits.len() > 1 {
        let details: Vec<String> = rate_limits
            .iter()
            .map(|(idx, p)| match p {
                PolicyRule::RateLimit {
                    max_requests,
                    window_seconds,
                } => format!(
                    "Policy #{}: {} requests / {} seconds",
                    idx, max_requests, window_seconds
                ),
                _ => unreachable!(),
            })
            .collect();

        // Find most restrictive limit
        let most_restrictive = rate_limits
            .iter()
            .min_by_key(|(_, p)| match p {
                PolicyRule::RateLimit {
                    max_requests,
                    window_seconds,
                } => (*max_requests as f64 / *window_seconds as f64 * 1000.0) as u64,
                _ => unreachable!(),
            })
            .map(|(idx, _)| idx);

        report.add_issue(ValidationIssue::warning(
            "Multiple rate limits defined".to_string(),
            Some(format!(
                "Found {} rate limit policies:\n{}",
                rate_limits.len(),
                details.join("\n")
            )),
            vec![
                ResolutionSuggestion {
                    description: "Use most restrictive limit".to_string(),
                    action: if let Some(idx) = most_restrictive {
                        format!("Keep policy #{} (most restrictive) and remove others", idx)
                    } else {
                        "Remove duplicate rate limits".to_string()
                    },
                },
                ResolutionSuggestion {
                    description: "Keep all if intentional".to_string(),
                    action: "Multiple rate limits will all be enforced (most restrictive applies)"
                        .to_string(),
                },
            ],
            rate_limits.iter().map(|(idx, _)| *idx).collect(),
        ));
    }
}

/// Detect multiple spending caps
fn detect_spending_cap_conflicts(policies: &[PolicyRule], report: &mut ValidationReport) {
    let spending_caps: Vec<(usize, &PolicyRule)> = policies
        .iter()
        .enumerate()
        .filter(|(_, p)| matches!(p, PolicyRule::SpendingCap { .. }))
        .collect();

    if spending_caps.len() > 1 {
        let details: Vec<String> = spending_caps
            .iter()
            .map(|(idx, p)| match p {
                PolicyRule::SpendingCap {
                    max_amount,
                    currency,
                    window_seconds,
                } => format!(
                    "Policy #{}: {} {} / {} seconds",
                    idx, max_amount, currency, window_seconds
                ),
                _ => unreachable!(),
            })
            .collect();

        // Find most restrictive cap
        let most_restrictive = spending_caps
            .iter()
            .min_by(|(_, a), (_, b)| {
                let a_rate = match a {
                    PolicyRule::SpendingCap {
                        max_amount,
                        window_seconds,
                        ..
                    } => max_amount / *window_seconds as f64,
                    _ => unreachable!(),
                };
                let b_rate = match b {
                    PolicyRule::SpendingCap {
                        max_amount,
                        window_seconds,
                        ..
                    } => max_amount / *window_seconds as f64,
                    _ => unreachable!(),
                };
                a_rate
                    .partial_cmp(&b_rate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, _)| idx);

        report.add_issue(ValidationIssue::warning(
            "Multiple spending caps defined".to_string(),
            Some(format!(
                "Found {} spending cap policies:\n{}",
                spending_caps.len(),
                details.join("\n")
            )),
            vec![
                ResolutionSuggestion {
                    description: "Use most restrictive cap".to_string(),
                    action: if let Some(idx) = most_restrictive {
                        format!("Keep policy #{} (most restrictive) and remove others", idx)
                    } else {
                        "Remove duplicate spending caps".to_string()
                    },
                },
                ResolutionSuggestion {
                    description: "Keep all if intentional".to_string(),
                    action:
                        "Multiple spending caps will all be enforced (most restrictive applies)"
                            .to_string(),
                },
            ],
            spending_caps.iter().map(|(idx, _)| *idx).collect(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_policies() {
        let policy_config = PolicyConfig { policies: vec![] };
        let report = validate_policies(&policy_config);

        assert!(report.is_valid());
        assert_eq!(report.issues.len(), 1);
        assert_eq!(report.issues[0].issue_type, IssueType::Info);
    }

    #[test]
    fn test_allowlist_denylist_conflict() {
        let policy_config = PolicyConfig {
            policies: vec![
                PolicyRule::Allowlist {
                    field: "agent_id".to_string(),
                    values: vec!["agent-abc-123".to_string(), "agent-xyz-789".to_string()],
                },
                PolicyRule::Denylist {
                    field: "agent_id".to_string(),
                    values: vec!["agent-abc-123".to_string()],
                },
            ],
        };

        let report = validate_policies(&policy_config);

        assert!(!report.is_valid());
        assert!(report.has_errors);
        assert_eq!(
            report
                .issues
                .iter()
                .filter(|i| matches!(i.issue_type, IssueType::Error))
                .count(),
            1
        );

        let error = &report.issues[0];
        assert!(error.message.contains("CONFLICT"));
        assert!(error.message.contains("agent_id"));
        assert_eq!(error.suggestions.len(), 3);
    }

    #[test]
    fn test_no_conflict_different_fields() {
        let policy_config = PolicyConfig {
            policies: vec![
                PolicyRule::Allowlist {
                    field: "agent_id".to_string(),
                    values: vec!["agent-123".to_string()],
                },
                PolicyRule::Denylist {
                    field: "wallet_address".to_string(),
                    values: vec!["wallet-456".to_string()],
                },
            ],
        };

        let report = validate_policies(&policy_config);

        assert!(report.is_valid());
        assert!(!report.has_errors);
    }

    #[test]
    fn test_multiple_rate_limits_warning() {
        let policy_config = PolicyConfig {
            policies: vec![
                PolicyRule::RateLimit {
                    max_requests: 100,
                    window_seconds: 3600,
                },
                PolicyRule::RateLimit {
                    max_requests: 50,
                    window_seconds: 3600,
                },
            ],
        };

        let report = validate_policies(&policy_config);

        assert!(report.is_valid()); // Warnings don't make it invalid
        assert!(report.has_warnings);
        assert!(report
            .issues
            .iter()
            .any(|i| i.message.contains("Multiple rate limits")));
    }

    #[test]
    fn test_multiple_spending_caps_warning() {
        let policy_config = PolicyConfig {
            policies: vec![
                PolicyRule::SpendingCap {
                    max_amount: 10.0,
                    currency: "USDC".to_string(),
                    window_seconds: 86400,
                },
                PolicyRule::SpendingCap {
                    max_amount: 5.0,
                    currency: "USDC".to_string(),
                    window_seconds: 86400,
                },
            ],
        };

        let report = validate_policies(&policy_config);

        assert!(report.is_valid());
        assert!(report.has_warnings);
        assert!(report
            .issues
            .iter()
            .any(|i| i.message.contains("Multiple spending caps")));
    }

    #[test]
    fn test_validation_report_counts() {
        let mut report = ValidationReport::new();

        report.add_issue(ValidationIssue::error(
            "Error 1".to_string(),
            None,
            vec![],
            vec![],
        ));
        report.add_issue(ValidationIssue::warning(
            "Warning 1".to_string(),
            None,
            vec![],
            vec![],
        ));
        report.add_issue(ValidationIssue::info("Info 1".to_string(), None));

        let (errors, warnings, info) = report.counts();
        assert_eq!(errors, 1);
        assert_eq!(warnings, 1);
        assert_eq!(info, 1);

        assert!(!report.is_valid());
        assert!(report.has_errors);
        assert!(report.has_warnings);
    }
}
