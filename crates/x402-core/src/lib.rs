// x402-core: Core library for x402 protocol development toolkit
//
// This library will provide:
// - Runtime management (deno_core integration)
// - Protocol handlers
// - Configuration management
// - Policy enforcement
// - Testing framework

pub mod policy;
pub mod testing;

pub use policy::{
    validate_policies, IssueType, PolicyAction, PolicyConfig, PolicyRule, PolicyType,
    ResolutionSuggestion, ValidationIssue, ValidationReport,
    // Runtime types
    PolicyEngine, PolicyDecision, Request, RuntimePolicy,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
