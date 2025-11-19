use weaver::core::binary::{Architecture, OperatingSystem, BinaryInfo, CompilerConfig};

#[test]
fn test_compiler_config_x86_64() {
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Linux,
    };
    
    let config = CompilerConfig::for_binary(&info);
    assert_eq!(config.gcc, "x86_64-linux-gnu-gcc");
    assert_eq!(config.objcopy_arch, "i386:x86-64");
    assert_eq!(config.objcopy_output, "elf64-x86-64");
}

#[test]
fn test_compiler_config_arm64() {
    let info = BinaryInfo {
        arch: Architecture::AArch64,
        os: OperatingSystem::Linux,
    };
    
    let config = CompilerConfig::for_binary(&info);
    assert_eq!(config.gcc, "aarch64-linux-gnu-gcc");
    assert_eq!(config.objcopy_arch, "aarch64");
    assert_eq!(config.objcopy_output, "elf64-littleaarch64");
}

#[test]
fn test_compiler_config_arm32() {
    let info = BinaryInfo {
        arch: Architecture::ARM,
        os: OperatingSystem::Linux,
    };
    
    let config = CompilerConfig::for_binary(&info);
    assert_eq!(config.gcc, "arm-linux-gnueabi-gcc");
    assert_eq!(config.objcopy_arch, "arm");
    assert_eq!(config.objcopy_output, "elf32-littlearm");
}

#[test]
fn test_compiler_config_x86() {
    let info = BinaryInfo {
        arch: Architecture::X86,
        os: OperatingSystem::Linux,
    };
    
    let config = CompilerConfig::for_binary(&info);
    assert_eq!(config.gcc, "gcc");
    assert_eq!(config.objcopy_arch, "i386");
    assert_eq!(config.objcopy_output, "elf32-i386");
}

#[test]
fn test_compiler_config_windows() {
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::Windows,
    };
    
    let config = CompilerConfig::for_binary(&info);
    assert_eq!(config.gcc, "x86_64-w64-mingw32-gcc");
    // Windows PE format should have "pe" in objcopy_output, not objcopy_arch
    assert!(config.objcopy_output.contains("pe"), 
            "Expected PE format in objcopy_output, got: {}", config.objcopy_output);
}

#[test]
fn test_compiler_config_macos() {
    let info = BinaryInfo {
        arch: Architecture::X86_64,
        os: OperatingSystem::MacOS,
    };
    
    let config = CompilerConfig::for_binary(&info);
    // macOS uses osxcross or native clang (darwin toolchain)
    assert!(
        config.gcc.contains("clang") || config.gcc.contains("darwin"),
        "Expected macOS compiler (clang/darwin), got: {}", config.gcc
    );
}
