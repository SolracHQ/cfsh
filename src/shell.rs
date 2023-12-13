use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell
}

impl Shell {
    pub fn posix_shell(&self) -> bool {
        matches!(self, Shell::Bash | Shell::Zsh | Shell::Fish)
    }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "powershell" => Ok(Shell::PowerShell),
            "fish" => Ok(Shell::Fish),
            _ => Err(format!("Invalid shell: {}", s)),
        }
    }
}