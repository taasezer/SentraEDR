use testing_infra::quality_gate::{QualityGateCommand, QualityGateSet};

#[test]
fn default_quality_gates_are_in_required_order() {
    let gates = QualityGateSet::default_workspace();
    let names: Vec<&str> = gates
        .commands
        .iter()
        .map(|gate| gate.name.as_str())
        .collect();

    assert_eq!(
        names,
        vec![
            "format",
            "clippy",
            "workspace-tests",
            "architecture-validation",
            "agent-dry-run"
        ]
    );
}

#[test]
fn default_quality_gates_include_required_commands() {
    let gates = QualityGateSet::default_workspace();
    let commands: Vec<String> = gates
        .commands
        .iter()
        .map(|gate| gate.command_line())
        .collect();

    assert!(
        commands
            .iter()
            .any(|cmd| cmd == "cargo fmt --all -- --check")
    );
    assert!(
        commands
            .iter()
            .any(|cmd| cmd == "cargo clippy --workspace --all-targets -- -D warnings")
    );
    assert!(commands.iter().any(|cmd| cmd == "cargo test --workspace"));
    assert!(
        commands
            .iter()
            .any(|cmd| cmd.contains(r"tools\validate-architecture.ps1"))
    );
    assert!(
        commands
            .iter()
            .any(|cmd| cmd == "cargo run -p sentra-agent")
    );
}

#[test]
fn default_quality_gates_are_non_destructive() {
    let gates = QualityGateSet::default_workspace();

    assert!(gates.commands.iter().all(|gate| !gate.destructive));
    assert!(gates.validate_safe().is_ok());
}

#[test]
fn destructive_quality_gate_is_rejected() {
    let gates = QualityGateSet::try_new(vec![QualityGateCommand::new(
        "delete",
        "Remove-Item",
        vec!["-Recurse", r"C:\"],
        true,
    )]);

    assert!(gates.is_err());
}
