//! Shared test utilities for building real binaries in unit tests

use std::process::Command;
use std::fs;
use tempfile::NamedTempFile;

/// Build a real binary for testing using the specified compiler
/// 
/// This function compiles a simple C program and returns the binary data.
/// Used across unit tests to ensure we test with real binaries, not fake headers.
/// 
/// # Arguments
/// * `compiler` - The compiler to use (e.g., "gcc", "x86_64-w64-mingw32-gcc")
/// 
/// # Returns
/// * `Ok(Vec<u8>)` - The compiled binary data
/// * `Err(String)` - Error message if compilation fails
pub fn build_real_test_binary(compiler: &str) -> Result<Vec<u8>, String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("Test\n");
    return 0;
}
"#;
    let source = NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(source.path(), code).map_err(|e| e.to_string())?;
    
    let output = NamedTempFile::new().map_err(|e| e.to_string())?;
    
    let result = Command::new(compiler)
        .arg(source.path())
        .arg("-o")
        .arg(output.path())
        .arg("-static")
        .output()
        .map_err(|e| format!("Compiler '{}' not found: {}", compiler, e))?;
    
    if !result.status.success() {
        return Err(format!(
            "Compilation with '{}' failed: {}",
            compiler,
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    
    fs::read(output.path()).map_err(|e| e.to_string())
}
