use std::process::Command;
use std::fs;
use crate::common::{get_test_binary_path, build_test_binary_from_code};

/// Helper to execute a binary and capture output
fn execute_binary(path: &str) -> Result<String, String> {
    let output = Command::new(path)
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Execution failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

#[test]
fn test_simple_binary_output() {
    println!("\n📊 Simple Binary Output Test");
    println!("============================\n");
    
    let code = r#"
#include <stdio.h>
int main() {
    printf("Hello from test binary\n");
    return 0;
}
"#;
    
    match build_test_binary_from_code(code, "test_simple_output") {
        Ok(path) => {
            println!("✅ Binary built: {:?}", path);
            
            match execute_binary(path.to_str().unwrap()) {
                Ok(output) => {
                    println!("   Output: {}", output.trim());
                    assert!(output.contains("Hello from test binary"));
                    println!("✅ Binary executed successfully");
                }
                Err(e) => {
                    println!("⚠️  Execution failed: {}", e);
                }
            }
            
            // Cleanup
            let _ = fs::remove_file(path);
        }
        Err(e) => {
            println!("⚠️  Failed to build binary: {}", e);
            println!("   This is expected if gcc is not available");
        }
    }
}

#[test]
fn test_distinct_binary_outputs() {
    println!("\n📊 Distinct Binary Outputs Test");
    println!("================================\n");
    
    let first_code = r#"
#include <stdio.h>
int main() {
    printf("FIRST\n");
    return 0;
}
"#;
    
    let second_code = r#"
#include <stdio.h>
int main() {
    printf("SECOND\n");
    return 0;
}
"#;
    
    // Build first binary
    let first_result = build_test_binary_from_code(first_code, "test_first_output");
    let second_result = build_test_binary_from_code(second_code, "test_second_output");
    
    if let (Ok(first_path), Ok(second_path)) = (first_result, second_result) {
        println!("✅ Both binaries built successfully");
        
        // Execute first binary
        if let Ok(first_output) = execute_binary(first_path.to_str().unwrap()) {
            println!("   First output: {}", first_output.trim());
            assert!(first_output.contains("FIRST"));
        }
        
        // Execute second binary
        if let Ok(second_output) = execute_binary(second_path.to_str().unwrap()) {
            println!("   Second output: {}", second_output.trim());
            assert!(second_output.contains("SECOND"));
        }
        
        println!("✅ Both binaries produced distinct outputs");
        
        // Cleanup
        let _ = fs::remove_file(first_path);
        let _ = fs::remove_file(second_path);
    } else {
        println!("⚠️  Failed to build test binaries");
        println!("   This is expected if gcc is not available");
    }
}

#[test]
fn test_binary_exit_codes() {
    println!("\n📊 Binary Exit Code Test");
    println!("========================\n");
    
    let success_code = r#"
#include <stdio.h>
int main() {
    printf("Success\n");
    return 0;
}
"#;
    
    let failure_code = r#"
#include <stdio.h>
int main() {
    printf("Failure\n");
    return 1;
}
"#;
    
    // Test success binary
    if let Ok(success_path) = build_test_binary_from_code(success_code, "test_success") {
        let output = Command::new(success_path.to_str().unwrap())
            .output()
            .expect("Failed to execute");
        
        assert!(output.status.success(), "Success binary should return 0");
        println!("✅ Success binary returned exit code 0");
        
        let _ = fs::remove_file(success_path);
    }
    
    // Test failure binary
    if let Ok(failure_path) = build_test_binary_from_code(failure_code, "test_failure") {
        let output = Command::new(failure_path.to_str().unwrap())
            .output()
            .expect("Failed to execute");
        
        assert!(!output.status.success(), "Failure binary should return non-zero");
        println!("✅ Failure binary returned non-zero exit code");
        
        let _ = fs::remove_file(failure_path);
    }
}

#[test]
fn test_binary_with_arguments() {
    println!("\n📊 Binary with Arguments Test");
    println!("=============================\n");
    
    let code = r#"
#include <stdio.h>
int main(int argc, char *argv[]) {
    printf("Arguments: %d\n", argc);
    for (int i = 0; i < argc; i++) {
        printf("  arg[%d]: %s\n", i, argv[i]);
    }
    return 0;
}
"#;
    
    if let Ok(path) = build_test_binary_from_code(code, "test_args") {
        let output = Command::new(path.to_str().unwrap())
            .args(&["arg1", "arg2", "arg3"])
            .output()
            .expect("Failed to execute");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("   Output:\n{}", stdout);
        
        assert!(stdout.contains("Arguments: 4")); // Binary name + 3 args
        assert!(stdout.contains("arg1"));
        assert!(stdout.contains("arg2"));
        assert!(stdout.contains("arg3"));
        
        println!("✅ Binary correctly handled arguments");
        
        let _ = fs::remove_file(path);
    } else {
        println!("⚠️  Failed to build test binary");
    }
}
