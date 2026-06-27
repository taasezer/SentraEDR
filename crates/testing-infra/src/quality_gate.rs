#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualityGateCommand {
    pub name: String,
    pub program: String,
    pub args: Vec<String>,
    pub destructive: bool,
}

impl QualityGateCommand {
    pub fn new(
        name: impl Into<String>,
        program: impl Into<String>,
        args: Vec<&str>,
        destructive: bool,
    ) -> Self {
        Self {
            name: name.into(),
            program: program.into(),
            args: args.into_iter().map(str::to_string).collect(),
            destructive,
        }
    }

    pub fn command_line(&self) -> String {
        if self.args.is_empty() {
            return self.program.clone();
        }

        format!("{} {}", self.program, self.args.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualityGateError {
    DestructiveCommandRejected(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualityGateSet {
    pub commands: Vec<QualityGateCommand>,
}

impl QualityGateSet {
    pub fn try_new(commands: Vec<QualityGateCommand>) -> Result<Self, QualityGateError> {
        let set = Self { commands };
        set.validate_safe()?;
        Ok(set)
    }

    pub fn default_workspace() -> Self {
        Self::try_new(vec![
            QualityGateCommand::new(
                "format",
                "cargo",
                vec!["fmt", "--all", "--", "--check"],
                false,
            ),
            QualityGateCommand::new(
                "clippy",
                "cargo",
                vec![
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--",
                    "-D",
                    "warnings",
                ],
                false,
            ),
            QualityGateCommand::new(
                "workspace-tests",
                "cargo",
                vec!["test", "--workspace"],
                false,
            ),
            QualityGateCommand::new(
                "architecture-validation",
                "powershell",
                vec![
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    r"tools\validate-architecture.ps1",
                ],
                false,
            ),
            QualityGateCommand::new(
                "agent-dry-run",
                "cargo",
                vec!["run", "-p", "sentra-agent"],
                false,
            ),
        ])
        .expect("default quality gates must be non-destructive")
    }

    pub fn validate_safe(&self) -> Result<(), QualityGateError> {
        for command in &self.commands {
            if command.destructive {
                return Err(QualityGateError::DestructiveCommandRejected(
                    command.name.clone(),
                ));
            }
        }

        Ok(())
    }
}
