use std::process::Command;
use std::fs;

#[test]
fn test_cli_help() {
    let output = Command::new("target/debug/qclang")
        .arg("--help")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("FLAGS:"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("target/debug/qclang")
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("qclang"));
}

#[test]
fn test_compile_valid_program() {
    // Create test file
    let test_code = "fn main() -> int { qubit q = |0>; q = H(q); cbit r = measure(q); return 0; }";
    fs::write("test_cli.qc", test_code).unwrap();
    
    let output = Command::new("target/debug/qclang")
        .arg("test_cli.qc")
        .output()
        .expect("Failed to execute command");
    
    // Clean up
    let _ = fs::remove_file("test_cli.qc");
    let _ = fs::remove_file("test_cli.qasm");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SUCCESS"));
    assert!(stdout.contains("Qubits:"));
}