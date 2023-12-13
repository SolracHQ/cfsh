use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OsFamily {
    Windows,
    Unix,
    MacOs,
    Linux,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum Condition {
    // Check if OS is the given one
    Is(OsFamily),
    /// Check if binary is in PATH
    InPath(String),
    /// Check if environment variable is set
    IsSet(String),
    /// Check if environment variable is set and equals the given value
    IsSetTo(String, String),
    /// Check if environment variable is not set
    Shell(crate::shell::Shell),
    /// And condition - all conditions must be true
    And(Vec<Condition>),
    /// Or condition - any condition must be true
    Or(Vec<Condition>),
    /// Not condition - condition must be false
    Not(Box<Condition>),
    #[default]
    /// Always true
    True,
}

impl Condition {
    pub fn eval(&self, target: &crate::shell::Shell) -> bool {
        match self {
            Condition::Is(os) => get_os_family().contains(os),
            Condition::InPath(bin) => which::which(bin).is_ok(),
            Condition::IsSet(var) => std::env::var(var).is_ok(),
            Condition::IsSetTo(var, val) => std::env::var(var).map(|v| v == *val).unwrap_or(false),
            Condition::Shell(shell) => shell == target,
            Condition::And(conds) => conds.iter().all(|cond| cond.eval(target)),
            Condition::Or(conds) => conds.iter().any(|cond| cond.eval(target)),
            Condition::Not(cond) => !cond.eval(target),
            Condition::True => true,
        }
    }
}

// Helper functions

/// Get os family of the current OS at compile time
fn get_os_family() -> &'static [OsFamily] {
    #[cfg(target_os = "windows")]
    {
        &[OsFamily::Windows]
    }
    #[cfg(target_os = "macos")]
    {
        &[OsFamily::MacOs, OsFamily::Unix]
    }
    #[cfg(target_os = "linux")]
    {
        &[OsFamily::Linux, OsFamily::Unix]
    }
}