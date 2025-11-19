pub mod detector;
pub mod compiler;

pub use detector::{arch::Architecture, os::OperatingSystem, BinaryInfo};
pub use compiler::CompilerConfig;
