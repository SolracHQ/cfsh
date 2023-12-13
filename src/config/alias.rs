use serde::{Deserialize, Serialize};

use crate::shell::Shell;

use super::condition::Condition;

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    name: String,
    command: String,

    #[serde(default)]
    when: Condition,
}

impl Alias {
    pub fn compile(&self, target: Shell) -> Option<String> {
        if !self.when.eval(&target) {
            return None;
        }

        Some(match target {
            shell if shell.posix_shell() => self.compile_posix(),
            Shell::PowerShell => self.compile_powershell(),
            _ => unreachable!("Shell::Any should be handled by the caller"),
        })
    }

    fn compile_posix(&self) -> String {
        format!("alias {}='{}'", self.name, self.command)
    }

    fn compile_powershell(&self) -> String {
        let uuid = uuid::Uuid::new_v4();

        format!(
            "function {uuid} {{ {command} $args }} New-Alias -Name {name} -Value {uuid} -Force -Option AllScope",
            command = self.command,
            name = self.name
        )
    }
}
