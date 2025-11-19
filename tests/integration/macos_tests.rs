use weaver::core::binary::{Architecture, OperatingSystem, BinaryInfo};
use crate::common::{load_test_binary, should_skip_cross_host_test};

// Note: macOS binary detection test removed - requires real macOS binaries
// To test macOS support, run on actual macOS hardware or use osxcross

#[test]
fn test_macos_binary_format() {
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    assert_eq!(info.os.binary_format(), "Mach-O");
    assert_eq!(info.os.name(), "macOS");
    assert!(info.is_supported());
    
    println!("✅ macOS binary format checks passed\n");
}

#[test]
fn test_macos_arm64_support() {
    let info = BinaryInfo {
        arch: Architecture::AArch64,
        os: OperatingSystem::MacOS,
    };
    
    assert_eq!(info.arch, Architecture::AArch64);
    assert_eq!(info.os, OperatingSystem::MacOS);
    assert!(info.is_supported());
    assert!(info.arch.is_64bit());
    
    println!("✅ macOS ARM64 (Apple Silicon) support verified\n");
}

#[test]
fn test_macos_x86_64_support() {
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    assert_eq!(info.arch, Architecture::X86_64);
    assert_eq!(info.os, OperatingSystem::MacOS);
    assert!(info.is_supported());
    assert!(info.arch.is_64bit());
    
    println!("✅ macOS x86-64 (Intel) support verified\n");
}

#[test]
fn test_macos_binary_compatibility() {
    let macos_x64 = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    let macos_arm = BinaryInfo {
        arch: Architecture::AArch64,
        os: OperatingSystem::MacOS,
    };
    
    let linux_x64 = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    
    // Same arch and OS should be compatible
    assert!(macos_x64.is_compatible_with(&macos_x64));
    
    // Different arch should not be compatible
    assert!(!macos_x64.is_compatible_with(&macos_arm));
    
    // Different OS should not be compatible
    assert!(!macos_x64.is_compatible_with(&linux_x64));
    
    println!("✅ macOS binary compatibility checks passed\n");
}

#[test]
fn test_macos_compiler_config() {
    use weaver::core::binary::CompilerConfig;
    
    // Skip if not on macOS and cross-host testing disabled
    if should_skip_cross_host_test("macos", "x86_64") {
        println!("⚠️  Skipping macOS compiler test - cross-host testing disabled");
        return;
    }
    
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    let config = CompilerConfig::for_binary(&info);
    
    // macOS uses osxcross or native clang
    assert!(
        config.gcc.contains("clang") || config.gcc.contains("darwin"),
        "macOS should use clang or darwin toolchain, got: {}", config.gcc
    );
    
    println!("✅ macOS compiler configuration verified\n");
}

#[test]
fn test_macos_universal_binary_concept() {
    // macOS supports universal binaries (fat binaries with multiple architectures)
    // This test verifies we understand the concept even if not fully implemented
    
    let x64_info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    let arm64_info = BinaryInfo {
        arch: Architecture::AArch64,
        os: OperatingSystem::MacOS,
    };
    
    // Both architectures should be supported on macOS
    assert!(x64_info.is_supported());
    assert!(arm64_info.is_supported());
    
    // But they're not compatible with each other for merging
    assert!(!x64_info.is_compatible_with(&arm64_info));
    
    println!("✅ macOS universal binary concept verified\n");
    println!("   Note: True universal binary support (fat binaries) is a future enhancement");
}
