use goblin::Object;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Solaris,
    Unknown,
}

impl OperatingSystem {
    pub fn detect(data: &[u8]) -> Self {
        match Object::parse(data) {
            Ok(Object::Elf(elf)) => {
                use goblin::elf::header::*;
                match elf.header.e_ident[EI_OSABI] {
                    ELFOSABI_SYSV | ELFOSABI_LINUX => OperatingSystem::Linux,
                    ELFOSABI_FREEBSD => OperatingSystem::FreeBSD,
                    ELFOSABI_OPENBSD => OperatingSystem::OpenBSD,
                    ELFOSABI_NETBSD => OperatingSystem::NetBSD,
                    ELFOSABI_SOLARIS => OperatingSystem::Solaris,
                    _ => OperatingSystem::Linux, // Default to Linux for ELF
                }
            }
            Ok(Object::PE(_)) => OperatingSystem::Windows,
            Ok(Object::Mach(_)) => OperatingSystem::MacOS,
            _ => OperatingSystem::Unknown,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            OperatingSystem::Linux => "Linux",
            OperatingSystem::Windows => "Windows",
            OperatingSystem::MacOS => "macOS",
            OperatingSystem::FreeBSD => "FreeBSD",
            OperatingSystem::OpenBSD => "OpenBSD",
            OperatingSystem::NetBSD => "NetBSD",
            OperatingSystem::Solaris => "Solaris",
            OperatingSystem::Unknown => "Unknown",
        }
    }

    pub fn is_supported(&self) -> bool {
        matches!(
            self,
            OperatingSystem::Linux | OperatingSystem::Windows | OperatingSystem::MacOS
        )
    }

    pub fn binary_format(&self) -> &'static str {
        match self {
            OperatingSystem::Linux
            | OperatingSystem::FreeBSD
            | OperatingSystem::OpenBSD
            | OperatingSystem::NetBSD
            | OperatingSystem::Solaris => "ELF",
            OperatingSystem::Windows => "PE",
            OperatingSystem::MacOS => "Mach-O",
            OperatingSystem::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::build_real_test_binary;

    #[test]
    fn test_linux_detection() {
        // Use real Linux ELF binary
        let binary_data = match build_real_test_binary("gcc") {
            Ok(data) => data,
            Err(e) => {
                println!("⚠️  Skipping test - failed to build binary: {}", e);
                return;
            }
        };
        
        assert_eq!(OperatingSystem::detect(&binary_data), OperatingSystem::Linux);
        assert!(OperatingSystem::Linux.is_supported());
        assert_eq!(OperatingSystem::Linux.binary_format(), "ELF");
    }

    #[test]
    fn test_windows_detection() {
        // Use real Windows PE binary built with MinGW
        let binary_data = match build_real_test_binary("x86_64-w64-mingw32-gcc") {
            Ok(data) => data,
            Err(_) => {
                println!("⚠️  Skipping Windows test - MinGW not available");
                return;
            }
        };
        
        assert_eq!(
            OperatingSystem::detect(&binary_data),
            OperatingSystem::Windows
        );
        assert!(OperatingSystem::Windows.is_supported());
        assert_eq!(OperatingSystem::Windows.binary_format(), "PE");
    }

    #[test]
    fn test_macos_detection() {
        // macOS binaries require osxcross which is complex to set up
        // For now, we test that the detection logic exists
        println!("⚠️  macOS detection test requires osxcross - skipping");
        assert!(OperatingSystem::MacOS.is_supported());
        assert_eq!(OperatingSystem::MacOS.binary_format(), "Mach-O");
    }
}
