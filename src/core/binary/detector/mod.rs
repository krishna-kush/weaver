pub mod arch;
pub mod os;

use arch::Architecture;
use os::OperatingSystem;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryInfo {
    pub arch: Architecture,
    pub os: OperatingSystem,
}

impl BinaryInfo {
    pub fn detect(data: &[u8]) -> Self {
        Self {
            arch: Architecture::detect(data),
            os: OperatingSystem::detect(data),
        }
    }

    pub fn is_compatible_with(&self, other: &BinaryInfo) -> bool {
        self.arch == other.arch && self.os == other.os
    }

    pub fn is_supported(&self) -> bool {
        self.arch.is_supported() && self.os.is_supported()
    }

    pub fn description(&self) -> String {
        format!("{} on {}", self.arch.name(), self.os.name())
    }
}

impl fmt::Display for BinaryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.os, self.arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::build_real_test_binary;

    #[test]
    fn test_binary_info_detection() {
        // Use real x86-64 binary
        let binary_data = match build_real_test_binary("gcc") {
            Ok(data) => data,
            Err(e) => {
                println!("⚠️  Skipping test - failed to build binary: {}", e);
                return;
            }
        };
        
        let info = BinaryInfo::detect(&binary_data);
        assert_eq!(info.arch, Architecture::X86_64, "Should detect x86-64 architecture");
        assert_eq!(info.os, OperatingSystem::Linux, "Should detect Linux OS");
        assert!(info.is_supported(), "x86-64 Linux should be supported");
    }

    #[test]
    fn test_compatibility_check() {
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
        
        assert!(info1.is_compatible_with(&info2), "Same arch/OS should be compatible");
        assert!(!info1.is_compatible_with(&info3), "Different arch should not be compatible");
    }
}
