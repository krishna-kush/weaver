use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to a test binary in the fixtures directory
pub fn get_test_binary_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/binaries");
    path.push(name);
    path
}

/// Get the path to a source file in the fixtures directory
pub fn get_test_source_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/source");
    path.push(name);
    path
}

/// Compile a C source file to a binary
pub fn compile_c_binary(source: &str, output: &str) -> Result<(), String> {
    let source_path = get_test_source_path(source);
    let output_path = get_test_binary_path(output);
    
    // Ensure source exists
    if !source_path.exists() {
        return Err(format!("Source file not found: {:?}", source_path));
    }
    
    // Create output directory if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create output dir: {}", e))?;
    }
    
    // Compile with gcc
    let result = Command::new("gcc")
        .arg("-static")
        .arg(&source_path)
        .arg("-o")
        .arg(&output_path)
        .output()
        .map_err(|e| format!("Failed to run gcc: {}", e))?;
    
    if !result.status.success() {
        return Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    
    Ok(())
}

/// Build a simple test binary from inline C code
pub fn build_test_binary_from_code(code: &str, output: &str) -> Result<PathBuf, String> {
    let output_path = get_test_binary_path(output);
    
    // Create output directory if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create output dir: {}", e))?;
    }
    
    // Write code to temp file
    let temp_source = format!("/tmp/{}.c", output);
    fs::write(&temp_source, code).map_err(|e| format!("Failed to write temp source: {}", e))?;
    
    // Compile
    let result = Command::new("gcc")
        .arg("-static")
        .arg(&temp_source)
        .arg("-o")
        .arg(&output_path)
        .output()
        .map_err(|e| format!("Failed to run gcc: {}", e))?;
    
    if !result.status.success() {
        return Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    
    Ok(output_path)
}

/// Ensure test_base and test_overload binaries exist (build if needed)
pub fn ensure_basic_test_binaries() -> Result<(), String> {
    let base_path = get_test_binary_path("test_base");
    let overload_path = get_test_binary_path("test_overload");
    
    // Build test_base if it doesn't exist
    if !base_path.exists() {
        println!("Building test_base from source...");
        compile_c_binary("test_base.c", "test_base")?;
    }
    
    // Build test_overload if it doesn't exist
    if !overload_path.exists() {
        println!("Building test_overload from source...");
        compile_c_binary("test_overload.c", "test_overload")?;
    }
    
    Ok(())
}

/// Ensure test_x86_64 binary exists (build if needed)
pub fn ensure_x86_64_binary() -> Result<(), String> {
    let x86_64_path = get_test_binary_path("test_x86_64");
    
    if !x86_64_path.exists() {
        println!("Building test_x86_64 from source...");
        
        // Create a simple test binary
        let code = r#"
#include <stdio.h>
int main() {
    printf("Test binary\n");
    return 0;
}
"#;
        build_test_binary_from_code(code, "test_x86_64")?;
    }
    
    Ok(())
}

/// Ensure ARM binary exists (build if needed)
pub fn ensure_arm_binary() -> Result<(), String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("ARM Test\n");
    return 0;
}
"#;
    build_cross_compiled_binary("arm-linux-gnueabi-gcc", "test_arm", code).map(|_| ())
}

/// Ensure ARM64 binary exists (build if needed)
pub fn ensure_arm64_binary() -> Result<(), String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("ARM64 Test\n");
    return 0;
}
"#;
    build_cross_compiled_binary("aarch64-linux-gnu-gcc", "test_arm64", code).map(|_| ())
}

/// Ensure MIPS binary exists (build if needed)
pub fn ensure_mips_binary() -> Result<(), String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("MIPS Test\n");
    return 0;
}
"#;
    build_cross_compiled_binary("mips-linux-gnu-gcc", "test_mips", code).map(|_| ())
}

/// Ensure Windows 64-bit binary exists (build if needed)
pub fn ensure_win64_binary() -> Result<(), String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("Windows 64 Test\n");
    return 0;
}
"#;
    build_cross_compiled_binary("x86_64-w64-mingw32-gcc", "test_win64.exe", code).map(|_| ())
}

/// Ensure Windows 32-bit binary exists (build if needed)
pub fn ensure_win32_binary() -> Result<(), String> {
    let code = r#"
#include <stdio.h>
int main() {
    printf("Windows 32 Test\n");
    return 0;
}
"#;
    build_cross_compiled_binary("i686-w64-mingw32-gcc", "test_win32.exe", code).map(|_| ())
}

/// Check if cross-host testing is enabled
pub fn is_cross_host_testing_enabled() -> bool {
    std::env::var("WEAVER_ENABLE_CROSS_HOST_TESTING")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false)
}

/// Get the current host OS
pub fn get_host_os() -> &'static str {
    #[cfg(target_os = "linux")]
    return "linux";
    
    #[cfg(target_os = "windows")]
    return "windows";
    
    #[cfg(target_os = "macos")]
    return "macos";
    
    "unknown"
}

/// Get the current host architecture
pub fn get_host_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "x86_64";
    
    #[cfg(target_arch = "x86")]
    return "x86";
    
    #[cfg(target_arch = "aarch64")]
    return "aarch64";
    
    #[cfg(target_arch = "arm")]
    return "arm";
    
    "unknown"
}

/// Check if we should skip a test based on cross-host testing settings
pub fn should_skip_cross_host_test(target_os: &str, target_arch: &str) -> bool {
    if is_cross_host_testing_enabled() {
        return false; // Cross-host testing enabled, don't skip
    }
    
    // Check if target matches host
    let host_os = get_host_os();
    let host_arch = get_host_arch();
    
    if target_os != host_os || target_arch != host_arch {
        return true; // Skip cross-host tests when disabled
    }
    
    false
}

/// Build a cross-compiled binary and return path + data
pub fn build_cross_compiled_binary(
    compiler: &str,
    name: &str,
    code: &str,
) -> Result<(PathBuf, Vec<u8>), String> {
    // Check if compiler is available
    let check = Command::new(compiler)
        .arg("--version")
        .output();
    
    if check.is_err() {
        return Err(format!("Cross-compiler '{}' not found", compiler));
    }
    
    let source_path = get_test_source_path(&format!("{}.c", name));
    fs::write(&source_path, code)
        .map_err(|e| format!("Failed to write source: {}", e))?;
    
    let binary_path = get_test_binary_path(name);
    
    let output = Command::new(compiler)
        .arg(&source_path)
        .arg("-o")
        .arg(&binary_path)
        .arg("-static")
        .output()
        .map_err(|e| format!("Failed to compile: {}", e))?;
    
    if !output.status.success() {
        return Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    let data = fs::read(&binary_path)
        .map_err(|e| format!("Failed to read binary: {}", e))?;
    
    // Cleanup source
    fs::remove_file(source_path).ok();
    
    Ok((binary_path, data))
}

/// Extract task ID from API response
pub fn extract_task_id(json: &serde_json::Value) -> Option<String> {
    json.get("task_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Check if a binary is executable
pub fn is_executable(path: &PathBuf) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
    }
    false
}

/// Load binary data from fixtures
pub fn load_test_binary(name: &str) -> Option<Vec<u8>> {
    let path = get_test_binary_path(name);
    fs::read(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths() {
        let bin_path = get_test_binary_path("test");
        assert!(bin_path.to_string_lossy().contains("tests/fixtures/binaries"));
        
        let src_path = get_test_source_path("test.c");
        assert!(src_path.to_string_lossy().contains("tests/fixtures/source"));
    }
    
    #[test]
    fn test_build_from_code() {
        let code = r#"
#include <stdio.h>
int main() {
    printf("Hello\n");
    return 0;
}
"#;
        let result = build_test_binary_from_code(code, "test_inline");
        assert!(result.is_ok(), "Should build binary from inline code");
        
        let path = result.unwrap();
        assert!(path.exists(), "Binary should exist");
        
        // Cleanup
        let _ = fs::remove_file(path);
    }
}
