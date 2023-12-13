use serde::{Deserialize, Serialize};

use crate::shell::Shell;

use super::condition::Condition;

#[derive(Debug, Serialize, Deserialize)]
pub struct Envar {
    name: String,
    value: String,

    #[serde(default)]
    when: Condition,
}

impl Envar {
    pub fn compile(&self, target: Shell) -> Option<String> {
        if !self.when.eval(&target) {
            return None;
        }

        Some(match target {
            Shell::Fish => self.compile_fish(),
            shell if shell.posix_shell() => self.compile_posix(),
            Shell::PowerShell => self.compile_powershell(),
            _ => unreachable!("Shell::Any should be handled by the caller"),
        })
    }

    fn compile_fish(&self) -> String {
        format!("set -gx {} {}", self.name, self.value)
    }

    fn compile_posix(&self) -> String {
        format!("export {}=\"{}\"", self.name, self.value)
    }

    fn compile_powershell(&self) -> String {
        format!("$env:{} = \"{}\"", self.name, self.value)
    }
}
