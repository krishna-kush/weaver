use std::fs;
use weaver::core::binary::{Architecture, OperatingSystem, BinaryInfo};
use crate::common::{get_test_binary_path, load_test_binary, ensure_win64_binary, ensure_win32_binary, should_skip_cross_host_test};

#[test]
fn test_windows_pe_detection() {
    println!("\n🪟 Testing Windows PE Binary Detection");
    println!("======================================\n");
    
    // Skip if cross-host testing is disabled and we're not on Windows
    if should_skip_cross_host_test("windows", "x86_64") {
        println!("⚠️  Skipping Windows PE test - cross-host testing disabled");
        return;
    }
    
    // Auto-build Windows binary if needed
    if let Err(e) = ensure_win64_binary() {
        println!("{}", e);
        println!("⚠️  Skipping Windows PE test");
        return;
    }
    
    if let Some(binary_data) = load_test_binary("test_win64.exe") {
        println!("📊 Binary properties:");
        println!("   Size: {} bytes", binary_data.len());
        println!("   Magic bytes: {:02X} {:02X} (should be 4D 5A = 'MZ')", 
                 binary_data[0], binary_data[1]);
        
        // Check PE magic bytes
        assert_eq!(binary_data[0], 0x4D, "Invalid PE magic byte 1 (should be 'M')");
        assert_eq!(binary_data[1], 0x5A, "Invalid PE magic byte 2 (should be 'Z')");
        
        // Detect binary info
        let info = BinaryInfo::detect(&binary_data);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.arch, Architecture::X86_64);
        assert!(info.is_supported());
        
        println!("✅ Windows PE binary detected correctly\n");
    } else {
        println!("⚠️  test_win64.exe not found, skipping test");
    }
}

#[test]
fn test_windows_pe_32bit_detection() {
    println!("\n🪟 Testing Windows PE 32-bit Binary Detection");
    println!("=============================================\n");
    
    // Skip if cross-host testing is disabled and we're not on Windows
    if should_skip_cross_host_test("windows", "x86") {
        println!("⚠️  Skipping Windows PE 32-bit test - cross-host testing disabled");
        return;
    }
    
    // Auto-build Windows 32-bit binary if needed
    if let Err(e) = ensure_win32_binary() {
        println!("{}", e);
        println!("⚠️  Skipping Windows PE 32-bit test");
        return;
    }
    
    if let Some(binary_data) = load_test_binary("test_win32.exe") {
        println!("📊 Binary properties:");
        println!("   Size: {} bytes", binary_data.len());
        
        // Check PE magic bytes
        assert_eq!(binary_data[0], 0x4D, "Invalid PE magic byte 1");
        assert_eq!(binary_data[1], 0x5A, "Invalid PE magic byte 2");
        
        // Detect binary info
        let info = BinaryInfo::detect(&binary_data);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert_eq!(info.arch, Architecture::X86);
        
        println!("✅ Windows PE 32-bit binary detected correctly\n");
    } else {
        println!("⚠️  test_win32.exe not found, skipping test");
    }
}

// Removed fake_windows_binary test - we now use real MinGW-compiled binaries

#[test]
fn test_windows_binary_compatibility() {
    let win64_info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Windows,
    };
    
    let win32_info = BinaryInfo {
        arch: Architecture::X86,
        os: OperatingSystem::Windows,
    };
    
    let linux_info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    
    // Same arch and OS should be compatible
    assert!(win64_info.is_compatible_with(&win64_info));
    
    // Different arch should not be compatible
    assert!(!win64_info.is_compatible_with(&win32_info));
    
    // Different OS should not be compatible
    assert!(!win64_info.is_compatible_with(&linux_info));
    
    println!("✅ Windows binary compatibility checks passed\n");
}
