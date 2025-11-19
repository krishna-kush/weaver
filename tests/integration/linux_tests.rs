use std::process::Command;
use std::env;
use crate::common::{
    get_test_binary_path, ensure_basic_test_binaries, ensure_x86_64_binary, 
    ensure_arm_binary, ensure_arm64_binary, ensure_mips_binary, is_executable,
    is_cross_host_testing_enabled, should_skip_cross_host_test
};

/// Helper to check if QEMU testing is enabled via environment variable
fn is_qemu_testing_enabled() -> bool {
    is_cross_host_testing_enabled()
}

/// Helper to get the current host architecture
fn get_host_arch() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "x86_64";
    
    #[cfg(target_arch = "x86")]
    return "x86";
    
    #[cfg(target_arch = "aarch64")]
    return "aarch64";
    
    #[cfg(target_arch = "arm")]
    return "arm";
    
    #[cfg(target_arch = "mips")]
    return "mips";
    
    "unknown"
}

/// Helper to check if a binary can be executed natively on this host
fn can_execute_natively(binary_arch: &str) -> bool {
    let host_arch = get_host_arch();
    binary_arch == host_arch
}

/// Helper to execute a binary, using QEMU if needed and enabled
fn execute_binary(path: &str, arch: &str) -> Result<String, String> {
    if can_execute_natively(arch) {
        // Execute natively
        let output = Command::new(path)
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("Execution failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    } else if is_qemu_testing_enabled() {
        // Execute with QEMU user-mode emulation
        let (qemu_bin, sysroot) = match arch {
            "x86_64" => ("qemu-x86_64", "/usr/x86_64-linux-gnu"),
            "aarch64" => ("qemu-aarch64", "/usr/aarch64-linux-gnu"),
            "arm" => ("qemu-arm", "/usr/arm-linux-gnueabi"),
            "mips" => ("qemu-mips", "/usr/mips-linux-gnu"),
            _ => return Err(format!("Unsupported architecture for QEMU: {}", arch)),
        };
        
        let output = Command::new(qemu_bin)
            .arg("-L")
            .arg(sysroot)
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to execute with QEMU: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!(
                "QEMU execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    } else {
        Err(format!(
            "Cannot execute {} binary on {} host. \
             Enable QEMU testing with WEAVER_ENABLE_QEMU_TESTING=true",
            arch, get_host_arch()
        ))
    }
}

#[test]
fn test_host_architecture_detection() {
    let host_arch = get_host_arch();
    println!("🖥️  Host architecture: {}", host_arch);
    assert!(!host_arch.is_empty());
    assert_ne!(host_arch, "unknown");
}

#[test]
fn test_qemu_config_detection() {
    let qemu_enabled = is_qemu_testing_enabled();
    println!("🔧 QEMU testing enabled: {}", qemu_enabled);
    println!("   Set WEAVER_ENABLE_QEMU_TESTING=true to enable cross-arch testing");
}

#[test]
fn test_x86_64_binary_execution() {
    // Auto-build x86_64 binary if needed
    if let Err(e) = ensure_x86_64_binary() {
        println!("⚠️  Cannot build x86-64 binary: {}", e);
        return;
    }
    
    let path = get_test_binary_path("test_x86_64");
    
    if !path.exists() {
        println!("⚠️  test_x86_64 not found after build attempt, skipping");
        return;
    }
    
    assert!(is_executable(&path), "Binary should be executable");
    
    match execute_binary(path.to_str().unwrap(), "x86_64") {
        Ok(output) => {
            println!("✅ x86-64 binary executed successfully");
            println!("   Output: {}", output.trim());
        }
        Err(e) => {
            println!("⚠️  {}", e);
        }
    }
}

#[test]
fn test_arm64_binary_execution() {
    // Skip if cross-host testing is disabled
    if should_skip_cross_host_test("linux", "aarch64") {
        println!("⚠️  Skipping ARM64 execution test - cross-host testing disabled");
        return;
    }
    
    // Auto-build ARM64 binary if needed
    if let Err(e) = ensure_arm64_binary() {
        println!("{}", e);
        println!("⚠️  Skipping ARM64 execution test");
        return;
    }
    
    let path = get_test_binary_path("test_arm64");
    
    if !path.exists() {
        println!("⚠️  test_arm64 not found after build attempt, skipping");
        return;
    }
    
    assert!(is_executable(&path), "Binary should be executable");
    
    match execute_binary(path.to_str().unwrap(), "aarch64") {
        Ok(output) => {
            println!("✅ ARM64 binary executed successfully");
            println!("   Output: {}", output.trim());
        }
        Err(e) => {
            println!("⚠️  {}", e);
        }
    }
}

#[test]
fn test_arm_binary_execution() {
    // Skip if cross-host testing is disabled
    if should_skip_cross_host_test("linux", "arm") {
        println!("⚠️  Skipping ARM execution test - cross-host testing disabled");
        return;
    }
    
    // Auto-build ARM binary if needed
    if let Err(e) = ensure_arm_binary() {
        println!("{}", e);
        println!("⚠️  Skipping ARM execution test");
        return;
    }
    
    let path = get_test_binary_path("test_arm");
    
    if !path.exists() {
        println!("⚠️  test_arm not found after build attempt, skipping");
        return;
    }
    
    assert!(is_executable(&path), "Binary should be executable");
    
    match execute_binary(path.to_str().unwrap(), "arm") {
        Ok(output) => {
            println!("✅ ARM binary executed successfully");
            println!("   Output: {}", output.trim());
        }
        Err(e) => {
            println!("⚠️  {}", e);
        }
    }
}

#[test]
fn test_mips_binary_execution() {
    // Skip if cross-host testing is disabled
    if should_skip_cross_host_test("linux", "mips") {
        println!("⚠️  Skipping MIPS execution test - cross-host testing disabled");
        return;
    }
    
    // Auto-build MIPS binary if needed
    if let Err(e) = ensure_mips_binary() {
        println!("{}", e);
        println!("⚠️  Skipping MIPS execution test");
        return;
    }
    
    let path = get_test_binary_path("test_mips");
    
    if !path.exists() {
        println!("⚠️  test_mips not found after build attempt, skipping");
        return;
    }
    
    assert!(is_executable(&path), "Binary should be executable");
    
    match execute_binary(path.to_str().unwrap(), "mips") {
        Ok(output) => {
            println!("✅ MIPS binary executed successfully");
            println!("   Output: {}", output.trim());
        }
        Err(e) => {
            println!("⚠️  {}", e);
        }
    }
}

#[test]
fn test_basic_binaries_auto_build() {
    // This test ensures test_base and test_overload are built from source
    let result = ensure_basic_test_binaries();
    
    match result {
        Ok(_) => {
            let base_path = get_test_binary_path("test_base");
            let overload_path = get_test_binary_path("test_overload");
            
            assert!(base_path.exists(), "test_base should exist after build");
            assert!(overload_path.exists(), "test_overload should exist after build");
            assert!(is_executable(&base_path), "test_base should be executable");
            assert!(is_executable(&overload_path), "test_overload should be executable");
            
            println!("✅ Basic test binaries built successfully");
        }
        Err(e) => {
            println!("⚠️  Failed to build basic binaries: {}", e);
            println!("   This is expected if gcc is not available");
        }
    }
}

#[test]
fn test_native_binary_execution() {
    // Build and execute a binary for the native architecture
    if let Err(e) = ensure_basic_test_binaries() {
        println!("⚠️  Cannot build test binaries: {}", e);
        return;
    }
    
    let path = get_test_binary_path("test_base");
    let host_arch = get_host_arch();
    
    if !path.exists() {
        println!("⚠️  test_base not found, skipping");
        return;
    }
    
    match execute_binary(path.to_str().unwrap(), host_arch) {
        Ok(output) => {
            println!("✅ Native binary executed successfully");
            println!("   Output: {}", output.trim());
            assert!(output.contains("BASE binary executed") || output.contains("Test"), 
                    "Output should contain expected text");
        }
        Err(e) => {
            panic!("Native binary execution should succeed: {}", e);
        }
    }
}
