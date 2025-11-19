use weaver::core::binary::{Architecture, OperatingSystem, BinaryInfo};
use crate::common::{
    load_test_binary, get_test_binary_path, ensure_x86_64_binary,
    ensure_arm_binary, ensure_arm64_binary, ensure_mips_binary,
    ensure_win64_binary, ensure_win32_binary, should_skip_cross_host_test
};

#[test]
fn test_x86_64_linux_detection() {
    // Auto-build x86_64 binary if needed
    if let Err(e) = ensure_x86_64_binary() {
        println!("⚠️  Cannot build x86-64 binary: {}", e);
        return;
    }
    
    if let Some(data) = load_test_binary("test_x86_64") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::X86_64);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert!(info.is_supported());
        assert!(info.arch.is_64bit());
    } else {
        println!("⚠️  No x86-64 test binary available, skipping test");
    }
}

#[test]
fn test_x86_linux_detection() {
    if let Some(data) = load_test_binary("test_x86") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::X86);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert!(info.is_supported());
        assert!(!info.arch.is_64bit());
    } else {
        println!("⚠️  No x86 (32-bit) test binary available, skipping test");
    }
}

#[test]
fn test_arm64_linux_detection() {
    // Skip if cross-host testing is disabled and we're not on ARM64
    if should_skip_cross_host_test("linux", "aarch64") {
        println!("⚠️  Skipping ARM64 test - cross-host testing disabled (set WEAVER_ENABLE_CROSS_HOST_TESTING=true)");
        return;
    }
    
    // Auto-build ARM64 binary if needed
    if let Err(e) = ensure_arm64_binary() {
        println!("{}", e);
        println!("⚠️  Skipping ARM64 test - cross-compiler not available");
        return;
    }
    
    if let Some(data) = load_test_binary("test_arm64") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::AArch64);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert!(info.is_supported());
        assert!(info.arch.is_64bit());
    } else {
        println!("⚠️  No ARM64 test binary available, skipping test");
    }
}

#[test]
fn test_arm_linux_detection() {
    // Skip if cross-host testing is disabled and we're not on ARM
    if should_skip_cross_host_test("linux", "arm") {
        println!("⚠️  Skipping ARM test - cross-host testing disabled (set WEAVER_ENABLE_CROSS_HOST_TESTING=true)");
        return;
    }
    
    // Auto-build ARM binary if needed
    if let Err(e) = ensure_arm_binary() {
        println!("{}", e);
        println!("⚠️  Skipping ARM test - cross-compiler not available");
        return;
    }
    
    if let Some(data) = load_test_binary("test_arm") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::ARM);
        assert_eq!(info.os, OperatingSystem::Linux);
        assert!(info.is_supported());
        assert!(!info.arch.is_64bit());
    } else {
        println!("⚠️  No ARM (32-bit) test binary available, skipping test");
    }
}

#[test]
fn test_mips_linux_detection() {
    // Skip if cross-host testing is disabled and we're not on MIPS
    if should_skip_cross_host_test("linux", "mips") {
        println!("⚠️  Skipping MIPS test - cross-host testing disabled (set WEAVER_ENABLE_CROSS_HOST_TESTING=true)");
        return;
    }
    
    // Auto-build MIPS binary if needed
    if let Err(e) = ensure_mips_binary() {
        println!("{}", e);
        println!("⚠️  Skipping MIPS test - cross-compiler not available");
        return;
    }
    
    if let Some(data) = load_test_binary("test_mips") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::MIPS);
        assert_eq!(info.os, OperatingSystem::Linux);
        // MIPS is detected but not yet fully supported
        assert!(!info.is_supported());
    } else {
        println!("⚠️  No MIPS test binary available, skipping test");
    }
}

#[test]
fn test_windows_pe_detection() {
    // Skip if cross-host testing is disabled and we're not on Windows
    if should_skip_cross_host_test("windows", "x86_64") {
        println!("⚠️  Skipping Windows PE test - cross-host testing disabled (set WEAVER_ENABLE_CROSS_HOST_TESTING=true)");
        return;
    }
    
    // Auto-build Windows binary if needed
    if let Err(e) = ensure_win64_binary() {
        println!("{}", e);
        println!("⚠️  Skipping Windows PE test - MinGW cross-compiler not available");
        return;
    }
    
    if let Some(data) = load_test_binary("test_win64.exe") {
        let info = BinaryInfo::detect(&data);
        assert_eq!(info.arch, Architecture::X86_64);
        assert_eq!(info.os, OperatingSystem::Windows);
        assert!(info.is_supported());
    } else {
        println!("⚠️  No Windows PE test binary available, skipping test");
    }
}

#[test]
fn test_binary_compatibility() {
    let info1 = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    
    let info2 = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    
    let info3 = BinaryInfo {
        arch: Architecture::ARM,
        os: OperatingSystem::Linux,
    };
    
    let info4 = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Windows,
    };
    
    assert!(info1.is_compatible_with(&info2));
    assert!(!info1.is_compatible_with(&info3)); // Different arch
    assert!(!info1.is_compatible_with(&info4)); // Different OS
}

#[test]
fn test_unsupported_architectures() {
    let mips_info = BinaryInfo {
        arch: Architecture::MIPS,
        os: OperatingSystem::Linux,
    };
    
    let riscv_info = BinaryInfo {
        arch: Architecture::RISCV64,
        os: OperatingSystem::Linux,
    };
    
    assert!(!mips_info.is_supported());
    assert!(!riscv_info.is_supported());
}

#[test]
fn test_architecture_names() {
    assert_eq!(Architecture::X86_64.name(), "x86-64 (64-bit)");
    assert_eq!(Architecture::X86.name(), "x86 (32-bit)");
    assert_eq!(Architecture::AArch64.name(), "ARM64 (AArch64)");
    assert_eq!(Architecture::ARM.name(), "ARM (32-bit)");
    assert_eq!(Architecture::MIPS64.name(), "MIPS64 (64-bit)");
    assert_eq!(Architecture::RISCV64.name(), "RISC-V (64-bit)");
}

#[test]
fn test_os_names() {
    assert_eq!(OperatingSystem::Linux.name(), "Linux");
    assert_eq!(OperatingSystem::Windows.name(), "Windows");
    assert_eq!(OperatingSystem::MacOS.name(), "macOS");
    assert_eq!(OperatingSystem::FreeBSD.name(), "FreeBSD");
}

#[test]
fn test_binary_format() {
    assert_eq!(OperatingSystem::Linux.binary_format(), "ELF");
    assert_eq!(OperatingSystem::Windows.binary_format(), "PE");
    assert_eq!(OperatingSystem::MacOS.binary_format(), "Mach-O");
    assert_eq!(OperatingSystem::FreeBSD.binary_format(), "ELF");
}
