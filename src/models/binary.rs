use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct StoredBinary {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    LinuxELF,
    WindowsPE,
    MacOSMachO,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Platform {
    pub fn detect(data: &[u8]) -> Self {
        if data.len() < 4 {
            return Platform::Unknown;
        }
        
        // ELF magic: 0x7f 'E' 'L' 'F'
        if data.starts_with(b"\x7fELF") {
            return Platform::LinuxELF;
        }
        
        // PE magic: 'M' 'Z'
        if data.starts_with(b"MZ") {
            return Platform::WindowsPE;
        }
        
        // Mach-O magic (various)
        if data.len() >= 4 {
            let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            match magic {
                0xfeedface | 0xfeedfacf | 0xcefaedfe | 0xcffaedfe => {
                    return Platform::MacOSMachO;
                }
                _ => {}
            }
        }
        
        Platform::Unknown
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Platform::LinuxELF => "Linux ELF",
            Platform::WindowsPE => "Windows PE",
            Platform::MacOSMachO => "macOS Mach-O",
            Platform::Unknown => "Unknown",
        }
    }
    
    pub fn is_supported(&self) -> bool {
        matches!(self, Platform::LinuxELF)
    }
}
