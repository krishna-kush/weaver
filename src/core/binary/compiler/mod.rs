use crate::core::binary::detector::{arch::Architecture, os::OperatingSystem, BinaryInfo};

#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub gcc: String,
    pub objcopy: String,
    pub objcopy_arch: String,
    pub objcopy_output: String,
}

impl CompilerConfig {
    pub fn for_binary(info: &BinaryInfo) -> Self {
        // Determine compiler based on OS and architecture
        let gcc = match info.os {
            OperatingSystem::Windows => {
                // Use MinGW for Windows cross-compilation
                match info.arch {
                    Architecture::X86_64 => "x86_64-w64-mingw32-gcc",
                    Architecture::X86 => "i686-w64-mingw32-gcc",
                    _ => "x86_64-w64-mingw32-gcc", // Default to 64-bit
                }
            }
            OperatingSystem::MacOS => {
                // Use osxcross or clang for macOS
                match info.arch {
                    Architecture::X86_64 => "x86_64-apple-darwin-clang",
                    Architecture::AArch64 => "aarch64-apple-darwin-clang",
                    _ => "clang",
                }
            }
            OperatingSystem::Linux => {
                // Use appropriate Linux cross-compiler
                info.arch.gcc_compiler()
            }
            _ => info.arch.gcc_compiler(),
        };
        
        let (objcopy, objcopy_arch, objcopy_output) = match info.os {
            OperatingSystem::Windows => {
                // Windows PE format
                let arch = match info.arch {
                    Architecture::X86_64 => "i386:x86-64",
                    Architecture::X86 => "i386",
                    _ => "i386:x86-64",
                };
                let output = match info.arch {
                    Architecture::X86_64 => "pe-x86-64",
                    Architecture::X86 => "pe-i386",
                    _ => "pe-x86-64",
                };
                ("objcopy", arch, output)
            }
            OperatingSystem::MacOS => {
                // macOS Mach-O format
                let arch = match info.arch {
                    Architecture::X86_64 => "i386:x86-64",
                    Architecture::AArch64 => "aarch64",
                    _ => "i386:x86-64",
                };
                ("objcopy", arch, "mach-o")
            }
            _ => {
                // Linux ELF format (default)
                let objcopy = match info.arch {
                    Architecture::X86_64 => "objcopy",
                    Architecture::X86 => "objcopy",
                    Architecture::AArch64 => "aarch64-linux-gnu-objcopy",
                    Architecture::ARM => "arm-linux-gnueabi-objcopy",
                    Architecture::MIPS | Architecture::MIPS64 => "mips-linux-gnu-objcopy",
                    _ => "objcopy",
                };
                (objcopy, info.arch.objcopy_arch(), info.arch.objcopy_output())
            }
        };
        
        Self {
            gcc: gcc.to_string(),
            objcopy: objcopy.to_string(),
            objcopy_arch: objcopy_arch.to_string(),
            objcopy_output: objcopy_output.to_string(),
        }
    }

    pub fn is_available(&self) -> bool {
        // Check if the compiler is available
        std::process::Command::new(&self.gcc)
            .arg("--version")
            .output()
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
